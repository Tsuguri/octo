pub type Address = usize;

pub type Op = (Address, Operation);

pub use super::ast::Type as ValueType;

pub struct PipelineIR {
    code: Vec<Op>,
    pub inputs: Vec<(ValueType, String)>,
    pub outputs: Vec<ValueType>,
}

impl PipelineIR {
    pub fn new(code: Vec<Op>) -> Self {
        PipelineIR {
            code,
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn with(code: Vec<Op>, prev: &Self) -> Self {
        PipelineIR {
            code,
            inputs: prev.inputs.clone(),
            outputs: prev.outputs.clone(),
        }
    }

    pub fn operations(&self) -> std::slice::Iter<Op> {
        self.code.iter()
    }

    pub fn take(self) -> (Vec<Op>, Vec<(ValueType, String)>, Vec<ValueType>) {
        (self.code, self.inputs, self.outputs)
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

#[derive(Debug, Copy, Clone)]
pub struct PhiRecord {
    pub new: Address,
    pub label: Address,
    pub old: Address,
    pub old_label: Address,
}

#[derive(Debug, Copy, Clone)]
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
    Min(Address, Address),
    Max(Address, Address),
    Clamp(Address, Address, Address),
    Length(Address),
    Cross(Address, Address),
    Normalize(Address),
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Arg(usize),
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
