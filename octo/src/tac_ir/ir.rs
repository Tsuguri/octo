pub type Address = usize;

pub type Op = (Address, Operation);

pub struct PipelineIR {
    code: Vec<Op>,
}

impl PipelineIR {
    pub fn new(code: Vec<Op>) -> Self {
        PipelineIR {
            code
        }
    }

    pub fn operations(&self) -> std::slice::Iter<Op> {
        self.code.iter()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
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
pub enum Operation {
    Arg(usize),
    StoreInt(i64),
    StoreFloat(f64),
    StoreVec2([f64; 2]),
    StoreVec3([f64; 3]),
    StoreBool(bool),
    Store(Address),
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

    Phi(PhiRecord),
    JumpIfElse(Address, Address, Address),
    Jump(Address),
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
            Store(..)=> "Store".to_string(),
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
            JumpIfElse(..) => "JumpIfElse".to_string(),
            Jump(..) => "Jump".to_string(),
            Label => "Label".to_string(),
        }
    }
}