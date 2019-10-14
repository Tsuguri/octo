use super::ast::Pipeline as IncomingIR;
use super::ast::Pipeline as OutgoingIR;
use errors::{SemanticError, SemanticWarning};
use parser::ast::{Expression, Statement};

use parser::ast::Type;
use super::semantics::env::Scope;


pub struct Diagnostics {
    pub errors: Vec<SemanticError>,
    pub warnings: Vec<SemanticWarning>,
}

impl Diagnostics {
    pub fn err(&mut self, err: SemanticError) -> &mut Self {
        self.errors.push(err);
        self
    }
    pub fn warning(&mut self, warning: SemanticWarning) -> &mut Self {
        self.warnings.push(warning);
        self
    }
}

struct FunctionState {
    returned: bool,
}

pub fn analyze(pip: IncomingIR) -> (Option<OutgoingIR>, Diagnostics) {
    let mut pip = pip;
    let global_scope = Scope::global();

    let mut program_scope = Scope::child_scope(&global_scope);

    let mut errs = Diagnostics { errors: vec![], warnings: vec![] };

    for arg in &pip.arguments {
        program_scope.create_variable(&arg.identifier.val, arg.typ.clone(), arg.identifier.span);
    }

    let state = FunctionState { returned: false };
    for statement in &mut pip.block.statements {
        analyze_statement(statement, &mut errs, &mut program_scope);
    }

    for var in program_scope.unused_variables() {
        errs.warning(SemanticWarning::UnusedVariable(var.span, var.name));
    }

    if errs.errors.len() > 0 {
        (Option::None, errs)
    } else {
        (Option::Some(pip), errs)
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
                        diagnostics.err(SemanticError::VariableRedefinition(name, err, var.identifier.span));
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
                        diagnostics.err(SemanticError::TypeMismatch(exp.span(), x.to_string(), typ.to_string()));
                    }
                }
                None => {
                    diagnostics.err(SemanticError::UndefinedIdentifier(var.identifier.span, name));
                    return;
                }
            }
        }
        Statement::Return(val) => {
            // TODO: do something here to check returned type
            let typ = analyze_expression(val, diagnostics, scope);
        }
        Statement::For(stat, exp1, exp2, block) => {

            let mut block_scope = Scope::child_scope(&scope);

            analyze_statement(&mut *stat, diagnostics, &mut block_scope);

            let cond_type = analyze_expression(&mut *exp1, diagnostics, &mut block_scope);
            match cond_type {
                Type::Bool => {},
                Type::Unknown => {},
                _ => {
                    diagnostics.err(SemanticError::TypeMismatch(exp1.span(), "Bool".to_owned(), cond_type.to_string()));
                }
            }

            analyze_statement(&mut *exp2, diagnostics, &mut block_scope);

            for statement in block.statements.iter_mut() {
                analyze_statement(statement, diagnostics, &mut block_scope);
            }
            // todo
            // analyze shiftness of block. If there is one we must be able to compute const number of loops

        }
        Statement::IfElse(exp, block1, block2) => {
            let exp_type = analyze_expression(&mut *exp, diagnostics, scope);
            match exp_type {
                Type::Bool | Type::Unknown => {},
                _ => {
                    diagnostics.err(SemanticError::TypeMismatch(exp.span(), "Bool".to_owned(), exp_type.to_string()));
                }
            }

            let mut block_scope = Scope::child_scope(&scope);

            for statement in block1.statements.iter_mut() {
                analyze_statement(statement, diagnostics, &mut block_scope);
            }

            drop(block_scope);

            if let Some(else_block) = block2 {
                let mut else_scope = Scope::child_scope(&scope);
                for statement in else_block.statements.iter_mut() {
                    analyze_statement(statement, diagnostics, &mut else_scope);
                }
            }
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
            diagnostics.err(SemanticError::OperationTypeMismatch(left_type.to_string(), left.span(), right_type.to_string(), right.span()));
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
                    diagnostics.err(SemanticError::UndefinedIdentifier(variable.identifier.span, variable.identifier.val.clone()));
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
        Negation(exp) => {
            analyze_expression(exp, diagnostics, scope)
        }
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
                diagnostics.err(SemanticError::LogicTypeMismatch(left_type.to_string(), "and".to_string(), left.span()));
            }
            if right_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(right_type.to_string(), "and".to_string(), right.span()));
            }
            Type::Bool
        }
        Or(left, right) => {
            let left_type = analyze_expression(left, diagnostics, scope);
            let right_type = analyze_expression(right, diagnostics, scope);
            if left_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(left_type.to_string(), "or".to_string(), left.span()));
            }
            if right_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(right_type.to_string(), "or".to_string(), right.span()));
            }
            Type::Bool
        }
        Shift(val, vec) => {
            let value_type = analyze_expression(val, diagnostics, scope);
            let vec_type = analyze_expression(vec, diagnostics, scope);
            if vec_type != Type::Vec2 {
                match vec_type {
                    Type::Unknown => {}
                    _ => {
                        diagnostics.err(SemanticError::TypeMismatch(vec.span(), "Vec2".to_owned(), vec_type.to_string()));
                    }
                };
            }
            vec_type
        }
//        Scale(left, right)=> {},
        _ => Type::Unknown,
    }
}