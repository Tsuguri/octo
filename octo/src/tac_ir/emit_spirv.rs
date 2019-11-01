
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

fn emit_single_shader(info: ShaderDef)->Vec<u32> {
    println!("Emitting single fragment shader\n\n");

    let mut module = Builder::new();
    let glsl = module.ext_inst_import("GLSL.std.450");
    module.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

    let function_id = module.id();

    module.entry_point(spirv::ExecutionModel::Fragment, function_id, "main", &[]);
    module.execution_mode(function_id, spirv::ExecutionMode::OriginUpperLeft);

    // generate IDs for all things

    // here goes all decorations
    // descriptor sets defs
    // binding defs
    // uniform defs
    // locations
    // etc

    // probably IDs generation should be done beforehand for all constructs

    // generate all types and store IDs

    // generate all module-level variables

    let void_type = module.type_void();

    let main_type = module.type_function(void_type, vec![]);
    module.begin_function(void_type,
                     Some(function_id),
                     spirv::FunctionControl::DONT_INLINE |
                      spirv::FunctionControl::CONST,
                     main_type)
     .unwrap();

    // emitting main function
    module.begin_basic_block(None).unwrap();
    module.ret().unwrap();
    module.end_function().unwrap();

    let m =module.module();

    println!("{}", m.disassemble());
    vec![]
}