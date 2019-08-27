extern crate lalrpop_util;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use octo_runtime::*;
use parser::ast;
use parser::codespan_reporting;
use parser::codespan::CodeMap;
use codespan_reporting::Diagnostic;

pub use shaderc::ShaderKind as Shader;
use std::borrow::ToOwned;
use std::collections::HashMap;


pub fn emit_test_module(path: &Path) {

    let basic_vertex = include_str!("basic_vertex.glsl");
    let compiled_vertex = process_glsl(basic_vertex, "basic vertex",Shader::Vertex);

    let mut fragment_shaders = HashMap::new();

    let basic_fragment = include_str!("basic_fragment.glsl");
    let compiled_fragment = process_glsl(basic_fragment, "basic fragment", Shader::Fragment);

    fragment_shaders.insert(0, (basic_fragment.to_owned(), compiled_fragment));


    let mut shader_passes = vec![];

    shader_passes.push(
        ShaderPass{
            id: 0,
            input: vec![
                InputType::ProvidedTexture(0),
                InputType::ProvidedTexture(1),
                InputType::ProvidedTexture(2),
            ],
            output: OutputType::Result,
            shader: 0,
            dependencies: None,
        }
    );

    let module = OctoModule{
        name: "Debug module".to_owned(),
        version: 0,
        basic_vertex: basic_vertex.to_owned(),
        basic_vertex_spirv: compiled_vertex,
        fragment_shaders,
        required_input: vec![
            ("color".to_owned(), TextureType::Vec4),
            ("normal".to_owned(), TextureType::Vec4),
            ("albedo".to_owned(), TextureType::Vec4),
        ],
        textures: vec![],
        passes: shader_passes,
    };


    // saving module to file
    {
        let
            module_data =
            serde_json::to_string_pretty(&module).
                unwrap();

        let
            mut output_file =
            File::create(&path).
                unwrap();
        output_file.
            write_all(module_data.as_bytes()).unwrap();
    }
}

pub fn process_glsl(code: &str,path: &str, shader_type: Shader) -> Vec<u32>{

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
    compilation_result.as_binary().to_vec()
}

//pub fn process_glsl_debug(path: &str, shader_type: Shader) {
//
//    let code = std::fs::read_to_string(path).unwrap();
//    let result = process_glsl(&code, path, shader_type);
//
//    let new_name = path.to_owned() + ".spirv";
//    let mut file = std::fs::File::create(new_name).unwrap();
//    file.write_all(&result);
//    file.
//
//
//}

fn create_module(ast: ast::Program, module: &mut OctoModule) {
    let mut ast = ast;
    let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
    for (id, func) in ast.items.iter_mut().enumerate() {
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
        module.fragment_shaders.insert(id, (func.code.val.clone(), compilation_result.as_binary().to_owned()));

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
    module.basic_vertex_spirv = vertex_compile_artifact.as_binary().to_owned();
}

pub fn process_file(path: &str) -> Result<(), ()> {
    println!("Processing file at: {}", path);
    println!("rerun-if-changed={}",path);
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    emit_test_module(&result_path);
/*
    //let mut file = File::open(path).unwrap();
    //let mut data = String::new();
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
    */
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
