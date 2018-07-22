use octo_parser::ast::*;
mod env;

use self::env::Scope;

#[derive(Debug)]
pub enum SemanticError {
    UndefinedIdentifier,
    UnusedArgument,
    TypeMismatch,
    ArgumentsNumberMismatch,
}

pub fn analyze(program: Program) -> Result<(), Vec<SemanticError>> {
    println!("Analyzing program semantics");
    let global_scope = Scope::global();
    println!("Using global env: {:?}", global_scope);
    // analyzing functions prototypes and adding to env
    let mut program_scope = Scope::child_scope(&global_scope);
    // Adding function definitions to scope.
    let mut errors = vec![];
    for item in &program.items {
        match item {
            ProgramItem::Function(func) => match analyze_function_header(&func, &program_scope) {
                Result::Ok(_) => program_scope.add_function(func),
                Result::Err(err) => errors.push(err),
            },
            ProgramItem::GpuFunction(func) => analyze_gpu_function_header(&func, &program_scope),
        }
    }

    // Analyzing content of functions
    for item in &program.items {
        match item {
            ProgramItem::Function(func) => analyze_function_content(&func, &mut program_scope),
            ProgramItem::GpuFunction(func) => {
                analyze_gpu_function_content(&func, &mut program_scope)
            }
        }
    }
    if errors.len() > 0 {
        Result::Err(errors)
    } else {
        Result::Ok(())
    }
}

fn analyze_function_header(function: &Function, env: &Scope) -> Result<(), SemanticError> {
    for arg in &function.arguments {
        if !env.check_type(&arg.1) {
            println!("Unknow argument type");
            return Result::Err(SemanticError::UndefinedIdentifier);
        }
    }
    if let Some(x) = &function.ret {
        if !env.check_type(x) {
            println!("Bad return type",);
            return Result::Err(SemanticError::UndefinedIdentifier);
        }
    }
    Result::Ok(())
}

fn analyze_function_content(function: &Function, env: &mut Scope) {
    for arg in &function.arguments {
        env.add_variable(&arg.0.identifier, arg.1.clone());
    }
    analyze_block(&function.block, env);
}

fn analyze_gpu_function_header(function: &GpuFunction, _env: &Scope) {
    println!("-- Analyzing function {}", function.name);
}

fn analyze_gpu_function_content(_function: &GpuFunction, _env: &mut Scope) {
    //
}

fn analyze_block(block: &Block, env: &Scope) {
    println!("--> Analyzing content of block with env: {:?}", env);
    let mut block_scope = Scope::child_scope(env);
    for statement in &block.statements {
        analyze_statement(statement, &mut block_scope);
    }
}

fn analyze_statement(statement: &Statement, env: &mut Scope) {
    match statement {
        Statement::Assignment(var, exp, creating) => {
            analyze_assignment(var, exp, env, *creating);
        }
        Statement::Expression(exp) => {
            analyze_expression(exp, env);
        }
    };
}

fn analyze_expression(expression: &Expression, ev: &mut Scope) -> Type {
    println!("Found expression");
    use octo_parser::ast::Expression::*;
    match expression {
        Add(exp1, exp2) => (),
        And(exp1, exp2) => (),
        Variable(var) => (),
        Literal(value) => (),
        Negation(exp) => (),
        Mul(exp1, exp2) => (),
        Div(exp1, exp2) => (),
        Add(exp1, exp2) => (),
        Sub(exp1, exp2) => (),
        Less(exp1, exp2) => (),
        LessEqual(exp1, exp2) => (),
        More(exp1, exp2) => (),
        MoreEqual(exp1, exp2) => (),
        Equals(exp1, exp2) => (),
        NotEquals(exp1, exp2) => (),
        And(exp1, exp2) => (),
        Or(exp1, exp2) => (),
        Invocation(func_name, arguments) => (),
    }
    Type::Unknown
}

fn analyze_assignment(
    variable: &Variable,
    expression: &Expression,
    env: &mut Scope,
    creating: bool,
) {
    let typ = analyze_expression(expression, env);
    match creating {
        true => {
            env.add_variable(&variable.identifier, typ);
        }
        false => {
            match env.check_variable(&variable.identifier) {
                None => {
                    //
                    // return error
                }
                Some(var_type) => {
                    if typ != var_type {
                        //
                        // return error
                    }
                }
            }
        }
    }
    println!("Found assignment, creating: {}", creating);
}