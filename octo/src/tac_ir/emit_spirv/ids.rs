use std::collections::HashMap;

use spirv::Word as SpirvAddress;
use spirv_headers as spirv;

use rspirv::mr::Builder;

use super::ir::{Address, Operation, ValueType};
use super::ShaderDef;

type PointerType = (bool, ValueType);

#[derive(Default)]
pub struct SpirvIds {
    uv_location: SpirvAddress,
    textures_location: SpirvAddress,
    sampler_location: SpirvAddress,
    push_constants_location: SpirvAddress,
    output_locations: Vec<SpirvAddress>,

    textures_access: Vec<Option<SpirvAddress>>,
    sampler_access: Option<SpirvAddress>,
    uv_access: Option<SpirvAddress>,
    uniform_access: Vec<Option<SpirvAddress>>,
    input_types: Vec<SpirvAddress>,

    //uniform_types: Vec<ValueType>,

    push_constants_type: SpirvAddress,
    push_constants_pointer_type: SpirvAddress,
    texture_type: SpirvAddress,
    texture_pointer_type: SpirvAddress,
    texture_array_type: SpirvAddress,
    sampled_texture_type: SpirvAddress,
    sampler_type: SpirvAddress,
    sampler_pointer_type: SpirvAddress,

    push_constant_types_locations: HashMap<ValueType, SpirvAddress>,
    pointer_types_locations: HashMap<PointerType, SpirvAddress>,
    type_addresses: HashMap<ValueType, SpirvAddress>,
    const_addresses: HashMap<Address, SpirvAddress>,
    const_types: HashMap<Address, ValueType>,
    pub bool2: SpirvAddress,
    pub bool3: SpirvAddress,
    pub bool4: SpirvAddress,
}

impl SpirvIds {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn generate_types(&mut self, module: &mut Builder, info: &ShaderDef, uniforms: &Vec<(ValueType, String)>) {
        let void_type = module.type_void();
        self.type_addresses.insert(ValueType::Void, void_type);

        let bool_id = module.type_bool();
        self.type_addresses.insert(ValueType::Bool, bool_id);

        let float_id = module.type_float(32);
        self.type_addresses.insert(ValueType::Float, float_id);

        let int_id = module.type_int(32, 0);
        self.type_addresses.insert(ValueType::Int, int_id);

        self.bool2 = module.type_vector(bool_id, 2);
        self.bool3 = module.type_vector(bool_id, 3);
        self.bool4 = module.type_vector(bool_id, 4);

        let vec2_id = module.type_vector(float_id, 2);
        let vec3_id = module.type_vector(float_id, 3);
        let vec4_id = module.type_vector(float_id, 4);
        self.type_addresses.insert(ValueType::Vec2, vec2_id);
        self.type_addresses.insert(ValueType::Vec3, vec3_id);
        self.type_addresses.insert(ValueType::Vec4, vec4_id);

        let mat3_id = module.type_matrix(vec3_id, 3);
        let mat4_id = module.type_matrix(vec4_id, 4);
        self.type_addresses.insert(ValueType::Mat3, mat3_id);
        self.type_addresses.insert(ValueType::Mat4, mat4_id);

        let id = module.type_pointer(None, spirv::StorageClass::Input, vec2_id);
        self.pointer_types_locations
            .insert((true, ValueType::Vec2), id);


        for ret in &info.output_type {
            let def = (false, *ret);
            if !self.pointer_types_locations.contains_key(&def) {
                //println!("looking for {}", *ret);
                let contained_id = self.map_type(*ret);
                let id = module.type_pointer(None, spirv::StorageClass::Output, contained_id);
                self.pointer_types_locations.insert(def, id);
            }
        }
        
        // push constants type
        let consts_types: Vec<_> = uniforms.iter().map(|x| self.map_type(x.0)).collect();

        self.push_constants_type = module.type_struct(&consts_types);
        self.push_constants_pointer_type = module.type_pointer(None, spirv::StorageClass::PushConstant, self.push_constants_type);


        for uniform in uniforms {
            if !self.push_constant_types_locations.contains_key(&uniform.0) {
                let contained_id = self.map_type(uniform.0);
                let id = module.type_pointer(None, spirv::StorageClass::PushConstant, contained_id);
                self.push_constant_types_locations.insert(uniform.0, id);
            }

        }

        let num_of_textures = info.input_type.len() as u32;
        println!("num of texture: {}", num_of_textures);
        if num_of_textures > 0 {
            // textures array
            self.texture_type = module.type_image(
                float_id,
                spirv::Dim::Dim2D,
                0u32,
                0u32,
                0u32,
                1u32,
                spirv::ImageFormat::Unknown,
                None,
            );

            let num_id = module.constant_u32(self.map_type(ValueType::Int), num_of_textures);

            let array_type_id = module.type_array(self.texture_type, num_id);
            println!("emitting array type pointer: {}, for type: {}", array_type_id, self.texture_type);

            self.texture_array_type =
                module.type_pointer(None, spirv::StorageClass::UniformConstant, array_type_id);

            self.texture_pointer_type = module.type_pointer(
                None,
                spirv::StorageClass::UniformConstant,
                self.texture_type,
            );

            self.sampler_type = module.type_sampler();
            self.sampler_pointer_type = module.type_pointer(
                None,
                spirv::StorageClass::UniformConstant,
                self.sampler_type,
            );

            self.sampled_texture_type = module.type_sampled_image(self.texture_type);
        }

        self.input_types = info.input_type.iter().map(|x| self.map_type(*x)).collect();

        // for arg in &info.input_type {
        //     let def = (true, *arg);
        //     if !self.pointer_types_locations.contains_key(&def) {
        //         let contained_id = self.map_type(*arg);
        //         let id = module.type_pointer(None, spirv::StorageClass::Input, contained_id);
        //         self.pointer_types_locations.insert(def, id);
        //     }
        // }
    }

    pub fn map_type(&self, typ: ValueType) -> SpirvAddress {
        //println!("mapping {:?}", typ);
        self.type_addresses[&typ]
    }

    pub fn store_constants(&mut self, module: &mut Builder, info: &ShaderDef) {
        for (addr, op) in &info.code {
            match op {
                Operation::StoreInt(x) => {
                    let result_addr = module.constant_u32(self.map_type(ValueType::Int), *x as u32);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Int);
                }
                Operation::StoreFloat(x) => {
                    let result_addr =
                        module.constant_f32(self.map_type(ValueType::Float), *x as f32);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Float);
                }
                Operation::StoreBool(x) => {
                    let result_addr = if *x {
                        module.constant_true(self.map_type(ValueType::Bool))
                    } else {
                        module.constant_false(self.map_type(ValueType::Bool))
                    };
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Bool);
                }
                Operation::StoreVec2(x) => {
                    let typ = self.map_type(ValueType::Float);
                    let comps: Vec<_> = x
                        .iter()
                        .map(|y| module.constant_f32(typ, *y as f32))
                        .collect();
                    let result_addr =
                        module.constant_composite(self.map_type(ValueType::Vec2), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Vec2);
                }
                Operation::StoreVec3(x) => {
                    let typ = self.map_type(ValueType::Float);
                    let comps: Vec<_> = x
                        .iter()
                        .map(|y| module.constant_f32(typ, *y as f32))
                        .collect();
                    let result_addr =
                        module.constant_composite(self.map_type(ValueType::Vec3), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Vec3);
                }
                Operation::Arg(x) => {
                    let typ = self.map_type(ValueType::Int);
                    let res_addr = module.constant_u32(typ, *x as u32);
                    self.const_addresses.insert(*addr, res_addr);
                    //self.const_types.insert(*addr, ValueType::Int);
                }
                Operation::Uniform(x)=>{
                    let typ = self.map_type(ValueType::Int);
                    let res_addr = module.constant_u32(typ, *x as u32);
                    self.const_addresses.insert(*addr, res_addr);
                    //self.const_types.insert(*addr, ValueType::Int);
                }
                _ => (),
            };
        }
    }

    pub fn get_const(&self, address: Address) -> SpirvAddress {
        self.const_addresses[&address]
    }

    pub fn get_const_mapping(&self) -> HashMap<Address, SpirvAddress> {
        self.const_addresses.clone()
    }

    pub fn get_const_types(&self) -> HashMap<Address, ValueType> {
        self.const_types.clone()
    }

    pub fn generate_ids(&mut self, module: &mut Builder, info: &ShaderDef) {
        self.uv_location = module.id();
        self.textures_location = module.id();
        self.sampler_location = module.id();
        self.push_constants_location = module.id();
        println!("uv: {}, tex: {}, sampler: {}, push_constants_location: {}", self.uv_location, self.textures_location, self.sampler_location, self.push_constants_location);
        self.output_locations = Vec::with_capacity(info.output_type.len());
        for _ret in &info.output_type {
            self.output_locations.push(module.id());
        }
    }

    pub fn interface_ids(&self) -> Vec<SpirvAddress> {
        self.output_locations
            .iter()
            .chain([self.uv_location].iter())
            .map(|x| *x)
            .collect()
    }

    pub fn create_uniform_variables(&mut self, module: &mut Builder, info: &ShaderDef, uniforms: &Vec<(ValueType, String)>) {
        println!("texture array type: {}, sampler pointer type: {}", self.texture_array_type, self.sampler_pointer_type);

        if info.input_type.len() > 0{
            module.variable(
                self.texture_array_type,
                Some(self.textures_location),
                spirv::StorageClass::UniformConstant,
                None,
            );
            module.variable(
                self.sampler_pointer_type,
                Some(self.sampler_location),
                spirv::StorageClass::UniformConstant,
                None,
            );
        }
        module.variable(
            self.pointer_types_locations[&(true, ValueType::Vec2)],
            Some(self.uv_location),
            spirv::StorageClass::Input,
            None,
        );

        for (id, loc) in self.output_locations.iter().enumerate() {
            let type_id = self.pointer_types_locations[&(false, info.output_type[id])];
            module.variable(type_id, Some(*loc), spirv::StorageClass::Output, None);
        }

        if uniforms.len() > 0 {
            module.variable(
                self.push_constants_pointer_type,
                Some(self.push_constants_location),
                spirv::StorageClass::PushConstant,
                None,
            );
        }
    }

    pub fn decorate(&self, module: &mut Builder, uniforms: &Vec<(ValueType, String)>) {
        module.decorate(
            self.uv_location,
            spirv::Decoration::Location,
            &[0u32.into()],
        );
        for (id, loc) in self.output_locations.iter().enumerate() {
            module.decorate(*loc, spirv::Decoration::Location, &[(id as u32).into()]);
        }
        if self.input_types.len()>0 {
            module.decorate(
                self.sampler_location,
                spirv::Decoration::DescriptorSet,
                &[0u32.into()],
            );
            module.decorate(
                self.sampler_location,
                spirv::Decoration::Binding,
                &[0u32.into()],
            );

            module.decorate(
                self.textures_location,
                spirv::Decoration::DescriptorSet,
                &[0u32.into()],
            );
            module.decorate(
                self.textures_location,
                spirv::Decoration::Binding,
                &[1u32.into()],
            );
        }



        let mut offset = 0u32;
        for (id, member) in uniforms.iter().enumerate() {
            let member_size = match member.0 {
                ValueType::Float => 16,
                ValueType::Vec2 => 16,
                ValueType::Vec3 => 16,
                ValueType::Vec4 => 16,
                ValueType::Mat3 => 36,
                ValueType::Mat4 => 64,
                ValueType::Int => 16,
                ValueType::Bool => 16,
                _ => panic!(),
            };
            
            module.member_decorate(
                self.push_constants_type,
                id as u32,
                spirv::Decoration::Offset,
                &[offset.into()],
            );
            match member.0 {
                ValueType::Mat3=>{
                    module.member_decorate(
                        self.push_constants_type,
                        id as u32,
                        spirv::Decoration::ColMajor,
                        &[],
                    );
                    module.member_decorate(
                        self.push_constants_type,
                        id as u32,
                        spirv::Decoration::MatrixStride,
                        &[9u32.into()],
                    );
                }
                ValueType::Mat4=>{
                    module.member_decorate(
                        self.push_constants_type,
                        id as u32,
                        spirv::Decoration::ColMajor,
                        &[],
                    );
                    module.member_decorate(
                        self.push_constants_type,
                        id as u32,
                        spirv::Decoration::MatrixStride,
                        &[16u32.into()],
                    );
                }
                _=>{},
            }
            offset += member_size;
        }
        module.decorate(
            self.push_constants_type,
            spirv::Decoration::Block,
            &[],
        );
    }

    pub fn access_sampler(&mut self, module: &mut Builder) -> SpirvAddress {
        let samp = self.sampler_access;
        match samp {
            None => {
                let samp = module
                    .load(self.sampler_type, None, self.sampler_location, None, &[])
                    .unwrap();
                samp
            }
            Some(x) => x,
        }
    }

    pub fn access_uv(&mut self, module: &mut Builder) -> SpirvAddress {
        let samp = self.uv_access;
        match samp {
            None => {
                let samp = module
                    .load(
                        self.map_type(ValueType::Vec2),
                        None,
                        self.uv_location,
                        None,
                        &[],
                    )
                    .unwrap();
                samp
            }
            Some(x) => x,
        }
    }

    pub fn access_arg(&mut self, id: usize, addr: Address, module: &mut Builder) -> SpirvAddress {
        while self.textures_access.len() <= id {
            self.textures_access.push(None);
        }

        let access = self.textures_access[id];
        match access {
            None => {
                let id_addr = self.get_const(addr);
                let access_chain = module
                    .access_chain(
                        self.texture_pointer_type,
                        None,
                        self.textures_location,
                        &[(id_addr as u32).into()],
                    )
                    .unwrap();
                let load = module
                    .load(self.texture_type, None, access_chain, None, &[])
                    .unwrap();
                self.textures_access[id] = Some(load);

                load
            }
            Some(x) => x,
        }
    }

    pub fn store_result(&mut self, id: usize, addr: SpirvAddress, module: &mut Builder) {
        let uniform_addr = self.output_locations[id];
        module.store(uniform_addr, addr, None, &[]).unwrap();
    }

    pub fn sample_arg(&mut self, id: usize, addr: Address, typ: ValueType, module: &mut Builder) -> SpirvAddress {
        let uv = self.access_uv(module);
        self.sample_arg_at(id, addr, uv, typ, module)
    }

    pub fn sample_arg_at(&mut self, id: usize, addr: Address, uv: SpirvAddress, typ: ValueType, module: &mut Builder) -> SpirvAddress {
        let address = self.access_arg(id, addr, module);
        let sampler = self.access_sampler(module);
        let type_addr = self.input_types[id];
        let vec4_type = self.map_type(ValueType::Vec4);

        let sampled = module
            .sampled_image(self.sampled_texture_type, None, address, sampler)
            .unwrap();

        let mut result = module
            .image_sample_implicit_lod(vec4_type, None, sampled, uv, None, &[])
            .unwrap();
        if type_addr != vec4_type {
            let float_type = self.map_type(ValueType::Float);
            result = match typ{
                ValueType::Float => {
                    let x = module.composite_extract(float_type, None, result, &[0]).unwrap();
                    x
                },
                ValueType::Vec2 => {
                    let x = module.composite_extract(float_type, None, result, &[0]).unwrap();
                    let y = module.composite_extract(float_type, None, result, &[1]).unwrap();
                    let vec = module.composite_construct(self.map_type(ValueType::Vec2), None, &[x, y]).unwrap();
                    vec
                },
                ValueType::Vec3 => {
                    let x = module.composite_extract(float_type, None, result, &[0]).unwrap();
                    let y = module.composite_extract(float_type, None, result, &[1]).unwrap();
                    let z = module.composite_extract(float_type, None, result, &[2]).unwrap();
                    let vec = module.composite_construct(self.map_type(ValueType::Vec3), None, &[x, y, z]).unwrap();
                    vec
                },
                _ => panic!("can't store non-float in texture!"),
            };
        }
        result
    }

    pub fn access_uniform(&mut self, id: usize, addr: Address, module: &mut Builder, uniform_type: ValueType) -> SpirvAddress {
        while self.uniform_access.len() <= id {
            self.uniform_access.push(None);
        }

        let access = self.uniform_access[id];
        match access {
            Some(x) => x,
            None =>{
                let ptr_type = self.push_constant_types_locations[&uniform_type];
                let typ = self.type_addresses[&uniform_type];
                let id_addr = self.get_const(addr);
                let access_chain = module.access_chain(
                    ptr_type,
                    None,
                    self.push_constants_location,
                    &[(id_addr as u32).into()],
                ).unwrap();
                let load = module.load(typ, None, access_chain, None, &[]).unwrap();
                self.uniform_access[id] = Some(load);
                load
            }
        }

    }
}
