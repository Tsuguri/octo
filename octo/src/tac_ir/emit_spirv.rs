
use super::ir::{
    Address,
    Operation,
    Op,
    PipelineIR,
    ValueType,
};
use super::{PipelineDef, ShaderDef};
use rspirv::mr::*;
use rspirv::binary::{Assemble, Disassemble};
use spirv_headers as spirv;
use octo_runtime::*;
use log::{error};
use std::collections::HashMap;

use shaderc::ShaderKind as Shader;

static VERTEX: &str = include_str!("../basic_vertex.glsl");

// TODO: move this to library compilation stage (build.rs)
fn create_basic_vertex()->Vec<u32> {
    let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
    let compilation_result = compiler
        .compile_into_spirv(
            &VERTEX,
            Shader::Vertex,
            "basic_vertex",
            "main",
            None,
        )
        .map_err(|e| {
            error!("{}", e);
            "Couldn't compile fragment shader!"
        }).unwrap();
    compilation_result.as_binary().to_vec()
}

fn version() -> u32{

    let version = env!("CARGO_PKG_VERSION");
    let numbers: Vec<_> = version.split('.').collect();
    assert!(numbers.len()==3);
    let numbers: Vec<u32> = numbers.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let final_version = numbers[0]*10000 + numbers[1]*100 + numbers[2];
    //println!("{}", final_version);
    final_version
}

pub fn emit_spirv(module_name: &str, code: PipelineDef) ->  OctoModule{
    let mut code = code;
    println!("Emitting spirv module");

    let mut module = OctoModule::new();
    module.name = module_name.to_owned();
    module.version = version();
    module.basic_vertex_spirv = create_basic_vertex();

    module.required_input = code.args.drain(0..code.args.len()).map(|(x, y)| {
        let x = match x {
            ValueType::Vec2=> TextureType::Vec2,
            ValueType::Vec3=> TextureType::Vec3,
            ValueType::Vec4=> TextureType::Vec4,
            _ => TextureType::Float,
        };

        (y,x)
    }).collect();

    for (id, shader_ir) in code.shaders.drain(0..code.shaders.len()).enumerate() {
        let shader_spirv = emit_single_shader(shader_ir);
        module.fragment_shaders.insert(id, shader_spirv);
    }

    for (id, def) in code.passes.drain(0..code.passes.len()).enumerate() {
        let octo_pass = ShaderPass{
            id,
            shader: def.shader_id,
            input: def.input.iter().map(|x| (*x).into()).collect(),
            output: def.output.into(),
            dependencies: def.dependencies,
        };
        module.passes.push(octo_pass);
    }

    for (id, tex) in code.textures.drain(0..code.textures.len()).enumerate() {
        let typ = match tex {
            ValueType::Vec2=> TextureType::Vec2,
            ValueType::Vec3=> TextureType::Vec3,
            ValueType::Vec4=> TextureType::Vec4,
            _ => TextureType::Float,
        };
        // probably to be changed once Scale operation is implemented
        let size = TextureSize::Original;
        module.textures.push((id, typ, size));
    }
    

    module
}

use spirv_headers::Word as SpirvAddress;

type PointerType = (bool, ValueType);

#[derive(Default)]
struct SpirvIds {
    uv_location: SpirvAddress,
    textures_location: SpirvAddress,
    sampler_location: SpirvAddress,
    output_locations: Vec<SpirvAddress>,

    textures_access: Vec<Option<SpirvAddress>>,
    sampler_access: Option<SpirvAddress>,
    uv_access: Option<SpirvAddress>,
    input_types: Vec<SpirvAddress>,

    texture_type: SpirvAddress,
    texture_pointer_type: SpirvAddress,
    texture_array_type: SpirvAddress,
    sampled_texture_type: SpirvAddress,
    sampler_type: SpirvAddress,
    sampler_pointer_type: SpirvAddress,

    pointer_types_locations: HashMap<PointerType, SpirvAddress>,
    type_addresses: HashMap<ValueType, SpirvAddress>,
    const_addresses: HashMap<Address, SpirvAddress>,
    pub bool2: SpirvAddress,
    pub bool3: SpirvAddress,
    pub bool4: SpirvAddress,
}

impl SpirvIds {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn generate_types(&mut self, module: &mut Builder, info: &ShaderDef) {
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

        let id = module.type_pointer(None, spirv::StorageClass::Input, vec2_id);
        self.pointer_types_locations.insert((true, ValueType::Vec2), id);

        for ret in &info.output_type {
            let def = (false, *ret);
            if !self.pointer_types_locations.contains_key(&def) {
                //println!("looking for {}", *ret);
                let contained_id = self.map_type(*ret);
                let id = module.type_pointer(None, spirv::StorageClass::Output, contained_id);
                self.pointer_types_locations.insert(def, id);
            }
        }

        let num_of_textures = info.input_type.len() as u32;
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

            self.texture_array_type = module.type_pointer(None, spirv::StorageClass::UniformConstant, array_type_id);

            self.texture_pointer_type = module.type_pointer(None, spirv::StorageClass::UniformConstant, self.texture_type);

            self.sampler_type = module.type_sampler();
            self.sampler_pointer_type = module.type_pointer(None, spirv::StorageClass::UniformConstant, self.sampler_type);

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
        // println!("mapping {:?}", typ);
        self.type_addresses[&typ]
    }

    pub fn store_constants(&mut self, module: &mut Builder, info: &ShaderDef) {
        for (addr, op) in &info.code {
            match op {
                Operation::StoreInt(x) =>{
                    let result_addr = module.constant_u32(self.map_type(ValueType::Int), *x as u32);
                    self.const_addresses.insert(*addr, result_addr);
                },
                Operation::StoreFloat(x)=>{
                    let result_addr = module.constant_f32(self.map_type(ValueType::Float), *x as f32);
                    self.const_addresses.insert(*addr, result_addr);
                },
                Operation::StoreBool(x)=>{
                    let result_addr = if *x {
                        module.constant_true(self.map_type(ValueType::Bool))

                    } else {
                        module.constant_false(self.map_type(ValueType::Bool))
                    };
                    self.const_addresses.insert(*addr, result_addr);
                },
                Operation::StoreVec2(x)=>{
                    let typ = self.map_type(ValueType::Float);
                    let comps : Vec<_> = x.iter().map(|y|{
                        module.constant_f32(typ, *y as f32)
                    }).collect();
                    let result_addr = module.constant_composite(self.map_type(ValueType::Vec2), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                },
                Operation::StoreVec3(x)=>{
                    let typ = self.map_type(ValueType::Float);
                    let comps : Vec<_> = x.iter().map(|y|{
                        module.constant_f32(typ, *y as f32)
                    }).collect();
                    let result_addr = module.constant_composite(self.map_type(ValueType::Vec3), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                },
                Operation::Arg(x) => {
                    let typ = self.map_type(ValueType::Int);
                    let res_addr = module.constant_u32(typ, *x as u32);
                    self.const_addresses.insert(*addr, res_addr);
                }
                _=> (),
            };

        }



    }

    pub fn get_const(&self, address: Address) -> SpirvAddress {
        self.const_addresses[&address]

    }

    pub fn generate_ids(&mut self, module: &mut Builder, info: &ShaderDef) {
        self.uv_location = module.id();
        self.textures_location = module.id();
        self.sampler_location = module.id();
        self.output_locations = Vec::with_capacity(info.output_type.len());
        for _ret in &info.output_type {
            self.output_locations.push(module.id());
        }
    }

    pub fn interface_ids(&self) -> Vec<SpirvAddress> {
        self.output_locations.iter().chain([self.uv_location].iter()).map(|x| *x).collect()
    }

    pub fn create_uniform_variables(&mut self, module: &mut Builder, info: &ShaderDef) {
        module.variable(self.texture_array_type, Some(self.textures_location), spirv::StorageClass::UniformConstant, None);
        module.variable(self.sampler_pointer_type, Some(self.sampler_location), spirv::StorageClass::UniformConstant, None);
        module.variable(self.pointer_types_locations[&(true, ValueType::Vec2)], Some(self.uv_location), spirv::StorageClass::Input, None);

        
        for (id, loc) in self.output_locations.iter().enumerate() {
            let type_id = self.pointer_types_locations[&(false, info.output_type[id])];
            module.variable(type_id, Some(*loc), spirv::StorageClass::Output, None);
        }
    }

    pub fn decorate(&self, module: &mut Builder) {
        module.decorate(self.uv_location, spirv::Decoration::Location, &[0u32.into()]);
        for (id, loc) in self.output_locations.iter().enumerate() {
            module.decorate(*loc, spirv::Decoration::Location, &[(id as u32).into()]);
        }

        module.decorate(self.sampler_location, spirv::Decoration::DescriptorSet, &[0u32.into()]);
        module.decorate(self.sampler_location, spirv::Decoration::Binding, &[0u32.into()]);

        module.decorate(self.textures_location, spirv::Decoration::DescriptorSet, &[0u32.into()]);
        module.decorate(self.textures_location, spirv::Decoration::Binding, &[1u32.into()]);

    }

    pub fn access_sampler(&mut self, module: &mut Builder) -> SpirvAddress {
        let samp = self.sampler_access;
        match samp {
            None => {
                let samp = module.load(self.sampler_type, None, self.sampler_location, None, &[]).unwrap();
                samp
            },
            Some(x)=> x
        }
    }

    pub fn access_uv(&mut self, module: &mut Builder) -> SpirvAddress {
        let samp = self.uv_access;
        match samp {
            None => {
                let samp = module.load(self.map_type(ValueType::Vec2), None, self.uv_location, None, &[]).unwrap();
                samp
            },
            Some(x)=> x
        }
    }

    pub fn access_arg(&mut self, id: usize, addr: Address, module: &mut Builder) -> SpirvAddress{
        while self.textures_access.len() <= id {
            self.textures_access.push(None); 
        }

        let access = self.textures_access[id];
        match access {
            None => {
                let id_addr = self.get_const(addr);
                let access_chain = module.access_chain(self.texture_pointer_type, None, self.textures_location, &[(id_addr as u32).into()]).unwrap();
                let load = module.load(self.texture_type, None, access_chain, None, &[]).unwrap();
                self.textures_access[id] = Some(load);

                load
            },
            Some(x) => {
                x
            }
        }
    }

    pub fn store_result(&mut self, id: usize, addr: SpirvAddress, module: &mut Builder){
        let uniform_addr = self.output_locations[id];
        module.store(uniform_addr, addr, None, &[]).unwrap();
    }

    pub fn sample_arg(&mut self, id: usize, addr: Address ,module: &mut Builder) -> SpirvAddress {
        let address = self.access_arg(id, addr, module);
        let sampler = self.access_sampler(module);
        let uv = self.access_uv(module);
        let typ = self.input_types[id];

        let sampled = module.sampled_image(self.sampled_texture_type, None, address, sampler).unwrap();

        let result = module.image_sample_implicit_lod(
            typ,
            None,
            sampled,
            uv,
            None,
            &[],
        ).unwrap();
        result
    }
}

struct MainEmitter<'a, I: std::iter::Iterator<Item=&'a Op>> {
    builder: &'a mut Builder,
    ids: &'a mut SpirvIds,
    value_map: HashMap<Address, SpirvAddress>,
    type_map: HashMap<Address, ValueType>,

    input_type: Vec<ValueType>,
    iter: I,
}

impl<'a, I: std::iter::Iterator<Item=&'a Op>> MainEmitter<'a, I> {
    pub fn new(ids: &'a mut SpirvIds, module: &'a mut Builder, input_type: Vec<ValueType>, iter: I) -> MainEmitter<'a, I>{
        module.begin_basic_block(None).unwrap();

        Self {
            builder: module,
            ids: ids,
            value_map: Default::default(),
            type_map: Default::default(),
            input_type,
            iter,
        }
    }

    fn emit_arg(&mut self, val_type: ValueType, id: usize, ret: Address) {
        let access = self.ids.sample_arg(id, ret, self.builder);
        self.value_map.insert(ret, access);
        self.type_map.insert(ret, val_type);
        // println!("loading arg {}", x);
    }

    fn emit_store(&mut self, addr: Address, ret: Address) {
        let spirv_addr = self.value_map[&addr];
        self.value_map.insert(ret,spirv_addr);
        self.type_map.insert(ret, self.type_map[&addr]);
    }

    fn emit_add(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Int);
                let ret_addr = self.builder.iadd(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Int);
            },
            typ =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(typ);
                let ret_addr = self.builder.fadd(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, typ);
            },
        }
    }

    fn emit_sub(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Int);
                let ret_addr = self.builder.isub(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Int);
            },
            typ =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(typ);
                let ret_addr = self.builder.fsub(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, typ);
            },
        }
    }

    fn emit_mul(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Int);
                let ret_addr = self.builder.imul(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Int);
            },
            typ =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(typ);
                let ret_addr = self.builder.fmul(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, typ);
            },
        }

    }

    fn emit_div(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Int);
                let ret_addr = self.builder.sdiv(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Int);
            },
            typ =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(typ);
                let ret_addr = self.builder.fdiv(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, typ);
            },
        }


    }

    pub fn emit_less(&mut self, left: Address, right: Address, ret: Address){

        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Bool);
                let ret_addr = self.builder.sless_than(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            },
            ValueType::Float =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(ValueType::Bool);
                let ret_addr = self.builder.ford_less_than(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            },
            _=>{
                // we can't compare vectors yet.
                assert!(false);

            }
        }
    }

    pub fn emit_less_eq(&mut self, left: Address, right: Address, ret: Address){
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        match left_type {
            ValueType::Bool =>{
                assert!(false);
            },
            ValueType::Int =>{
                let ret_type = self.ids.map_type(ValueType::Bool);
                let ret_addr = self.builder.sless_than_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            },
            ValueType::Float =>{
                // all other types are floats for spir-v
                let ret_type = self.ids.map_type(ValueType::Bool);
                let ret_addr = self.builder.ford_less_than_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            },
            _=>{
                // we can't compare vectors yet.
                assert!(false);

            }
        }

    }
    pub fn emit_eq(&mut self, left: Address, right: Address, ret: Address){

        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        let ret_type = self.ids.map_type(ValueType::Bool);

        match left_type {
            ValueType::Bool =>{
                let ret_addr = self.builder.logical_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            }
            ValueType::Int => {
                let ret_addr = self.builder.iequal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Float => {
                let ret_addr = self.builder.ford_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            }
            ValueType::Vec2 => {
                let temp_ret_type = self.ids.bool2;
                let ret_addr = self.builder.ford_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.all(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Vec3 => {
                let temp_ret_type = self.ids.bool3;
                let ret_addr = self.builder.ford_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.all(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Vec4 => {
                let temp_ret_type = self.ids.bool4;
                let ret_addr = self.builder.ford_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.all(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            _=>{}
        }

    }
    pub fn emit_neq(&mut self, left: Address, right: Address, ret: Address){

        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==right_type);
        let ret_type = self.ids.map_type(ValueType::Bool);

        match left_type {
            ValueType::Bool =>{
                let ret_addr = self.builder.logical_not_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            }
            ValueType::Int => {
                let ret_addr = self.builder.inot_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Float => {
                let ret_addr = self.builder.ford_not_equal(ret_type, None, left_address, right_address).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);
            }
            ValueType::Vec2 => {
                let temp_ret_type = self.ids.bool2;
                let ret_addr = self.builder.ford_not_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.any(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Vec3 => {
                let temp_ret_type = self.ids.bool3;
                let ret_addr = self.builder.ford_not_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.any(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            ValueType::Vec4 => {
                let temp_ret_type = self.ids.bool4;
                let ret_addr = self.builder.ford_not_equal(temp_ret_type, None, left_address, right_address).unwrap();
                let ret_addr = self.builder.any(ret_type, None, ret_addr).unwrap();
                self.value_map.insert(ret, ret_addr);
                self.type_map.insert(ret, ValueType::Bool);

            }
            _=>{}
        }
    }

    fn emit_and(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==ValueType::Bool);
        assert!(right_type==ValueType::Bool);
        let ret_type = self.ids.map_type(ValueType::Bool);

        let ret_addr = self.builder.logical_and(ret_type, None, left_address, right_address).unwrap();
        self.value_map.insert(ret, ret_addr);
        self.type_map.insert(ret, ValueType::Bool);

    }

    fn emit_or(&mut self, left: Address, right: Address, ret: Address) {
        let left_address = self.value_map[&left];
        let right_address = self.value_map[&right];
        let left_type = self.type_map[&left];
        let right_type = self.type_map[&right];
        // should be checked by static analysis
        assert!(left_type==ValueType::Bool);
        assert!(right_type==ValueType::Bool);
        let ret_type = self.ids.map_type(ValueType::Bool);

        let ret_addr = self.builder.logical_or(ret_type, None, left_address, right_address).unwrap();
        self.value_map.insert(ret, ret_addr);
        self.type_map.insert(ret, ValueType::Bool);

    }

    pub fn emit(mut self){

        while let Some((ret, op_code)) = self.iter.next() {
            self.emit_operation(*ret, *op_code);

        }
    }

    fn emit_operation(&mut self, ret: Address, operation: Operation) {
        match operation {
            Operation::Arg(x)=>{
                self.emit_arg(self.input_type[x], x, ret);
            },
            Operation::Store(addr) =>{
                self.emit_store(addr, ret);
            }
            Operation::Add(left, right) =>{
                self.emit_add(left, right, ret);
            }
            Operation::Sub(left, right) =>{
                self.emit_sub(left, right, ret);
            }
            Operation::Mul(left, right) =>{
                self.emit_mul(left, right, ret);
            }
            Operation::Div(left, right) =>{
                self.emit_div(left, right, ret);
            }
            Operation::Less(left, right) => {
                self.emit_less(left, right, ret);
            }
            Operation::LessEq(left, right) => {
                self.emit_less_eq(left, right, ret);
            }
            Operation::Eq(left, right) =>{
                self.emit_eq(left, right, ret);
            }
            Operation::Neq(left, right) =>{
                self.emit_neq(left, right, ret);
            }
            Operation::And(left, right) => {
                self.emit_and(left, right, ret);
            }
            Operation::Or(left, right) => {
                self.emit_or(left, right, ret);
            }
            Operation::Exit(val, _label)=>{
                let value_addr = self.value_map[&val];
                self.ids.store_result(0, value_addr, self.builder);
            }


            _ => (),
        }
    }
}

impl<'a, I : Iterator<Item=&'a Op>> std::ops::Drop for MainEmitter<'a, I> {
    fn drop(&mut self) {
        self.builder.ret().unwrap();
    }
}

fn emit_single_shader(info: ShaderDef)->Vec<u32> {
    println!("Emitting single fragment shader\n\n");

    let mut module = Builder::new();
    module.capability(spirv::Capability::Shader);
    let _glsl = module.ext_inst_import("GLSL.std.450");
    module.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

    let mut ids = SpirvIds::new();

    let function_id = module.id();
    ids.generate_ids(&mut module, &info);

    let interface = ids.interface_ids();

    module.entry_point(spirv::ExecutionModel::Fragment, function_id, "main", &interface);
    module.execution_mode(function_id, spirv::ExecutionMode::OriginUpperLeft, &[]);


    ids.decorate(&mut module);
    ids.generate_types(&mut module, &info);
    ids.create_uniform_variables(&mut module, &info);
    ids.store_constants(&mut module, &info);

    let main_type = module.type_function(ids.map_type(ValueType::Void), vec![]);
    module.begin_function(ids.map_type(ValueType::Void),
                     Some(function_id),
                     spirv::FunctionControl::DONT_INLINE |
                      spirv::FunctionControl::CONST,
                     main_type)
     .unwrap();

    // emitting main function
    let emitter = MainEmitter::new(&mut ids, &mut module, info.input_type.clone(), info.code.iter());

    emitter.emit();

    module.end_function().unwrap();

    let m =module.module();

    println!("{}", m.disassemble());
    m.assemble()
}