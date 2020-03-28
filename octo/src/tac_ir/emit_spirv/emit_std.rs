use super::super::ir::*;
use super::*;
use spirv_headers::Word as SpirvAddress;
use parser::ast::Type;

pub fn emit_std_function<'a, I: std::iter::Iterator<Item=&'a Op>>(func: StdFunction, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress
{
    use StdFunction::*;
    return match func { 
        Round(data_0,) => emit_round(data_0, ret_addr, emitter),
        Trunc(data_0,) => emit_trunc(data_0, ret_addr, emitter),
        Abs(data_0,) => emit_abs(data_0, ret_addr, emitter),
        Sign(data_0,) => emit_sign(data_0, ret_addr, emitter),
        Floor(data_0,) => emit_floor(data_0, ret_addr, emitter),
        Ceil(data_0,) => emit_ceil(data_0, ret_addr, emitter),
        Fract(data_0,) => emit_fract(data_0, ret_addr, emitter),
        Radians(data_0,) => emit_radians(data_0, ret_addr, emitter),
        Degrees(data_0,) => emit_degrees(data_0, ret_addr, emitter),
        Sin(data_0,) => emit_sin(data_0, ret_addr, emitter),
        Cos(data_0,) => emit_cos(data_0, ret_addr, emitter),
        Tan(data_0,) => emit_tan(data_0, ret_addr, emitter),
        Asin(data_0,) => emit_asin(data_0, ret_addr, emitter),
        Acos(data_0,) => emit_acos(data_0, ret_addr, emitter),
        Atan(data_0,) => emit_atan(data_0, ret_addr, emitter),
        Sinh(data_0,) => emit_sinh(data_0, ret_addr, emitter),
        Cosh(data_0,) => emit_cosh(data_0, ret_addr, emitter),
        Tanh(data_0,) => emit_tanh(data_0, ret_addr, emitter),
        Asinh(data_0,) => emit_asinh(data_0, ret_addr, emitter),
        Acosh(data_0,) => emit_acosh(data_0, ret_addr, emitter),
        Atanh(data_0,) => emit_atanh(data_0, ret_addr, emitter),
        Atan2(data_0,data_1,) => emit_atan2(data_0,data_1, ret_addr, emitter),
        Pow(data_0,data_1,) => emit_pow(data_0,data_1, ret_addr, emitter),
        Exp(data_0,) => emit_exp(data_0, ret_addr, emitter),
        Log(data_0,) => emit_log(data_0, ret_addr, emitter),
        Exp2(data_0,) => emit_exp2(data_0, ret_addr, emitter),
        Log2(data_0,) => emit_log2(data_0, ret_addr, emitter),
        Sqrt(data_0,) => emit_sqrt(data_0, ret_addr, emitter),
        Min(data_0,data_1,) => emit_min(data_0,data_1, ret_addr, emitter),
        Max(data_0,data_1,) => emit_max(data_0,data_1, ret_addr, emitter),
        Clamp(data_0,data_1,data_2,) => emit_clamp(data_0,data_1,data_2, ret_addr, emitter),
        Dot(data_0,data_1,) => emit_dot(data_0,data_1, ret_addr, emitter),
        Length(data_0,) => emit_length(data_0, ret_addr, emitter),
        Cross(data_0,data_1,) => emit_cross(data_0,data_1, ret_addr, emitter),
        Normalize(data_0,) => emit_normalize(data_0, ret_addr, emitter),
    };
}



const PASS_THROUGH_ROUND: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_round<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting round of {:?}", data_0_type);


    
    let id = 
        
            1;
    
    if PASS_THROUGH_ROUND.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_TRUNC: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_trunc<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting trunc of {:?}", data_0_type);


    
    let id = 
        
            3;
    
    if PASS_THROUGH_TRUNC.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ABS: [Type; 5] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
        Type::Int,
];



fn emit_abs<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting abs of {:?}", data_0_type);


    
    let id = 
        
            if data_0_type == ValueType::Int {
                5
            } else {
                4
            };
    
    if PASS_THROUGH_ABS.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_SIGN: [Type; 5] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
        Type::Int,
];



fn emit_sign<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting sign of {:?}", data_0_type);


    
    let id = 
        
            if data_0_type == ValueType::Int {
                7
            } else {
                6
            };
    
    if PASS_THROUGH_SIGN.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_FLOOR: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_floor<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting floor of {:?}", data_0_type);


    
    let id = 
        
            8;
    
    if PASS_THROUGH_FLOOR.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_CEIL: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_ceil<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting ceil of {:?}", data_0_type);


    
    let id = 
        
            9;
    
    if PASS_THROUGH_CEIL.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_FRACT: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_fract<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting fract of {:?}", data_0_type);


    
    let id = 
        
            10;
    
    if PASS_THROUGH_FRACT.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_RADIANS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_radians<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting radians of {:?}", data_0_type);


    
    let id = 
        
            11;
    
    if PASS_THROUGH_RADIANS.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_DEGREES: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_degrees<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting degrees of {:?}", data_0_type);


    
    let id = 
        
            12;
    
    if PASS_THROUGH_DEGREES.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_SIN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_sin<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting sin of {:?}", data_0_type);


    
    let id = 
        
            13;
    
    if PASS_THROUGH_SIN.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_COS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_cos<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting cos of {:?}", data_0_type);


    
    let id = 
        
            14;
    
    if PASS_THROUGH_COS.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_TAN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_tan<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting tan of {:?}", data_0_type);


    
    let id = 
        
            15;
    
    if PASS_THROUGH_TAN.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ASIN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_asin<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting asin of {:?}", data_0_type);


    
    let id = 
        
            16;
    
    if PASS_THROUGH_ASIN.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ACOS: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_acos<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting acos of {:?}", data_0_type);


    
    let id = 
        
            17;
    
    if PASS_THROUGH_ACOS.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ATAN: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_atan<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting atan of {:?}", data_0_type);


    
    let id = 
        
            18;
    
    if PASS_THROUGH_ATAN.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_SINH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_sinh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting sinh of {:?}", data_0_type);


    
    let id = 
        
            19;
    
    if PASS_THROUGH_SINH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_COSH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_cosh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting cosh of {:?}", data_0_type);


    
    let id = 
        
            20;
    
    if PASS_THROUGH_COSH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_TANH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_tanh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting tanh of {:?}", data_0_type);


    
    let id = 
        
            21;
    
    if PASS_THROUGH_TANH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ASINH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_asinh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting asinh of {:?}", data_0_type);


    
    let id = 
        
            22;
    
    if PASS_THROUGH_ASINH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ACOSH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_acosh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting acosh of {:?}", data_0_type);


    
    let id = 
        
            23;
    
    if PASS_THROUGH_ACOSH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_ATANH: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_atanh<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting atanh of {:?}", data_0_type);


    
    let id = 
        
            24;
    
    if PASS_THROUGH_ATANH.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}



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


fn emit_atan2<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting atan2 of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_ATAN2.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            25;
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_pow<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting pow of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_POW.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            26;
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}


const PASS_THROUGH_EXP: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_exp<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting exp of {:?}", data_0_type);


    
    let id = 
        
            27;
    
    if PASS_THROUGH_EXP.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_LOG: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_log<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting log of {:?}", data_0_type);


    
    let id = 
        
            28;
    
    if PASS_THROUGH_LOG.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_EXP2: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_exp2<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting exp2 of {:?}", data_0_type);


    
    let id = 
        
            29;
    
    if PASS_THROUGH_EXP2.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_LOG2: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_log2<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting log2 of {:?}", data_0_type);


    
    let id = 
        
            30;
    
    if PASS_THROUGH_LOG2.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}


const PASS_THROUGH_SQRT: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_sqrt<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting sqrt of {:?}", data_0_type);


    
    let id = 
        
            31;
    
    if PASS_THROUGH_SQRT.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}



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


fn emit_min<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting min of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_MIN.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            if result_type == ValueType::Int {
                39
            } else {
                37
            };
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_max<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting max of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_MAX.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            if result_type == ValueType::Int {
                42
            } else {
                40
            };
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_clamp<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address,data_2: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    let data_2_type = emitter.get_single_type(data_2);
    
    let args =[data_0_type, data_1_type, data_2_type,  ];
    println!("emitting clamp of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_CLAMP.iter() {
        if proto.1.len() != 3 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            if result_type == ValueType::Int {
                45
            } else {
                43
            };
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,data_2,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_dot<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting dot of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_DOT.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            0;
        
        
        return emitter.emit_dot_instruction(data_0, data_1, ret_addr);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_length<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting length of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_LENGTH.iter() {
        if proto.1.len() != 1 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            66;
        
        
        return emitter.emit_prototyped(id, &[data_0,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}



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


fn emit_cross<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address,data_1: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    let data_1_type = emitter.get_single_type(data_1);
    
    let args =[data_0_type, data_1_type,  ];
    println!("emitting cross of {:?}", data_0_type);


    
    
    for proto in PROTOTYPES_CROSS.iter() {
        if proto.1.len() != 2 {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = 
            
            if result_type == ValueType::Int {
                45
            } else {
                43
            };
        
        
        return emitter.emit_prototyped(id, &[data_0,data_1,],ret_addr, result_type);
        
    }
    panic!();
    
    return 0;
}


const PASS_THROUGH_NORMALIZE: [Type; 4] = [

        Type::Float,
        Type::Vec2,
        Type::Vec3,
        Type::Vec4,
];



fn emit_normalize<'a, I: std::iter::Iterator<Item=&'a Op>>(data_0: Address, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    
    let data_0_type = emitter.get_single_type(data_0);
    
    let args =[data_0_type,  ];
    println!("emitting normalize of {:?}", data_0_type);


    
    let id = 
        
            69;
    
    if PASS_THROUGH_NORMALIZE.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    
    
    return 0;
}
