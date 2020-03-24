pub type Address = usize;

pub type Op = (Address, Operation);

pub use super::ast::Type as ValueType;

pub struct PipelineIR {
    code: Vec<Op>,
    pub inputs: Vec<(ValueType, String)>,
    pub outputs: Vec<ValueType>,
    pub uniforms: Vec<(ValueType, String)>,
}

impl PipelineIR {
    pub fn new(code: Vec<Op>) -> Self {
        PipelineIR {
            code,
            inputs: vec![],
            outputs: vec![],
            uniforms: vec![],
        }
    }

    pub fn with(code: Vec<Op>, prev: &Self) -> Self {
        PipelineIR {
            code,
            inputs: prev.inputs.clone(),
            outputs: prev.outputs.clone(),
            uniforms: prev.uniforms.clone(),
        }
    }
    pub fn construct(code: Vec<Op>, inputs: Vec<(ValueType, String)>, outputs: Vec<ValueType>, uniforms: Vec<(ValueType, String)>) -> Self {
        PipelineIR {
            code,
            inputs,
            outputs,
            uniforms,
        }
    }

    pub fn operations(&self) -> std::slice::Iter<Op> {
        self.code.iter()
    }

    pub fn take(self) -> (Vec<Op>, Vec<(ValueType, String)>, Vec<ValueType>, Vec<(ValueType, String)>) {
        (self.code, self.inputs, self.outputs, self.uniforms)
    }
}

impl std::fmt::Debug for PipelineIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op in &self.code {
            writeln!(f, "{} = {:?}", op.0, op.1)?;
        }
        Result::Ok(())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Vec4([f64; 4]),
    Bool(bool),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PhiRecord {
    pub new: Address,
    pub label: Address,
    pub old: Address,
    pub old_label: Address,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StdFunction {
    Round(Address),
    Trunc(Address),
    Abs(Address),
    Sign(Address),
    Floor(Address),
    Ceil(Address),
    Fract(Address),
    Radians(Address),
    Degrees(Address),
    Sin(Address),
    Cos(Address),
    Tan(Address),
    Asin(Address),
    Acos(Address),
    Atan(Address),
    Sinh(Address),
    Cosh(Address),
    Tanh(Address),
    Asinh(Address),
    Acosh(Address),
    Atanh(Address),
    Atan2(Address, Address),
    Pow(Address, Address),
    Exp(Address),
    Log(Address),
    Exp2(Address),
    Log2(Address),
    Sqrt(Address),
    Dot(Address, Address),
    Min(Address, Address),
    Max(Address, Address),
    Clamp(Address, Address, Address),
    Length(Address),
    Cross(Address, Address),
    Normalize(Address),
}

impl StdFunction {
    pub fn deps(&self) -> Vec<Address> {
        use StdFunction::*;
        match *self {
            Round(a) => vec![a],
            Trunc(a) => vec![a],
            Abs(a) => vec![a],
            Sign(a) => vec![a],
            Floor(a) => vec![a],
            Ceil(a) => vec![a],
            Fract(a) => vec![a],
            Radians(a) => vec![a],
            Degrees(a) => vec![a],
            Sin(a) => vec![a],
            Cos(a) => vec![a],
            Tan(a) => vec![a],
            Asin(a) => vec![a],
            Acos(a) => vec![a],
            Atan(a) => vec![a],
            Sinh(a) => vec![a],
            Cosh(a) => vec![a],
            Tanh(a) => vec![a],
            Asinh(a) => vec![a],
            Acosh(a) => vec![a],
            Atanh(a) => vec![a],
            Atan2(a,b) => vec![a,b],
            Pow(a,b) => vec![a,b],
            Exp(a) => vec![a],
            Log(a) => vec![a],
            Exp2(a) => vec![a],
            Log2(a) => vec![a],
            Sqrt(a) => vec![a],
            Dot(a,b) => vec![a,b],
            Min(a,b) => vec![a,b],
            Max(a,b) => vec![a,b],
            Clamp(a,b,c) => vec![a,b,c],
            Length(a) => vec![a],
            Cross(a,b) => vec![a,b],
            Normalize(a) => vec![a],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Arg(usize),
    Uniform(usize),
    StoreInt(i64),
    StoreFloat(f64),
    StoreVec2([f64; 2]),
    StoreVec3([f64; 3]),
    StoreVec4([f64; 4]),
    StoreBool(bool),
    Store(Address),
    ConstructVec2(Address, Address),
    ConstructVec3(Address, Address, Address),
    ConstructVec4(Address, Address, Address, Address),
    ExtractComponent(Address, usize),
    StoreComponent(Address, usize, Address),
    Add(Address, Address),
    Sub(Address, Address),
    Mul(Address, Address),
    Div(Address, Address),
    Less(Address, Address),
    LessEq(Address, Address),
    Eq(Address, Address),
    Neq(Address, Address),
    And(Address, Address),
    Or(Address, Address),

    Neg(Address),

    Shift(Address, Address),
    Exit(Address, Address),
    Sync(Address),

    Invoke(StdFunction),

    Phi(PhiRecord),
    JumpIfElse(Address, Address, Address),
    Jump(Address),
    LoopMerge(Address, Address), // cond label, exit label
    Label,
}

impl std::string::ToString for Operation {
    fn to_string(&self) -> String {
        use Operation::*;

        match *self {
            Arg(i) => format!("Arg({})", i),
            Uniform(i) => format!("Uniform({})", i),
            StoreInt(i) => format!("int ({})", i),
            StoreFloat(i) => format!("float({})", i),
            StoreBool(i) => format!("Bool({})", i),
            StoreVec2(i) => format!("Vec2({}, {})", i[0], i[1]),
            StoreVec3(i) => format!("Vec3({}, {}, {})", i[0], i[1], i[2]),
            StoreVec4(i) => format!("Vec4({}, {}, {}, {})", i[0], i[1], i[2], i[3]),
            ConstructVec2(..) => "ConstructVec2".to_string(),
            ConstructVec3(..) => "ConstructVec3".to_string(),
            ConstructVec4(..) => "ConstructVec4".to_string(),
            ExtractComponent(..) => "ExtractComponent".to_string(),
            StoreComponent(..) => "StoreComponent".to_string(),
            Store(..) => "Store".to_string(),
            Add(..) => "Add".to_string(),
            Sub(..) => "Sub".to_string(),
            Mul(..) => "Mul".to_string(),
            Div(..) => "Div".to_string(),
            Less(..) => "Less".to_string(),
            LessEq(..) => "LessEq".to_string(),
            Eq(..) => "Eq".to_string(),
            Neq(..) => "Neq".to_string(),
            And(..) => "And".to_string(),
            Or(..) => "Or".to_string(),
            Shift(..) => "Shift".to_string(),
            Phi(..) => "Phi".to_string(),
            Neg(..) => "Neg".to_string(),
            Exit(..) => "Exit".to_string(),
            Sync(..) => "Sync".to_string(),
            Invoke(..) => "Std()".to_string(),
            JumpIfElse(..) => "JumpIfElse".to_string(),
            Jump(..) => "Jump".to_string(),
            LoopMerge(..) => "LoopMerge".to_string(),
            Label => "Label".to_string(),
        }
    }
}

macro_rules! replace {
    ($i1:ident, $i2:ident, $i3:ident) => {
        if *$i1 == $i2 {
            *$i1 = $i3
        }
    };
}

pub fn replace(op: &mut (Address, Operation), from: Address, to: Address, replace_result: bool) {
    if replace_result {
        op.0 = if op.0 == from { to } else {op.0};
    }

    match &mut op.1 {
        Operation::Arg(..)=>(),
        Operation::Uniform(..)=>(),
        Operation::StoreInt(..)=>(),
        Operation::StoreFloat(..)=>(),
        Operation::StoreVec2(..)=>(),
        Operation::StoreVec3(..)=>(),
        Operation::StoreVec4(..)=>(),
        Operation::StoreBool(..)=>(),
        Operation::Label => (),
        Operation::ConstructVec2(a, b) => {
            replace!(a, from, to);
            replace!(b, from, to);
        }
        Operation::ConstructVec3(a, b, c) => {
            replace!(a, from, to);
            replace!(b, from, to);
            replace!(c, from, to);
        }
        Operation::ConstructVec4(a, b, c, d) => {
            replace!(a, from, to);
            replace!(b, from, to);
            replace!(c, from, to);
            replace!(d, from, to);
        }
        Operation::ExtractComponent(a, b) => {
            replace!(a, from, to);
        }
        Operation::StoreComponent(vec, id, float) => {
            replace!(vec, from, to);
            replace!(float, from, to);
        }
        Operation::Invoke(function) => {
            // ignore for now,
        }
        Operation::LoopMerge(a, b) => {
            replace!(a, from, to);
            replace!(b, from, to);
        }
        Operation::Add(l, r) => {
            replace!(l, from, to);
            replace!(r, from, to);
        }
        Operation::Sub(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Mul(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Div(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Less(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::LessEq(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Eq(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Neq(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::And(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Or(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Shift(l, r)=>{
            replace!(l, from, to);
            replace!(r, from, to);
        },
        Operation::Phi(l) => {
            let left = &mut l.new;
            let right = &mut l.old;

            replace!(left, from, to);
            replace!(right, from, to);
        }
        Operation::Jump(a) => {
            replace!(a, from, to)
        },
        Operation::Neg(a) => {
            replace!(a, from, to)
        },
        Operation::Exit(a, b) => {
            //println!("replacing in exit: {} into {} with a:{} and b:{}", from, to, a, b);
            replace!(a, from, to);
            replace!(b, from, to);
        },
        Operation::Store(a) => {
            replace!(a, from, to)
        },
        Operation::Sync(a) => {
            replace!(a, from, to)
        },
        Operation::JumpIfElse(a, b, c) => {
            replace!(a, from, to);
            replace!(b, from, to);
            replace!(c, from, to);
        },

        //_ => (),
    }

}

