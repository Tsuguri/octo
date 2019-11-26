use std::collections::HashMap;
use parser::ast::Type;

pub enum PrototypeMatchError{
    NameNotFound,
    NoMatchingPrototype
}
pub fn match_prototype(name: &str, args: &Vec<Type>) -> Result<Type, PrototypeMatchError> {
    match name { 
        "sin"=> match_sin(args),
        "cos"=> match_cos(args),
        _=>{
            Result::Err(PrototypeMatchError::NameNotFound)
        }
    }
}


const PASS_THROUGH_SIN: [Type; 4] = [
    
        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];
// ignore normal prototypes for now xD
fn match_sin(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_SIN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }
    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}

const PASS_THROUGH_COS: [Type; 4] = [
    
        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];
// ignore normal prototypes for now xD
fn match_cos(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_COS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }
    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
