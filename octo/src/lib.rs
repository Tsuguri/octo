extern crate lalrpop_util;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use octo_runtime::OctoModule;
use parser::ast;

fn create_module(ast: ast::Program, module: &mut OctoModule) {
    for fun in ast.items {
        let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
        match fun {
            ast::ProgramItem::Function(_) => (),
            ast::ProgramItem::GpuFunction(func) => {
                let compilation_result = compiler
                    .compile_into_spirv(
                        &func.code,
                        shaderc::ShaderKind::Fragment,
                        &func.name,
                        "main",
                        None,
                    )
                    .map_err(|e| {
                        println!("{}", e);
                        "Couldn't compile fragment shader!"
                    }).unwrap();
                module.fragment_shaders.insert(func.name, (func.code, compilation_result.as_binary_u8().to_owned()));
            }
        }

        let vertex_compile_artifact = compiler
            .compile_into_spirv(
                &module.basic_vertex,
                shaderc::ShaderKind::Vertex,
                "vertex.vert",
                "main",
                None,
            )
            .map_err(|_| "Couldn't compile vertex shader!").unwrap();
        module.basic_vertex_spirv = vertex_compile_artifact.as_binary_u8().to_owned();
    }
}

pub fn process_file(path: &str) -> Result<(), ()> {
    println!("Processing file at: {}", path);
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();


    // syntax analysis
    let mut ast = match parser::parse(path, &data, false) {
        Err(warning) => {
            println!("Command failed: {:?}", warning);
            return Result::Err(());
        },
        Ok(ast) => ast,
    };

    // semantic analysis
    semantics::analyze(&mut ast).map_err(|errs| {
        for error in errs {
            println!("--> {:?}", error);
        }
    }).unwrap();

    let mut module = OctoModule::new();
    create_module(ast, &mut module);

    let module_data = serde_json::to_string_pretty(&module).unwrap();

    let mut output_file = File::create(&result_path).unwrap();
    output_file.write_all(module_data.as_bytes()).unwrap();

    println!("processed as: {:?}", result_path);
    Result::Ok(())
}
pub mod semantics;
