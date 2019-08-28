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

use askama::Template;
use parser::ast::GpuFunction;

static VERTEX: &str = include_str!("basic_vertex.glsl");

#[derive(Template)] // this will generate the code...
#[template(path = "shader.glsl", escape = "none")] // using the template in this path, relative
// to the templates dir in the crate root
struct ShaderTemplate<'a> {
    code: &'a str,
    input: Vec<(String, String)>,
    output: Vec<(String, String)>,
}


fn type_string(x: &ast::Type) -> &'static str {
    use ast::Type::*;
    let t = match x {
        Float => "float",
        Vec2 => "vec2",
        Vec3 => "vec3",
        Vec4 => "vec4",
        _ => panic!("shouldn't ever happen"),
    };
    t
}

fn construct_function(function: &GpuFunction) -> String {
    let input = function.arguments.iter().map(|x| {
        (type_string(&x.typ).to_owned(), x.identifier.val.clone())
    }).collect();

    let output = function.results.iter().map(|x| {
        (type_string(&x.typ).to_owned(), x.identifier.val.clone())
    }).collect();


    let template = ShaderTemplate {
        input,
        output,
        code: &function.code.val,
    };

    template.render().unwrap()
}

pub fn emit_module(name: &str, program: ast::Program, path: &Path) {
    use ast::Type;

    let mut fragments_map = HashMap::new();
    let mut fragment_shaders = HashMap::new();

    let mut current_id = 0;
    for function in program.items {
        let name = function.name.val.clone();
        let fragment_code = construct_function(&function);
        let id = current_id;
        current_id = current_id + 1;

        println!("{}", fragment_code);

        let compiled_fragment = process_glsl(&fragment_code, &name, Shader::Fragment);
        fragments_map.insert(name, (function, id));

        fragment_shaders.insert(id, compiled_fragment);
    }

    let basic_vertex = VERTEX;
    let compiled_vertex = process_glsl(basic_vertex, "basic vertex", Shader::Vertex);

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

    let module = OctoModule {
        name: name.to_owned(),
        version: 0,
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

pub fn process_glsl(code: &str, path: &str, shader_type: Shader) -> Vec<u32> {
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

pub fn process_file(path: &str) -> Result<(), ()> {
    println!("Processing file at: {}", path);
    println!("rerun-if-changed={}", path);
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");
    let module_name =p.file_stem().unwrap().to_str().unwrap();
    println!("{}", module_name);

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();


    // syntax analysis
    let mut ast = match parser::parse(path, &data, false) {
        Err(failure_info) => {
            report_errors(&data, path, &[parser::ErrWrap { err: &failure_info.errors[0] }.into()]);
            return Result::Err(());
        }
        Ok(ast) => ast,
    };

    emit_module(&module_name, ast, &result_path);
    // semantic analysis
    /*
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
