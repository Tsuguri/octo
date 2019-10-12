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

    for var in program_scope.unused_variables() {
        errs.warnings.push(SemanticWarning::UnusedVariable(var.span, var.name));
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

fn analyze_statement(stat: &mut Statement, diagnostics: &mut Diagnostics, scope: &mut Scope) {
    match stat {
        Statement::Expression(ex) => {
            analyze_expression(ex, diagnostics, scope);
        }
        Statement::Assignment(var, exp, create) => {
            let typ = analyze_expression(exp, diagnostics, scope);
            let name = var.identifier.val.clone();

            // we want to create even if type is invalid. It will prevent cascading errors
            if *create {
                match scope.create_variable(&name, typ, var.identifier.span) {
                    Result::Ok(()) => {}
                    Err(err) => {
                        diagnostics.errors.push(SemanticError::VariableRedefinition(name, err, var.identifier.span));
                    }
                }
                return;
            }

            match typ {
                Type::Unknown => return,
                _ => {}
            }

            let variable = scope.use_variable(&name);
            match variable {
                Some(x) => {
                    if x != typ {
                        diagnostics.errors.push(SemanticError::TypeMismatch(exp.span(), x.to_string(), typ.to_string()));
                    }
                }
                None => {
                    diagnostics.errors.push(SemanticError::UndefinedIdentifier(var.identifier.span, name));
                    return;
                }
            }
        }
        Statement::Return(val) => {
            // TODO: do something here to check returned type
            let typ = analyze_expression(val, diagnostics, scope);
        }
    }
}

fn analyze_binary_operation(left: &mut Expression, right: &mut Expression, scope: &Scope, diagnostics: &mut Diagnostics) -> Type {
    let left_type = analyze_expression(left, diagnostics, scope);
    let right_type = analyze_expression(right, diagnostics, scope);
    if left_type == right_type {
        left_type
    } else {
        if match (left_type.clone(), right_type.clone()) {
            (Type::Unknown, _) => false,
            (_, Type::Unknown) => false,
            (_, _) => true,
        } {
            diagnostics.errors.push(SemanticError::OperationTypeMismatch(left_type.to_string(), left.span(), right_type.to_string(), right.span()));
        }
        Type::Unknown
    }
}

fn analyze_expression(exp: &mut Expression, diagnostics: &mut Diagnostics, scope: &Scope) -> Type {
    use Expression::*;

    match exp {
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
//        Negation(Box < Expression >)=> {},
        Mul(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)
        }
        Div(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)
        }
        Add(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)
        }
        Sub(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics)
        }
        Less(left, right) => {
            // make sure that both sides are of the same type.
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        LessEqual(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        More(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        MoreEqual(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        Equals(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        NotEquals(left, right) => {
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        And(left, right) => {
            let left_type = analyze_expression(left, diagnostics, scope);
            let right_type = analyze_expression(right, diagnostics, scope);
            if left_type != Type::Bool {
                diagnostics.errors.push(SemanticError::LogicTypeMismatch(left_type.to_string(), "&&".to_string(), left.span()));
            }
            if right_type != Type::Bool {
                diagnostics.errors.push(SemanticError::LogicTypeMismatch(right_type.to_string(), "&&".to_string(), right.span()));
            }
            Type::Bool
        }
        Or(left, right) => {
            let left_type = analyze_expression(left, diagnostics, scope);
            let right_type = analyze_expression(right, diagnostics, scope);
            if left_type != Type::Bool {
                diagnostics.errors.push(SemanticError::LogicTypeMismatch(left_type.to_string(), "||".to_string(), left.span()));
            }
            if right_type != Type::Bool {
                diagnostics.errors.push(SemanticError::LogicTypeMismatch(right_type.to_string(), "||".to_string(), right.span()));
            }
            Type::Bool
        }
//        Shift(left, right)=> {},
//        Scale(left, right)=> {},
        _ => Type::Unknown,
    }
}