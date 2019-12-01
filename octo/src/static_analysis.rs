use super::ast::Pipeline as IncomingIR;
use super::ast::Pipeline as OutgoingIR;
use errors::{SemanticError, SemanticWarning, Sp};
use parser::ast::{Expression, Statement, Spanned};

use super::semantics::env::Scope;
use parser::ast::Type;
use lazy_static::lazy_static;

use std::collections::HashMap;

use super::prototypes::{match_prototype, PrototypeMatchError, get_prototypes};

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

pub fn analyze(pip: IncomingIR) -> (Option<OutgoingIR>, Diagnostics) {
    let mut pip = pip;
    let global_scope = Scope::global();

    let mut program_scope = Scope::child_scope(&global_scope);

    let mut errs = Diagnostics {
        errors: vec![],
        warnings: vec![],
    };

    // analyze argument and result types here...
    for arg in &pip.arguments {
        program_scope
            .create_variable(&arg.identifier.val, arg.typ.clone(), arg.identifier.span)
            .unwrap();
    }

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
                        diagnostics.err(SemanticError::VariableRedefinition(
                            name,
                            err,
                            var.identifier.span,
                        ));
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
                        diagnostics.err(SemanticError::TypeMismatch(
                            exp.span(),
                            x.to_string(),
                            typ.to_string(),
                        ));
                    }
                }
                None => {
                    diagnostics.err(SemanticError::UndefinedIdentifier(
                        var.identifier.span,
                        name,
                    ));
                    return;
                }
            }
        }
        Statement::Return(val) => {
            // TODO: do something here to check returned type
            let _typ = analyze_expression(val, diagnostics, scope);
        }
        Statement::For(stat, exp1, exp2, block) => {
            let mut block_scope = Scope::child_scope(&scope);

            analyze_statement(&mut *stat, diagnostics, &mut block_scope);

            let cond_type = analyze_expression(&mut *exp1, diagnostics, &mut block_scope);
            match cond_type {
                Type::Bool => {}
                Type::Unknown => {}
                _ => {
                    diagnostics.err(SemanticError::TypeMismatch(
                        exp1.span(),
                        "Bool".to_owned(),
                        cond_type.to_string(),
                    ));
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
                Type::Bool | Type::Unknown => {}
                _ => {
                    diagnostics.err(SemanticError::TypeMismatch(
                        exp.span(),
                        "Bool".to_owned(),
                        exp_type.to_string(),
                    ));
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

fn analyze_binary_operation(
    left: &mut Expression,
    right: &mut Expression,
    scope: &Scope,
    diagnostics: &mut Diagnostics,
) -> Type {
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
            diagnostics.err(SemanticError::OperationTypeMismatch(
                left_type.to_string(),
                left.span(),
                right_type.to_string(),
                right.span(),
            ));
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
                    diagnostics.err(SemanticError::UndefinedIdentifier(
                        variable.identifier.span,
                        variable.identifier.val.clone(),
                    ));
                    Type::Unknown
                }
                Some(t) => t,
            }
        }
        Literal(lit) => match lit {
            super::ast::Literal::Float(_) => Type::Float,
            super::ast::Literal::Int(_) => Type::Int,
        },
        Negation(exp) => analyze_expression(exp, diagnostics, scope),
        Mul(left, right) => analyze_binary_operation(left, right, scope, diagnostics),
        Div(left, right) => analyze_binary_operation(left, right, scope, diagnostics),
        Add(left, right) => analyze_binary_operation(left, right, scope, diagnostics),
        Sub(left, right) => analyze_binary_operation(left, right, scope, diagnostics),
        Less(left, right) => {
            // TODO add check that types can be ordered (float/int, not vecs)
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        LessEqual(left, right) => {
            // TODO add check that types can be ordered (float/int, not vecs)
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        More(left, right) => {
            // TODO add check that types can be ordered (float/int, not vecs)
            analyze_binary_operation(left, right, scope, diagnostics);
            Type::Bool
        }
        MoreEqual(left, right) => {
            // TODO add check that types can be ordered (float/int, not vecs)
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
                diagnostics.err(SemanticError::LogicTypeMismatch(
                    left_type.to_string(),
                    "and".to_string(),
                    left.span(),
                ));
            }
            if right_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(
                    right_type.to_string(),
                    "and".to_string(),
                    right.span(),
                ));
            }
            Type::Bool
        }
        Or(left, right) => {
            let left_type = analyze_expression(left, diagnostics, scope);
            let right_type = analyze_expression(right, diagnostics, scope);
            if left_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(
                    left_type.to_string(),
                    "or".to_string(),
                    left.span(),
                ));
            }
            if right_type != Type::Bool {
                diagnostics.err(SemanticError::LogicTypeMismatch(
                    right_type.to_string(),
                    "or".to_string(),
                    right.span(),
                ));
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
                        diagnostics.err(SemanticError::TypeMismatch(
                            vec.span(),
                            "Vec2".to_owned(),
                            vec_type.to_string(),
                        ));
                    }
                };
            }
            value_type
        }
        Scale(_left, _right)=> {Type::Unknown},
        Invocation(name, args) => {

            analyze_invocation(&name.val, name.span, args, diagnostics, scope)
        }
        Access(val, field) => {
            analyze_access(val, field, diagnostics, scope)
        }
    }
}

fn analyze_access(exp: &mut Box<Expression>, field: &mut Spanned<String>, diagnostics: &mut Diagnostics, scope: &Scope) -> Type{
    let value_type = analyze_expression(exp, diagnostics, scope);
    //let name = field.val.clone();

    let ret = match value_type {
        Type::Unknown => Type::Unknown,
        Type::Float =>{
            // float has no fields
            Type::Unknown
        },
        Type::Int => {
            // int has no fields
            Type::Unknown
        }
        Type::Bool => {
            Type::Unknown
        }
        Type::Vec2 => {
            match field.val.as_str() {
                "x"=> Type::Float,
                "y"=> Type::Float,
                "r" => Type::Float,
                "g" => Type::Float,
                "u" => Type::Float,
                "v" => Type::Float,
                _=> Type::Unknown
            }
        }
        Type::Vec3 => {
            match field.val.as_str() {
                "x"=> Type::Float,
                "y"=> Type::Float,
                "z"=> Type::Float,
                "r" => Type::Float,
                "g" => Type::Float,
                "b" => Type::Float,
                "xy"=>Type::Vec2,
                "yx"=>Type::Vec2,
                "xz"=>Type::Vec2,
                "zx"=>Type::Vec2,
                "yz"=>Type::Vec2,
                "zy"=>Type::Vec2,
                "rg"=>Type::Vec2,
                "gr"=>Type::Vec2,
                "rb"=>Type::Vec2,
                "br"=>Type::Vec2,
                "gb"=>Type::Vec2,
                "bg"=>Type::Vec2,
                _=> Type::Unknown
            }
        }
        Type::Vec4 => {
            match field.val.as_str() {
                "x"=> Type::Float,
                "y"=> Type::Float,
                "z"=> Type::Float,
                "w"=> Type::Float,
                "r" => Type::Float,
                "g" => Type::Float,
                "b" => Type::Float,
                "a" => Type::Float,
                "xy"=>Type::Vec2,
                "yx"=>Type::Vec2,
                "xz"=>Type::Vec2,
                "zx"=>Type::Vec2,
                "yz"=>Type::Vec2,
                "zy"=>Type::Vec2,
                "rg"=>Type::Vec2,
                "gr"=>Type::Vec2,
                "rb"=>Type::Vec2,
                "br"=>Type::Vec2,
                "gb"=>Type::Vec2,
                "bg"=>Type::Vec2,
                "xyz"=>Type::Vec3,
                "rgb"=>Type::Vec3,
                _=> Type::Unknown
            }
        }
        Type::Void => {
            Type::Unknown
        }
    };
    match ret {
        Type::Unknown  if value_type != Type::Unknown => {
            diagnostics.err(SemanticError::NoField(
                value_type.to_string(),
                exp.span(),
                field.val.clone()
            ));
        }
        _=>{}
    };
    ret
}

lazy_static::lazy_static! {
    static ref TYPE_SET: std::collections::HashSet<&'static str> = {
        let mut m = std::collections::HashSet::new();
        m.insert("vec2");
        m.insert("vec3");
        m.insert("vec4");
        m
    };
}
 
fn match_constructor(name: &str, name_span: Sp, args: &Vec<Type>, diagnostics: &mut Diagnostics, scope: &Scope)-> Type {
    match name {
        "vec2"=> {
            if args.len() ==2 && args[0] == Type::Float && args[1] ==Type::Float {
                Type::Vec2
            } else {
                // error
                Type::Unknown
            }
        },
        "vec3"=> {
            if args.len() ==3 && args[0] == Type::Float && args[1] ==Type::Float && args[2] == Type::Float {
                Type::Vec3
            } else {
                // error
                Type::Unknown
            }
        },
        "vec4" => {
            if args.len() ==3 && args[0] == Type::Float && args[1] ==Type::Float && args[2] == Type::Float && args[3] == Type::Float {
                Type::Vec4
            } else {
                // error
                Type::Unknown
            }
        }
        _=>{unreachable!()}
    }

}
fn analyze_invocation(name: &str, name_span: Sp, args: &mut Vec<Box<Expression>>, diagnostics: &mut Diagnostics, scope: &Scope)-> Type {

    let mut types = Vec::with_capacity(args.len());
    for arg in args {
        let typ = analyze_expression(arg, diagnostics, scope);
        types.push(typ);
    }
    if TYPE_SET.contains(name){
        println!("lolz");
        return match_constructor(name, name_span, &types, diagnostics, scope);
    }
    match match_prototype(name, &types) {
        Ok(val) => return val,
        Err(e) => {
            match e {
                PrototypeMatchError::NameNotFound =>{
                    diagnostics.err(SemanticError::UnknownFunction(
                        name.to_owned(),
                        name_span
                    ));

                },
                PrototypeMatchError::NoMatchingPrototype =>{
                    diagnostics.err(SemanticError::ArgumentsMismatch(
                        name.to_owned(),
                        name_span,
                        get_prototypes(name).iter().map(|x| x.iter().map(|y| y.to_string()).collect()).collect()
                    ));

                    //emit error
                }
            }
        }
    }

    // check user functions here
    Type::Unknown
}