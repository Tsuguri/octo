use octo_parser::ast::*;
mod env;

use self::env::{Env, ScopeToken};

pub enum SemanticError {
    UndefinedIdentifier,
    UnusedArgument,
    TypeMismatch,
    ArgumentsNumberMismatch,
}

pub fn analyze(program: Program) {
    println!("Analyzing program semantics");
    let mut env = Env::new();
    println!("Using global env: {:?}", env);
    // analyzing functions prototypes and adding to env
    let scope_token = env.push_scope();
    for item in program.items {
        match item {
            ProgramItem::Function(func) => analyze_function(&func, &mut env, &scope_token),
            ProgramItem::GpuFunction(func) => analyze_gpu_function(&func, &mut env),
        }
    }
    println!("Global env after analysis: {:?}", env);
    env.pop_scope(scope_token);
    println!("Global env after pop: {:?}", env);
}

fn analyze_function(function: &Function, env: &mut Env, token: &ScopeToken) {
    println!("-- Analyzing function {}", function.name);
    let mut ready = true;
    for arg in &function.arguments {
        if !env.check_type(&arg.1) {
            println!("Unknow argument type");
            ready = false;
        }
    }
    if let Some(x) = &function.ret {
        if !env.check_type(x) {
            println!("Bad return type",);
            ready = false;
        }
    }
    if ready {
        env.add_function(function, token);
    }
}

fn analyze_gpu_function(function: &GpuFunction, env: &mut Env) {
    println!("-- Analyzing function {}", function.name);
}
