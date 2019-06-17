extern crate lalrpop_util;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use octo_runtime::OctoModule;
use parser::ast;
use parser::codespan_reporting;
use parser::codespan::CodeMap;
use codespan_reporting::Diagnostic;

pub use shaderc::ShaderKind as Shader;


fn process_glsl_debug(path: &str, shader_type: Shader) {

    let code = std::fs::read_to_string(path).unwrap();

    let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
    let compilation_result = compiler
        .compile_into_spirv(
            &code,
            shader_type,
            &path,
            "main",
            None,
        )
        .map_err(|e| {
            println!("{}", e);
            "Couldn't compile fragment shader!"
        }).unwrap();


}

fn create_module(ast: ast::Program, module: &mut OctoModule) {
    let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
    for func in ast.items {
        let compilation_result = compiler
            .compile_into_spirv(
                &func.code.val,
                shaderc::ShaderKind::Fragment,
                &func.name.val,
                "main",
                None,
            )
            .map_err(|e| {
                println!("{}", e);
                "Couldn't compile fragment shader!"
            }).unwrap();
        module.fragment_shaders.insert(func.name.val, (func.code.val, compilation_result.as_binary_u8().to_owned()));

    }

    let vertex_compile_artifact = compiler
        .compile_into_spirv(
            &module.basic_vertex,
            shaderc::ShaderKind::Vertex,
            "vertex.vert",
            "main",
            None,
        )
        .map_err(|e| e.to_string() + "Couldn't compile vertex shader!").unwrap();
    module.basic_vertex_spirv = vertex_compile_artifact.as_binary_u8().to_owned();
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
        Err(failure_info) => {
            report_errors(&data, path, &[parser::ErrWrap{err: &failure_info.errors[0]}.into()]);
            return Result::Err(());
        }
        Ok(ast) => ast,
    };

// semantic analysis
    match semantics::analyze(&mut ast) {
        Result::Err(errs) => {
            let (mut errs, warnings) = errs;
            let diagnostics: Vec<Diagnostic> = warnings.into_iter().map(|x| semantics::WarningWrap::new(x).into()).collect();
            report_errors(&data, path, &diagnostics);
            panic!();
        }
        Result::Ok(()) => (),
    };
    let
        mut module =
        OctoModule::new();
    create_module(ast, &mut module);

    let
        module_data =
        serde_json::to_string_pretty(&module).
            unwrap();

    let
        mut output_file =
        File::create(&result_path).
            unwrap();
    output_file.
        write_all(module_data.as_bytes()).unwrap();

    println!("processed as: {:?}", result_path);
    Result::Ok(())
}

fn report_errors(src: &str, location: &str, messages: &[codespan_reporting::Diagnostic]) {
    let mut map = CodeMap::new();
    let src2 = src.to_string();
    map.add_filemap(location.to_string().into(), src2);
    use codespan_reporting::termcolor::StandardStream;
    let writer = StandardStream::stderr(codespan_reporting::termcolor::ColorChoice::Auto);
    for message in messages {
        let wr = &mut writer.lock();
        codespan_reporting::emit(wr, &map, message).unwrap();
    }
}

pub mod semantics;
