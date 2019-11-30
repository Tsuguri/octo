use super::ir::{
    Address, 
    ConstantValue, 
    Operation, 
    StdFunction,
};
use super::code::{Code};

#[derive(Debug)]
pub enum BuiltinEmitError {
    NameNotFound,
    CompilerError,
}

pub fn emit_builtin(name: &str, args: &Vec<Address>, code: &mut Code) -> Result<Address, BuiltinEmitError> {
    match name { 
        "round"=> emit_round(args, code),
        "trunc"=> emit_trunc(args, code),
        "abs"=> emit_abs(args, code),
        "sign"=> emit_sign(args, code),
        "floor"=> emit_floor(args, code),
        "ceil"=> emit_ceil(args, code),
        "fract"=> emit_fract(args, code),
        "radians"=> emit_radians(args, code),
        "degrees"=> emit_degrees(args, code),
        "sin"=> emit_sin(args, code),
        "cos"=> emit_cos(args, code),
        "tan"=> emit_tan(args, code),
        "asin"=> emit_asin(args, code),
        "acos"=> emit_acos(args, code),
        "atan"=> emit_atan(args, code),
        "sinh"=> emit_sinh(args, code),
        "cosh"=> emit_cosh(args, code),
        "tanh"=> emit_tanh(args, code),
        "asinh"=> emit_asinh(args, code),
        "acosh"=> emit_acosh(args, code),
        "atanh"=> emit_atanh(args, code),
        "atan2"=> emit_atan2(args, code),
        "pow"=> emit_pow(args, code),
        "exp"=> emit_exp(args, code),
        "log"=> emit_log(args, code),
        "exp2"=> emit_exp2(args, code),
        "log2"=> emit_log2(args, code),
        "sqrt"=> emit_sqrt(args, code),
        "min"=> emit_min(args, code),
        "max"=> emit_max(args, code),
        "clamp"=> emit_clamp(args, code),
        "length"=> emit_length(args, code),
        "cross"=> emit_cross(args, code),
        "normalize"=> emit_normalize(args, code),
        _=>{
            Result::Err(BuiltinEmitError::NameNotFound)
        }
    }
}


// ignore normal prototypes for now xD
fn emit_round(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Round(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_trunc(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Trunc(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_abs(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Abs(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_sign(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Sign(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_floor(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Floor(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_ceil(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Ceil(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_fract(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Fract(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_radians(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Radians(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_degrees(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Degrees(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_sin(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Sin(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_cos(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Cos(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_tan(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Tan(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_asin(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Asin(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_acos(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Acos(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_atan(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Atan(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_sinh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Sinh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_cosh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Cosh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_tanh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Tanh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_asinh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Asinh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_acosh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Acosh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_atanh(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Atanh(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_atan2(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_pow(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_exp(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Exp(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_log(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Log(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_exp2(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Exp2(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_log2(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Log2(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_sqrt(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Sqrt(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_min(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_max(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_clamp(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_length(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Length(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_cross(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    Result::Err(BuiltinEmitError::CompilerError)
}

// ignore normal prototypes for now xD
fn emit_normalize(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::Normalize(args[0]))))
    }
    
    Result::Err(BuiltinEmitError::CompilerError)
}
