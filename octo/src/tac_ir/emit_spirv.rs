use super::ir;
use super::{PipelineDef, ShaderDef};
use ir::{Address, Op, Operation, PipelineIR, ValueType};
use octo_runtime::{
    OctoModule, ShaderPass, TextureSize, TextureType as RTTextureType, ValueType as RTValueType,
};
use rspirv::binary::{Assemble, Disassemble};
use rspirv::dr::Builder;
use spirv_headers as spirv;
use std::collections::HashMap;

mod emit_std;
mod ids;
mod main_emitter;

use ids::SpirvIds;
use main_emitter::MainEmitter;

fn create_basic_vertex() -> Vec<u32> {
    use rspirv::spirv::{
        AddressingModel, BuiltIn, Capability, Decoration, ExecutionModel, FunctionControl,
        MemoryModel, StorageClass,
    };

    let mut module = Builder::new();
    module.capability(Capability::Shader);
    module.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);

    let void_type = module.type_void();
    let int_type = module.type_int(32, 1);
    let float_type = module.type_float(32);
    let vec2_type = module.type_vector(float_type, 2);
    let vec4_type = module.type_vector(float_type, 4);

    let vertex_index_pointer = module.type_pointer(None, StorageClass::Input, int_type);
    let position_pointer = module.type_pointer(None, StorageClass::Output, vec4_type);
    let uv_pointer = module.type_pointer(None, StorageClass::Output, vec2_type);

    let vertex_index_location = module.id();
    let position_location = module.id();
    let uv_location = module.id();

    module.variable(
        vertex_index_pointer,
        Some(vertex_index_location),
        StorageClass::Input,
        None,
    );
    module.variable(
        position_pointer,
        Some(position_location),
        StorageClass::Output,
        None,
    );
    module.variable(uv_pointer, Some(uv_location), StorageClass::Output, None);

    module.decorate(
        vertex_index_location,
        Decoration::BuiltIn,
        [BuiltIn::VertexIndex.into()],
    );
    module.decorate(
        position_location,
        Decoration::BuiltIn,
        [BuiltIn::Position.into()],
    );
    module.decorate(uv_location, Decoration::Location, [0u32.into()]);

    let int_one = module.constant_bit32(int_type, 1);
    let int_two = module.constant_bit32(int_type, 2);
    let float_zero = module.constant_bit32(float_type, f32::to_bits(0.0));
    let float_one = module.constant_bit32(float_type, f32::to_bits(1.0));
    let float_two = module.constant_bit32(float_type, f32::to_bits(2.0));
    let float_neg_one = module.constant_bit32(float_type, f32::to_bits(-1.0));
    let vec2_two = module.constant_composite(vec2_type, [float_two, float_two]);
    let vec2_neg_one = module.constant_composite(vec2_type, [float_neg_one, float_neg_one]);

    let function_id = module.id();
    module.entry_point(
        ExecutionModel::Vertex,
        function_id,
        "main",
        &[vertex_index_location, position_location, uv_location],
    );

    let main_type = module.type_function(void_type, vec![]);
    module
        .begin_function(
            void_type,
            Some(function_id),
            FunctionControl::NONE,
            main_type,
        )
        .unwrap();
    module.begin_block(None).unwrap();

    let vertex_index = module
        .load(int_type, None, vertex_index_location, None, [])
        .unwrap();
    let shifted_vertex_index = module
        .shift_left_logical(int_type, None, vertex_index, int_one)
        .unwrap();
    let uv_x_int = module
        .bitwise_and(int_type, None, shifted_vertex_index, int_two)
        .unwrap();
    let uv_y_int = module
        .bitwise_and(int_type, None, vertex_index, int_two)
        .unwrap();
    let uv_x = module.convert_s_to_f(float_type, None, uv_x_int).unwrap();
    let uv_y = module.convert_s_to_f(float_type, None, uv_y_int).unwrap();
    let uv = module
        .composite_construct(vec2_type, None, [uv_x, uv_y])
        .unwrap();

    let scaled_uv = module.f_mul(vec2_type, None, uv, vec2_two).unwrap();
    let position_xy = module
        .f_add(vec2_type, None, scaled_uv, vec2_neg_one)
        .unwrap();
    let position_x = module
        .composite_extract(float_type, None, position_xy, [0])
        .unwrap();
    let position_y = module
        .composite_extract(float_type, None, position_xy, [1])
        .unwrap();
    let position = module
        .composite_construct(
            vec4_type,
            None,
            [position_x, position_y, float_zero, float_one],
        )
        .unwrap();

    module.store(uv_location, uv, None, []).unwrap();
    module.store(position_location, position, None, []).unwrap();
    module.ret().unwrap();
    module.end_function().unwrap();

    module.module().assemble()
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

fn uniform_size(typ: ValueType) -> usize {
    use ValueType::*;
    match typ {
        ValueType::Float => 16,
        ValueType::Vec2 => 16,
        ValueType::Vec3 => 16,
        ValueType::Vec4 => 16,
        ValueType::Mat3 => 36,
        ValueType::Mat4 => 64,
        ValueType::Int => 16,
        ValueType::Bool => 16,
        _ => panic!(),
    }
}

fn map_type(typ: ir::ValueType) -> octo_runtime::ValueType {
    use ir::ValueType as it;
    use octo_runtime::ValueType as rt;
    match typ {
        it::Float => rt::Float,
        it::Vec2 => rt::Vec2,
        it::Vec3 => rt::Vec3,
        it::Vec4 => rt::Vec4,
        it::Mat3 => rt::Mat3,
        it::Mat4 => rt::Mat4,
        it::Int => rt::Int,
        it::Bool => rt::Bool,
        _ => panic!(),
    }
}

pub fn emit_spirv(module_name: &str, code: PipelineDef) -> OctoModule {
    let mut code = code;
    //println!("Emitting spirv module");

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

    module.uniform_block_size = code.uniforms.iter().map(|x| uniform_size(x.0)).sum();
    module.uniform_block = code
        .uniforms
        .iter()
        .map(|x| (x.1.clone(), map_type(x.0)))
        .collect();

    module
}

fn emit_single_shader(info: ShaderDef, uniforms: &Vec<(ValueType, String)>) -> Vec<u32> {
    if true {
        println!("Emitting single fragment shader for:\n\n");
        for c in &info.code {
            println!("{:?}", c);
        }
    }

    let mut module = Builder::new();
    module.capability(rspirv::spirv::Capability::Shader);
    let glsl = module.ext_inst_import("GLSL.std.450");
    module.memory_model(
        rspirv::spirv::AddressingModel::Logical,
        rspirv::spirv::MemoryModel::GLSL450,
    );

    let mut ids = SpirvIds::new();

    let function_id = module.id();
    ids.generate_ids(&mut module, &info);

    let interface = ids.interface_ids();

    module.entry_point(
        rspirv::spirv::ExecutionModel::Fragment,
        function_id,
        "main",
        &interface,
    );
    module.execution_mode(
        function_id,
        rspirv::spirv::ExecutionMode::OriginUpperLeft,
        &[],
    );

    ids.generate_types(&mut module, &info, uniforms);
    ids.decorate(&mut module, uniforms);
    ids.create_uniform_variables(&mut module, &info, uniforms);
    ids.store_constants(&mut module, &info);

    let main_type = module.type_function(ids.map_type(ValueType::Void), vec![]);
    module
        .begin_function(
            ids.map_type(ValueType::Void),
            Some(function_id),
            rspirv::spirv::FunctionControl::DONT_INLINE | rspirv::spirv::FunctionControl::CONST,
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

    //println!("{}", m.disassemble());
    m.assemble()
}

#[cfg(test)]
mod tests {
    use super::create_basic_vertex;
    use rspirv::dr::{load_words, Module, Operand};
    use rspirv::spirv::{BuiltIn, Decoration, ExecutionModel};

    fn decorated_builtin_id(module: &Module, built_in: BuiltIn) -> Option<u32> {
        module.annotations.iter().find_map(|inst| match inst.operands.as_slice() {
            [
                Operand::IdRef(id),
                Operand::Decoration(Decoration::BuiltIn),
                Operand::BuiltIn(actual),
            ] if *actual == built_in => Some(*id),
            _ => None,
        })
    }

    fn location_id(module: &Module, location: u32) -> Option<u32> {
        module.annotations.iter().find_map(|inst| match inst.operands.as_slice() {
            [
                Operand::IdRef(id),
                Operand::Decoration(Decoration::Location),
                Operand::LiteralBit32(actual),
            ] if *actual == location => Some(*id),
            _ => None,
        })
    }

    #[test]
    fn basic_vertex_shader_declares_expected_interface() {
        let words = create_basic_vertex();
        let module = load_words(&words).expect("basic vertex SPIR-V should load");

        let entry_point = module
            .entry_points
            .iter()
            .find(|inst| {
                matches!(
                    inst.operands.as_slice(),
                    [
                        Operand::ExecutionModel(ExecutionModel::Vertex),
                        Operand::IdRef(_),
                        Operand::LiteralString(name),
                        ..
                    ] if name == "main"
                )
            })
            .expect("basic vertex SPIR-V should contain a vertex main entry point");

        let position_id = decorated_builtin_id(&module, BuiltIn::Position)
            .expect("basic vertex SPIR-V should decorate gl_Position");
        let vertex_index_id = decorated_builtin_id(&module, BuiltIn::VertexIndex)
            .expect("basic vertex SPIR-V should decorate gl_VertexIndex");
        let uv_id =
            location_id(&module, 0).expect("basic vertex SPIR-V should decorate location 0 UV");

        let interface = &entry_point.operands[3..];
        assert!(interface.contains(&Operand::IdRef(position_id)));
        assert!(interface.contains(&Operand::IdRef(vertex_index_id)));
        assert!(interface.contains(&Operand::IdRef(uv_id)));
    }
}
