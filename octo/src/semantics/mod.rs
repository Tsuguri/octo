use parser::ast::*;

use self::env::Scope;
use errors::SemanticError;

mod env;

pub fn analyze(program: &mut Program) -> Result<(), Vec<SemanticError>> {
    println!("Analyzing program semantics");
    let global_scope = Scope::global();
    println!("Using global env: {:?}", global_scope);
    // analyzing functions prototypes and adding to env
    let mut program_scope = Scope::child_scope(&global_scope);
    // Adding function definitions to scope.
    #[allow(unused_mut)]
    let mut errors = vec![];
    for item in &program.items {
        analyze_gpu_function_header(item, &program_scope);
    }

    // Analyzing content of functions
    for item in &program.items {
        analyze_gpu_function_content(&item, &mut program_scope)
    }

    if errors.len() > 0 {
        Result::Err(errors)
    } else {
        Result::Ok(())
    }
}

fn analyze_gpu_function_header(function: &GpuFunction, _env: &Scope) {
    println!("-- Analyzing function {}", function.name);
}

fn analyze_gpu_function_content(_function: &GpuFunction, _env: &mut Scope) {
//
}
