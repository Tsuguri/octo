use super::ir;
use super::{PipelineDef, ShaderDef};
use ir::{Address, Op, Operation, PipelineIR, ValueType};
use log::error;
use octo_runtime::{
    TextureType as RTTextureType,
    ValueType as RTValueType,
    OctoModule,
    ShaderPass,
    TextureSize,
};
use rspirv::binary::{Assemble, Disassemble};
use rspirv::mr::*;
use spirv_headers as spirv;
use std::collections::HashMap;

use shaderc::ShaderKind as Shader;

static VERTEX: &str = include_str!("../basic_vertex.glsl");

mod for_loop;
mod ids;
mod if_else;
mod main_emitter;
mod peekable_code;

use for_loop::{find_loop, LoopCode};
use ids::SpirvIds;
use if_else::{find_if_else, IfElseCode};
use main_emitter::MainEmitter;
use peekable_code::PeekableCode;

// TODO: move this to library compilation stage (build.rs)
fn create_basic_vertex() -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new()
        .ok_or("shaderc not found!")
        .unwrap();
    let compilation_result = compiler
        .compile_into_spirv(&VERTEX, Shader::Vertex, "basic_vertex", "main", None)
        .map_err(|e| {
            error!("{}", e);
            "Couldn't compile fragment shader!"
        })
        .unwrap();
    compilation_result.as_binary().to_vec()
}

fn version() -> u32 {
    let version = env!("CARGO_PKG_VERSION");
    let numbers: Vec<_> = version.split('.').collect();
    assert!(numbers.len() == 3);
    let numbers: Vec<u32> = numbers.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let final_version = numbers[0] * 10000 + numbers[1] * 100 + numbers[2];
    //println!("{}", final_version);
    final_version
}

pub fn emit_spirv(module_name: &str, code: PipelineDef) -> OctoModule {
    let mut code = code;
    println!("Emitting spirv module");

    let mut module = OctoModule::new();
    module.name = module_name.to_owned();
    module.version = version();
    module.basic_vertex_spirv = create_basic_vertex();

    module.required_input = code
        .args
        .drain(0..code.args.len())
        .map(|(x, y)| {
            let x = match x {
                ValueType::Vec2 => RTValueType::Vec2,
                ValueType::Vec3 => RTValueType::Vec3,
                ValueType::Vec4 => RTValueType::Vec4,
                ValueType::Int => RTValueType::Int,
                ValueType::Bool => RTValueType::Bool,
                _ => RTValueType::Float,
            };

            (y, x)
        })
        .collect();

    for (id, shader_ir) in code.shaders.drain(0..code.shaders.len()).enumerate() {
        let shader_spirv = emit_single_shader(shader_ir, &code.uniforms);
        module.fragment_shaders.insert(id, shader_spirv);
    }

    for (id, def) in code.passes.drain(0..code.passes.len()).enumerate() {
        let octo_pass = ShaderPass {
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
            ValueType::Vec2 => RTTextureType::Vec2,
            ValueType::Vec3 => RTTextureType::Vec3,
            ValueType::Vec4 => RTTextureType::Vec4,
            _ => RTTextureType::Float,
        };
        // probably to be changed once Scale operation is implemented
        let size = TextureSize::Original;
        module.textures.push((id, typ, size));
    }

    module
}

fn emit_single_shader(info: ShaderDef, uniforms: &Vec<(ValueType, String)>) -> Vec<u32> {
    println!("Emitting single fragment shader\n\n");

    let mut module = Builder::new();
    module.capability(spirv::Capability::Shader);
    let glsl = module.ext_inst_import("GLSL.std.450");
    module.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

    let mut ids = SpirvIds::new();

    let function_id = module.id();
    ids.generate_ids(&mut module, &info);

    let interface = ids.interface_ids();

    module.entry_point(
        spirv::ExecutionModel::Fragment,
        function_id,
        "main",
        &interface,
    );
    module.execution_mode(function_id, spirv::ExecutionMode::OriginUpperLeft, &[]);

    ids.generate_types(&mut module, &info, uniforms);
    ids.decorate(&mut module, uniforms);
    ids.create_uniform_variables(&mut module, &info, uniforms);
    ids.store_constants(&mut module, &info);

    let main_type = module.type_function(ids.map_type(ValueType::Void), vec![]);
    module
        .begin_function(
            ids.map_type(ValueType::Void),
            Some(function_id),
            spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST,
            main_type,
        )
        .unwrap();

    // emitting main function
    let emitter = MainEmitter::new(
        &mut ids,
        &mut module,
        info.input_type.clone(),
        uniforms.iter().map(|x| x.0).collect(),
        glsl,
        info.code.iter(),
    );

    emitter.emit();

    module.end_function().unwrap();

    let m = module.module();

    println!("{}", m.disassemble());
    m.assemble()
}
