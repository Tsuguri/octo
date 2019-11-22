
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
    const_types: HashMap<Address, ValueType>,
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
                    self.const_types.insert(*addr, ValueType::Int);
                },
                Operation::StoreFloat(x)=>{
                    let result_addr = module.constant_f32(self.map_type(ValueType::Float), *x as f32);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Float);
                },
                Operation::StoreBool(x)=>{
                    let result_addr = if *x {
                        module.constant_true(self.map_type(ValueType::Bool))

                    } else {
                        module.constant_false(self.map_type(ValueType::Bool))
                    };
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Bool);
                },
                Operation::StoreVec2(x)=>{
                    let typ = self.map_type(ValueType::Float);
                    let comps : Vec<_> = x.iter().map(|y|{
                        module.constant_f32(typ, *y as f32)
                    }).collect();
                    let result_addr = module.constant_composite(self.map_type(ValueType::Vec2), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Vec2);
                },
                Operation::StoreVec3(x)=>{
                    let typ = self.map_type(ValueType::Float);
                    let comps : Vec<_> = x.iter().map(|y|{
                        module.constant_f32(typ, *y as f32)
                    }).collect();
                    let result_addr = module.constant_composite(self.map_type(ValueType::Vec3), &comps);
                    self.const_addresses.insert(*addr, result_addr);
                    self.const_types.insert(*addr, ValueType::Vec3);
                },
                Operation::Arg(x) => {
                    let typ = self.map_type(ValueType::Int);
                    let res_addr = module.constant_u32(typ, *x as u32);
                    self.const_addresses.insert(*addr, res_addr);
                    self.const_types.insert(*addr, ValueType::Vec4);
                }
                _=> (),
            };

        }



    }

    pub fn get_const(&self, address: Address) -> SpirvAddress {
        self.const_addresses[&address]

    }

    pub fn get_const_mapping(&self) -> HashMap<Address, SpirvAddress>{
        self.const_addresses.clone()
    }

    pub fn get_const_types(&self) -> HashMap<Address, ValueType> {
        self.const_types.clone()

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

struct IfElseCode {
    condition_label: Address,
    true_block: Vec<Op>,
    false_block: Option<Vec<Op>>,
    phi_nodes: Vec<Op>,
    if_label: Address,
    else_label: Option<Address>,
    end_label:Address,
}

struct PeekableCode<'a, I: std::iter::Iterator<Item=&'a Op>> {
    iter: I,
    peek_val: Option<&'a Op>,
}

impl<'a, I: std::iter::Iterator<Item=&'a Op>> PeekableCode<'a, I> {
    pub fn new(iter: I) -> Self {
        let mut iter = iter;
        let peek = iter.next();
        Self{
            iter,
            peek_val: peek,
        }
    }

    pub fn peek(&mut self)-> Option<&Op> {
        self.peek_val
    }

    pub fn next(&mut self) ->Option<&Op> {
        let val = self.peek_val;
        self.peek_val = self.iter.next();
        val
    }
}

struct LoopCode {
    content: Vec<Op>,
    cond: Vec<Op>, //includes phi nodes

}

struct MainEmitter<'a, I: std::iter::Iterator<Item=&'a Op>> {
    builder: &'a mut Builder,
    ids: &'a mut SpirvIds,
    value_map: HashMap<Address, SpirvAddress>,
    type_map: HashMap<Address, ValueType>,
    current_block: SpirvAddress,

    input_type: Vec<ValueType>,
    iter: Option<PeekableCode<'a, I>>,
}

impl<'a, I: std::iter::Iterator<Item=&'a Op>> MainEmitter<'a, I> {
    pub fn new(ids: &'a mut SpirvIds, module: &'a mut Builder, input_type: Vec<ValueType>, iter: I) -> MainEmitter<'a, I>{

        let mut map = ids.get_const_mapping();
        let types = ids.get_const_types();
        let current_block = module.begin_basic_block(None).unwrap();
        map.insert(1, current_block);
        Self {
            builder: module,
            ids: ids,
            value_map: map,
            type_map: types,
            current_block,
            input_type,
            iter: Some(PeekableCode::new(iter)),
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

    fn find_if_else<'b, I2: std::iter::Iterator<Item=&'b Op>>(ret: Address, op: Operation, code: &mut PeekableCode<'b, I2>) -> IfElseCode{
        let (cond, if_label, else_label) = match op {
            Operation::JumpIfElse(x,y,z)=>(x,y,z),
            _=>{unreachable!();}
        };

        let mut true_code = vec![];
        loop {
            let op = match code.next(){
                None => panic!("internal compiler error"),
                Some(x) => x,
            };
            match op.1 {
                Operation::Label if op.0 == else_label =>{
                    // found end of block
                    break;
                }
                _=>{
                    // entry inside a block
                    true_code.push(*op);
                }
            }
        };
        assert!(true_code.len() >0);
        let end_label = match true_code[true_code.len()-1].1 {
            Operation::Jump(lab) => lab,
            x=> {
                println!("error at {:#?}", x);
                panic!();
            }
        };
        true_code.pop();
        println!("labels: start: {}, else: {}, end: {}", if_label, else_label, end_label);

        let false_code = if end_label != else_label {
            let mut false_code : Vec<(Address, Operation)> = vec![];
            false_code.push((else_label, Operation::Label));
            loop {
                let op = match code.next(){
                    None => panic!("internal compiler error"),
                    Some(x) => x,
                };
                match op.1 {
                    Operation::Label if op.0 == end_label =>{
                        break;
                    }
                    _=>{
                        false_code.push(*op);
                    }
                }
            };
            false_code.pop();
            Some(false_code)
        } else {
            None
        };

        let mut phi_nodes = vec![];
        loop {
            let op = code.peek();
            if op.is_none() {
                break;
            }
            let op = op.unwrap();
            match op.1 {
                Operation::Phi(..) => {
                    phi_nodes.push(*code.next().unwrap());
                },
                _=>{
                    break;
                }
            }
        }
        
        // println!("end label is {}", end_label);

        // println!("true branch code: {:#?}", true_code);
        // println!("false branch code: {:#?}", false_code);
        // println!("Phi nodes: {:#?}", phi_nodes);

        IfElseCode{
            condition_label: cond,
            true_block: true_code,
            false_block: false_code,
            phi_nodes,
            if_label,
            else_label: if else_label == end_label {None} else {Some(else_label)},
            end_label,
        }
    }

    fn find_loop<'b, I2: std::iter::Iterator<Item=&'b Op>>(ret: Address, op: Operation, code: &mut PeekableCode<'b, I2>) -> LoopCode{
        let label = match op {
            Operation::Jump(x)=>(x),
            _=>{unreachable!();}
        };

        let content_label = match code.peek() {
            None => unreachable!(),
            Some(x) => match x.1 {
                Operation::Label => x.0,
                _ => unreachable!(),
            }
        };

        let mut content = vec![];
        loop {
            let op = match code.next(){
                None => panic!("internal compiler error"),
                Some(x) => x,
            };
            match op.1 {
                Operation::Label if op.0 == label =>{
                    // found end of block
                    break;
                }
                _=>{
                    // entry inside a block
                    content.push(*op);
                }
            }
        };

        assert!(content.len() >0);
        let cond_label = match content[content.len()-1].1 {
            Operation::Jump(lab) => lab,
            x=> {
                println!("error at {:#?}", x);
                panic!();
            }
        };
        assert!(label==cond_label);
        println!("labels match");
        let mut condition_code = vec![];

        let mut end_label: Address = Default::default();
        loop {
            let op = match code.next(){
                None => panic!("internal compiler error"),
                Some(x) => x,
            };
            match op.1 {
                Operation::JumpIfElse(x,y,z) if y== content_label =>{
                    end_label = z;
                    break;
                }
                Operation::JumpIfElse(x,y,z) if z== content_label =>{
                    end_label = y;
                    break;
                }
                _=>{
                    // entry inside a block
                    condition_code.push(*op);
                }
            }
        };
        println!("labels: content {}, cond {}, end {}", content_label, cond_label, end_label);
        let next = code.next().unwrap();
        let next_label = match next.1{
            Operation::Label => next.0,
            _ =>{panic!("internal compiler error");}
        };
        assert!(next_label == end_label);

        LoopCode{
            cond: condition_code,
            content,
        }
    }

    fn emit_block(&mut self, id: Option<SpirvAddress>)->SpirvAddress{
        self.current_block = self.builder.begin_basic_block(id).unwrap();
        self.current_block
    }

    fn emit_if_else(&mut self, data: IfElseCode) {
        println!("starting if else");

        let true_label = self.builder.id();
        self.value_map.insert(data.if_label, true_label);
        let end_label = self.builder.id();
        self.value_map.insert(data.end_label, end_label);
        let false_label = if data.else_label.is_some() {
            let id = self.builder.id();
            self.value_map.insert(data.else_label.unwrap(),id);
            id
        } else {
            end_label
        };


        let cond_address = self.value_map[&data.condition_label];


        let pre_if_block_label = self.current_block;
        self.builder.selection_merge(end_label, spirv::SelectionControl::NONE);
        self.builder.branch_conditional(cond_address, true_label, false_label, &[]);
        self.emit_block(Some(true_label));

        let block_code = data.true_block;
        let mut peekable_code = PeekableCode::new(block_code.iter());

        self.emit_all(&mut peekable_code);

        let post_then_block_label = self.current_block;

        self.builder.branch(end_label).unwrap();


        if data.false_block.is_some(){
            self.emit_block(Some(false_label));

            let block_code = data.false_block.unwrap();
            let mut peekable_code = PeekableCode::new(block_code.iter());

            self.emit_all(&mut peekable_code);


            self.builder.branch(end_label).unwrap();
        }
        // not used if else block was not emitted
        let post_false_block_label = self.current_block;

        println!("blocks, pre: {}, postif: {}, postelse: {}", pre_if_block_label, post_then_block_label, post_false_block_label);
        println!("labels, if: {}, else: {:?}, end: {}", data.if_label, data.else_label, data.end_label);

        self.emit_block(Some(end_label));


        for phi in data.phi_nodes {
            let ret =phi.0;
            let phi_record = match phi.1 {
                Operation::Phi(rec) => rec,
                _=>{panic!("Internal compiler error");}
            };

            let typ = if self.type_map.contains_key(&phi_record.new) {
                self.type_map[&phi_record.new]
            } else if self.type_map.contains_key(&phi_record.old) {
                self.type_map[&phi_record.old]
            } else {
                panic!("phi internal compiler error");
            };

            let spirv_type = self.ids.map_type(typ);


            let new_address = self.value_map[&phi_record.new];
            let old_address = self.value_map[&phi_record.old];

            let first = self.value_map[&phi_record.label];
            let second = self.value_map[&phi_record.old_label];

            let id =self.builder.phi(spirv_type,None, &[(new_address, first), (old_address, second)]).unwrap();

            self.value_map.insert(ret, id);
            self.type_map.insert(ret, typ);
        }
        println!("finished if else");
    }

    fn emit_loop(&mut self, data: LoopCode) {

    }

    pub fn emit(mut self){
        let mut code = self.iter.take().unwrap();
        self.emit_all(&mut code);

    }

    fn emit_all<'b, I2: std::iter::Iterator<Item=&'b Op>>(&mut self, code: &mut PeekableCode<'b, I2>) {
         while let Some((ret, op_code)) = code.next().copied() {
             self.emit_next(ret, op_code, code);
        }
    }

    fn emit_next<'b, I2: std::iter::Iterator<Item=&'b Op>>(&mut self, ret: usize, op_code: Operation, code: &mut PeekableCode<'b, I2>){
        match op_code {
            Operation::JumpIfElse(..)=>{
                let if_data = Self::find_if_else(ret, op_code, code);
                self.emit_if_else(if_data);
            },
            Operation::Jump(label)=>{
                let loop_data = Self::find_loop(ret, op_code, code);
                self.emit_loop(loop_data);
            }
            _=>{
                self.emit_operation(ret, op_code);
            }
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
            Operation::Label => {
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