use parser::ast::Type;

pub enum PrototypeMatchError{
    NameNotFound,
    NoMatchingPrototype
}
pub fn match_prototype(name: &str, args: &Vec<Type>) -> Result<Type, PrototypeMatchError> {
    match name { 
        "round"=> match_round(args),
        "trunc"=> match_trunc(args),
        "abs"=> match_abs(args),
        "sign"=> match_sign(args),
        "floor"=> match_floor(args),
        "ceil"=> match_ceil(args),
        "fract"=> match_fract(args),
        "radians"=> match_radians(args),
        "degrees"=> match_degrees(args),
        "sin"=> match_sin(args),
        "cos"=> match_cos(args),
        "tan"=> match_tan(args),
        "asin"=> match_asin(args),
        "acos"=> match_acos(args),
        "atan"=> match_atan(args),
        "sinh"=> match_sinh(args),
        "cosh"=> match_cosh(args),
        "tanh"=> match_tanh(args),
        "asinh"=> match_asinh(args),
        "acosh"=> match_acosh(args),
        "atanh"=> match_atanh(args),
        "atan2"=> match_atan2(args),
        "pow"=> match_pow(args),
        "exp"=> match_exp(args),
        "log"=> match_log(args),
        "exp2"=> match_exp2(args),
        "log2"=> match_log2(args),
        "sqrt"=> match_sqrt(args),
        "min"=> match_min(args),
        "max"=> match_max(args),
        "clamp"=> match_clamp(args),
        "dot"=> match_dot(args),
        "length"=> match_length(args),
        "cross"=> match_cross(args),
        "normalize"=> match_normalize(args),
        _=>{
            Result::Err(PrototypeMatchError::NameNotFound)
        }
    }
}
pub fn get_prototypes(name: &str) -> Vec<Vec<Type>> {
    match name { 
        "round"=> prototypes_round(),
        "trunc"=> prototypes_trunc(),
        "abs"=> prototypes_abs(),
        "sign"=> prototypes_sign(),
        "floor"=> prototypes_floor(),
        "ceil"=> prototypes_ceil(),
        "fract"=> prototypes_fract(),
        "radians"=> prototypes_radians(),
        "degrees"=> prototypes_degrees(),
        "sin"=> prototypes_sin(),
        "cos"=> prototypes_cos(),
        "tan"=> prototypes_tan(),
        "asin"=> prototypes_asin(),
        "acos"=> prototypes_acos(),
        "atan"=> prototypes_atan(),
        "sinh"=> prototypes_sinh(),
        "cosh"=> prototypes_cosh(),
        "tanh"=> prototypes_tanh(),
        "asinh"=> prototypes_asinh(),
        "acosh"=> prototypes_acosh(),
        "atanh"=> prototypes_atanh(),
        "atan2"=> prototypes_atan2(),
        "pow"=> prototypes_pow(),
        "exp"=> prototypes_exp(),
        "log"=> prototypes_log(),
        "exp2"=> prototypes_exp2(),
        "log2"=> prototypes_log2(),
        "sqrt"=> prototypes_sqrt(),
        "min"=> prototypes_min(),
        "max"=> prototypes_max(),
        "clamp"=> prototypes_clamp(),
        "dot"=> prototypes_dot(),
        "length"=> prototypes_length(),
        "cross"=> prototypes_cross(),
        "normalize"=> prototypes_normalize(),
        _=>{
            return vec![];
        }
    }
}

const PASS_THROUGH_ROUND: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_round(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ROUND.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_round() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ROUND.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_TRUNC: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_trunc(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_TRUNC.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_trunc() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_TRUNC.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ABS: [Type; 5] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
        Type::Int,
];

fn match_abs(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ABS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_abs() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ABS.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_SIGN: [Type; 5] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
        Type::Int,
];

fn match_sign(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_SIGN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_sign() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_SIGN.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_FLOOR: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_floor(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_FLOOR.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_floor() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_FLOOR.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_CEIL: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_ceil(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_CEIL.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_ceil() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_CEIL.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_FRACT: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_fract(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_FRACT.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_fract() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_FRACT.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_RADIANS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_radians(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_RADIANS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_radians() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_RADIANS.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_DEGREES: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_degrees(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_DEGREES.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_degrees() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_DEGREES.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_SIN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_sin(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_SIN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_sin() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_SIN.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_COS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_cos(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_COS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_cos() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_COS.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_TAN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_tan(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_TAN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_tan() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_TAN.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ASIN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_asin(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ASIN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_asin() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ASIN.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ACOS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_acos(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ACOS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_acos() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ACOS.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ATAN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_atan(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ATAN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_atan() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ATAN.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_SINH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_sinh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_SINH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_sinh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_SINH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_COSH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_cosh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_COSH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_cosh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_COSH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_TANH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_tanh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_TANH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_tanh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_TANH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ASINH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_asinh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ASINH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_asinh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ASINH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ACOSH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_acosh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ACOSH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_acosh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ACOSH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ATANH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_atanh(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ATANH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_atanh() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ATANH.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_ATAN2: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_ATAN2: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(1);

        {
            let p = vec![
                Type::Float,Type::Float,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_atan2(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_ATAN2.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_ATAN2.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_atan2() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_ATAN2.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_ATAN2.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_POW: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_POW: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(4);

        {
            let p = vec![
                Type::Float,Type::Float,
            ];
            m.push((Type::Float, p));
}
        {
            let p = vec![
                Type::Vec2,Type::Float,
            ];
            m.push((Type::Vec2, p));
}
        {
            let p = vec![
                Type::Vec3,Type::Float,
            ];
            m.push((Type::Vec3, p));
}
        {
            let p = vec![
                Type::Vec4,Type::Float,
            ];
            m.push((Type::Vec4, p));
}
        m
    };
}

fn match_pow(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_POW.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_POW.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_pow() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_POW.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_POW.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_EXP: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_exp(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_EXP.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_exp() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_EXP.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_LOG: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_log(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_LOG.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_log() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_LOG.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_EXP2: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_exp2(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_EXP2.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_exp2() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_EXP2.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_LOG2: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_log2(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_LOG2.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_log2() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_LOG2.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_SQRT: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_sqrt(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_SQRT.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_sqrt() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_SQRT.iter().map(|x| vec![*x]);

    tmp.collect()
}

const PASS_THROUGH_MIN: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_MIN: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(2);

        {
            let p = vec![
                Type::Int,Type::Int,
            ];
            m.push((Type::Int, p));
}
        {
            let p = vec![
                Type::Float,Type::Float,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_min(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_MIN.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_MIN.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_min() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_MIN.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_MIN.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_MAX: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_MAX: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(2);

        {
            let p = vec![
                Type::Int,Type::Int,
            ];
            m.push((Type::Int, p));
}
        {
            let p = vec![
                Type::Float,Type::Float,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_max(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_MAX.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_MAX.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_max() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_MAX.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_MAX.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_CLAMP: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_CLAMP: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(2);

        {
            let p = vec![
                Type::Int,Type::Int,Type::Int,
            ];
            m.push((Type::Int, p));
}
        {
            let p = vec![
                Type::Float,Type::Float,Type::Float,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_clamp(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_CLAMP.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_CLAMP.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_clamp() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_CLAMP.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_CLAMP.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_DOT: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_DOT: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(3);

        {
            let p = vec![
                Type::Vec4,Type::Vec4,
            ];
            m.push((Type::Float, p));
}
        {
            let p = vec![
                Type::Vec3,Type::Vec3,
            ];
            m.push((Type::Float, p));
}
        {
            let p = vec![
                Type::Vec2,Type::Vec2,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_dot(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_DOT.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_DOT.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_dot() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_DOT.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_DOT.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_LENGTH: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_LENGTH: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(3);

        {
            let p = vec![
                Type::Vec2,
            ];
            m.push((Type::Float, p));
}
        {
            let p = vec![
                Type::Vec3,
            ];
            m.push((Type::Float, p));
}
        {
            let p = vec![
                Type::Vec4,
            ];
            m.push((Type::Float, p));
}
        m
    };
}

fn match_length(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_LENGTH.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_LENGTH.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_length() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_LENGTH.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_LENGTH.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_CROSS: [Type; 0] = [

];

lazy_static::lazy_static! {
    static ref PROTOTYPES_CROSS: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity(1);

        {
            let p = vec![
                Type::Vec3,Type::Vec3,
            ];
            m.push((Type::Vec3, p));
}
        m
    };
}

fn match_cross(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_CROSS.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    'outer: for proto in PROTOTYPES_CROSS.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_cross() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_CROSS.iter().map(|x| vec![*x]);

    let tmp = tmp.chain(PROTOTYPES_CROSS.iter().map(|x| x.1.clone()));

    tmp.collect()
}

const PASS_THROUGH_NORMALIZE: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];

fn match_normalize(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_NORMALIZE.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }

    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_normalize() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_NORMALIZE.iter().map(|x| vec![*x]);

    tmp.collect()
}
