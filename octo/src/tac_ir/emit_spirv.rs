
use super::ir::{
    Address,
    Operation,
    Op,
    PipelineIR,
};
use super::ShaderDef;
use rspirv::mr::*;
use rspirv::binary::Assemble;
use spirv_headers as spirv;
use octo_runtime::*;


pub fn emit_spirv(module_name: &str, code: ShaderDef) ->  OctoModule{
    println!("Emitting spirv module");

    // let mut module = Builder::new();
    // module.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

    // let void_type = module.type_void();

    // let main_type = module.type_function(void_type, vec![void_type]);
    // module.begin_function(void_type,
    //                  None,
    //                  spirv::FunctionControl::DONT_INLINE |
    //                   spirv::FunctionControl::CONST,
    //                  main_type)
    //  .unwrap();

    // // emitting main function
    // module.begin_basic_block(None).unwrap();
    // module.ret().unwrap();
    // module.end_function().unwrap();

    // module.module().assemble()
}