//extern crate lalrpop_util;

pub mod semantics;
mod static_analysis;
mod tac_ir;
mod prototypes;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use codespan_reporting::Diagnostic;
use parser::ast;
use parser::codespan::CodeMap;
use parser::codespan_reporting;

use std::borrow::ToOwned;

use crate::static_analysis::Diagnostics;
use log::info;

pub fn process_file(path: &str) -> Result<(), ()> {
    info!("Processing file at: {}", path);

    //info!("rerun-if-changed={}", path);

    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let ast = parse_data(&data, path)?;

    let static_analysis_res = static_analysis::analyze(ast);
    let Diagnostics { errors, warnings } = static_analysis_res.1;
    let error_happened = errors.len() > 0;
    let mut diagnostics: Vec<Diagnostic> = warnings
        .into_iter()
        .map(|x| semantics::WarningWrap::new(x).into())
        .collect();
    diagnostics.extend(
        errors
            .into_iter()
            .map(|x| semantics::ErrorWrap::new(x).into()),
    );
    report_errors(&data, path, &diagnostics);
    if error_happened {
        return Result::Err(());
    }
    let valid_ast = match static_analysis_res.0 {
        None => unreachable!(),
        Some(x) => x,
    };

    let tac = tac_ir::emit_ir(valid_ast);
    tac_ir::emit_graph(&tac, &(path.to_owned() + "1"));
    println!("before constant propagation");
    println!("{:?}", tac);

    let tac = tac_ir::propagate_constants(tac);
    tac_ir::emit_graph(&tac,&(path.to_owned() + "2"));
    println!("after constant propagation");
    println!("{:?}", tac);

    let tac = tac_ir::remove_unused_operations(tac);
    println!("after unused operation removal");
    println!("{:?}", tac);

    let pipeline_definition = tac_ir::split_passes(tac);

    let module = tac_ir::emit_spirv(path, pipeline_definition);

    //let shaders: Vec<_> = pipeline_definition.shaders.iter().map(|x| tac_ir::emit_spirv(x)).collect();

    //tac_ir::emit_spirv(tac);

    let mut output_file = std::fs::File::create(&result_path).unwrap();
    let data = serde_json::to_string(&module).unwrap();
    use std::io::Write;
    output_file.write_all(data.as_bytes()).unwrap();

    //emit debug directory

    {
        let dir_name = p.file_stem().unwrap();
        println!("dir name:{:?}", dir_name);
        let _ = std::fs::create_dir_all(&dir_name).unwrap();

        for (id, shader) in module.fragment_shaders {
            let mut dir_n = std::path::PathBuf::from(dir_name);
            dir_n.push(id.to_string());
            dir_n.set_extension("frag");
            let mut file = std::fs::File::create(&dir_n).unwrap();
            let data: Vec<u8> = shader
                .iter()
                .map(|x| x.to_le_bytes())
                .collect::<Vec<_>>()
                .iter()
                .flatten()
                .cloned()
                .collect(); //.flatten().collect();
            file.write_all(&data).unwrap();
        }
    }

    Result::Ok(())
}

fn parse_data(data: &str, path: &str) -> Result<ast::Pipeline, ()> {
    match parser::parse(data, false) {
        Err(failure_info) => {
            println!("{:#?}", failure_info.errors);
            report_errors(
                data,
                path,
                &[parser::ErrWrap {
                    err: &failure_info.errors[0],
                }
                .into()],
            );
            Result::Err(())
        }
        Ok(ast) => Result::Ok(ast),
    }
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
