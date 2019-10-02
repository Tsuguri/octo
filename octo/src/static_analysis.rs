use super::ast::Program as IncomingIR;
use super::ast::Program as OutgoingIR;
use errors::{SemanticError, SemanticWarning};
use parser::ast::{Expression, Statement};

use parser::ast::Type;
use super::semantics::env::Scope;


pub struct Diagnostics {
    pub errors: Vec<SemanticError>,
    pub warnings: Vec<SemanticWarning>,
}

struct FunctionState {
    returned: bool,
}

pub fn analyze(program: IncomingIR) -> (Option<OutgoingIR>, Diagnostics) {
    let mut program = program;
    let global_scope = Scope::global();

    let mut program_scope = Scope::child_scope(&global_scope);

    let mut errs = Diagnostics { errors: vec![], warnings: vec![] };

    let pip = &mut program.pipeline;
    for arg in &pip.arguments {
        program_scope.create_variable(&arg.identifier.val, arg.typ.clone(), arg.identifier.span);
    }

    let state = FunctionState { returned: false };
    for statement in &mut pip.block.statements {
        analyze_statement(statement, &mut errs, &mut program_scope);
    }

    if errs.errors.len() > 0 {
        (Option::None, errs)
    } else {
        (Option::Some(program), errs)
    }

    // check names of variables
    // check types of variables

    //analyze types
}

fn analyze_statement(stat: &mut Statement, diagnostics: &mut Diagnostics, scope: &mut Scope) -> Result<(), ()> {
    match stat {
        Statement::Expression(ex) => {
            analyze_expression(ex, diagnostics, scope)?;
        }
        Statement::Assignment(var, exp, create) => {
            let typ = analyze_expression(exp, diagnostics, scope)?;
            let name = var.identifier.val.clone();
            if *create {
                match scope.create_variable(&name, typ, var.identifier.span) {
                    Result::Ok(()) => {}
                    Err(err) => {
                        diagnostics.errors.push(SemanticError::VariableRedefinition(name, err, var.identifier.span));
                    }
                }
            } else {
                let variable = scope.use_variable(&name);
                match variable {
                    Some(x) => {
                        if x != typ {
                            diagnostics.errors.push(SemanticError::TypeMismatch(exp.span(), x.to_string(), typ.to_string()));
                        }
                    }
                    None => {
                        diagnostics.errors.push(SemanticError::UndefinedIdentifier(var.identifier.span, name));
                        return Result::Err(());
                    }
                }
            }
        }
        Statement::Return(val) => {}
    }

    Result::Ok(())
}

fn analyze_binary_operation(left: &mut Expression, right: &mut Expression, scope: &Scope, diagnostics: &mut Diagnostics) -> Result::<Type,()> {
    let left_type = analyze_expression(left, diagnostics, scope)?;
    let right_type = analyze_expression(right, diagnostics, scope)?;
    if left_type == right_type {
        Result::Ok(left_type)
    } else {
        if match (left_type.clone(), right_type.clone()) {
            (Type::Unknown, _) => false,
            (_, Type::Unknown) => false,
            (_, _) => true,
        } {
            //diagnostics.errors.push(SemanticError::TypeMismatch(exp.span(), left_type.to_string(), right_type.to_string()));
            diagnostics.errors.push(SemanticError::OperationTypeMismatch(left_type.to_string(), left.span(), right_type.to_string(), right.span()));
        }
        Result::Ok(Type::Unknown)
    }
}

fn analyze_expression(exp: &mut Expression, diagnostics: &mut Diagnostics, scope: &Scope) -> Result<Type, ()> {
    use Expression::*;

    let p = match exp {
        Variable(variable) => {
            let var = scope.use_variable(&variable.identifier.val);
            match var {
                None => {
                    diagnostics.errors.push(SemanticError::UndefinedIdentifier(variable.identifier.span, variable.identifier.val.clone()));
                    Type::Unknown
                }
                Some(t) => t,
            }
        }
        Literal(lit) => {
            match lit {
                super::ast::Literal::Float(_) => Type::Float,
                super::ast::Literal::Int(_) => Type::Int,
            }
        }
//        Literal(Literal)=> {},
//        Negation(Box < Expression >)=> {},
        Mul(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)?
        }
        Div(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)?
        }
        Add(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)?
        }
        Sub(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)?
        }
//        Less(Box < Expression >, Box < Expression >)=> {},
//        LessEqual(Box < Expression >, Box < Expression >)=> {},
//        More(Box < Expression >, Box < Expression >)=> {},
//        MoreEqual(Box < Expression >, Box < Expression >)=> {},
//        Equals(Box < Expression >, Box < Expression >)=> {},
//        NotEquals(Box < Expression >, Box < Expression >)=> {},
//        And(Box < Expression >, Box < Expression >)=> {},
//        Or(Box < Expression >, Box < Expression >)=> {},
//        Shift(Box < Expression >, Box < Expression >)=> {},
//        Scale(Box < Expression >, Box < Expression >)=> {},
        _ => Type::Unknown,
    };

    Result::Ok(p)
}