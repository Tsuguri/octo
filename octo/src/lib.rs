//extern crate lalrpop_util;

mod shader_generation;
mod static_analysis;
mod tac_ir;
pub mod experimental_ir;


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


use log::{info, trace};
use crate::static_analysis::Diagnostics;

fn generate_fragment_shaders(program: &mut ast::Program) -> (HashMap<usize, Vec<u32>>, HashMap<String, (ast::GpuFunction, usize)>) {
    let mut fragment_shaders = HashMap::new();
    let mut fragments_map = HashMap::new();

    let mut current_id = 0;
    for function in program.items.drain(0..program.items.len()) {
        let id = current_id;
        current_id = current_id + 1;

        let compiled_fragment = shader_generation::construct_function(&function);

        fragments_map.insert(function.name.val.clone(), (function, id));
        fragment_shaders.insert(id, compiled_fragment);
    }
    (fragment_shaders, fragments_map)
}

fn emit_module(program: ast::Program, path: &Path) {
    let mut program = program;


    let (fragment_shaders, fragments_map) = generate_fragment_shaders(&mut program);

    let compiled_vertex = shader_generation::basic_vertex();

    let pipeline = &program.pipeline;

    trace!("{:#?}", pipeline);
    // generate shader passes based on pipeline ast
    // generate needed textures from pipeline ast


    let mut shader_passes = vec![];

    let shader_id = fragments_map["test_name"].1;

    shader_passes.push(
        ShaderPass {
            id: shader_id,
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

    let required_input: Vec<_> = pipeline.arguments.iter().map(|x| {
        let t = match x.typ {
            ast::Type::Float => TextureType::Float,
            ast::Type::Vec2 => TextureType::Vec2,
            ast::Type::Vec3 => TextureType::Vec3,
            ast::Type::Vec4 => TextureType::Vec4,
            _ => unreachable!(),
        };
        (x.identifier.val.clone(), t)
    }).collect();

    let module = OctoModule {
        name: pipeline.name.val.clone(),
        version: 0,
        basic_vertex_spirv: compiled_vertex,
        fragment_shaders,
        required_input,
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


pub fn process_file(path: &str) -> Result<(), ()> {
    info!("Processing file at: {}", path);
    info!("rerun-if-changed={}", path);
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();


    // syntax analysis
    let mut ast = match parser::parse(&data, false) {
        Err(failure_info) => {
            println!("{:#?}", failure_info.errors);
            report_errors(&data, path, &[parser::ErrWrap { err: &failure_info.errors[0] }.into()]);
            return Result::Err(());
        }
        Ok(ast) => ast,
    };

    //println!("{:#?}", ast);

    let static_analysis_res = static_analysis::analyze(ast);
    let Diagnostics{errors, warnings} = static_analysis_res.1;
    let error_happened = errors.len() > 0;
    let mut diagnostics: Vec<Diagnostic> = warnings.into_iter().map(|x| semantics::WarningWrap::new(x).into()).collect();
    diagnostics.extend(errors.into_iter().map(|x| semantics::ErrorWrap::new(x).into()));
    report_errors(&data, path, &diagnostics);
    if error_happened {
        return Result::Err(());
    }
    let valid_ast = match static_analysis_res.0 {
        None => unreachable!(),
        Some(x) => x,
    };

    let tac = tac_ir::emit_ir(valid_ast);

    println!("before constant propagation");
    for op in &tac {
        println!("{} = {:?}",op.0, op.1);

    }

    let tac = tac_ir::propagate_constants(tac);

    println!("after constant propagation");
    for op in tac {
        println!("{} = {:?}",op.0, op.1);

    }

    let tac = tac_ir::remove_unused_operations(tac);

    println!("after unused operation removal");
    for op in tac {
        println!("{} = {:?}",op.0, op.1);

    }
    // do next things

    //emit_module(ast, &result_path);

    Result::Ok(())
}

fn report_errors(src: &str, location: &str, messages: &[codespan_reporting::Diagnostic]) {
    let mut map = CodeMap::new();
    let src2 = src.to_owned();
    map.add_filemap(location.to_owned().into(), src2);
    use codespan_reporting::termcolor::StandardStream;
    let writer = StandardStream::stderr(codespan_reporting::termcolor::ColorChoice::Auto);
    for message in messages {
        let wr = &mut writer.lock();
        codespan_reporting::emit(wr, &map, message).unwrap();
    }
}

pub mod semantics;
