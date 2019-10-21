pub mod optimalizations;

// tac with SSA
use std::collections::HashMap;
use super::ast;
use ast::Pipeline as IncomingIR;

pub type Address = usize;

pub type Op = (Address, Operation);

pub use optimalizations::*;
use std::ops::Range;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ConstantValue {
    Int(i64),
    Float(f64),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Bool(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Arg(usize),
    StoreInt(i64),
    StoreFloat(f64),
    StoreVec2([f64; 2]),
    StoreVec3([f64; 3]),
    StoreBool(bool),
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
    Exit(Address),

    Phi(Address, Address),
    JumpIfElse(Address, Address, Address),
    Jump(Address),
    Label,
}


type PhiCollection = HashMap<String, (Address, Address)>;

struct Code {
    pub code: Vec<Op>,
    variables: HashMap<String, Address>,
    constants: HashMap<Address, ConstantValue>,

    phi_assignments: Option<PhiCollection>,

    counter: usize,
}


macro_rules! replace {
    ($i1:ident, $i2:ident, $i3:ident) => {if $i1==$i2 {$i3}else{$i1}};
}

macro_rules! replace_two {
    ($id1:path, $left:ident, $right:ident, $old:ident, $new:ident) => {

            $id1(replace!($left, $old, $new), replace!($right, $old, $new))

    };
}

impl Code {
    pub fn new() -> Self {
        Code {
            code: vec![],
            variables: HashMap::new(),
            constants: HashMap::new(),
            counter: 0,
            phi_assignments: None,
        }
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }

    pub fn replace_label(&mut self, range: Range<usize>, old: Address, new: Address) {

        for index in range {
            let operation = self.code[index].1;
            let op = match operation {
                Operation::Add(l, r)=>replace_two!(Operation::Add, l, r, old, new),
                Operation::Sub(l, r)=>replace_two!(Operation::Sub, l, r, old, new),
                Operation::Mul(l, r)=>replace_two!(Operation::Mul, l, r, old, new),
                Operation::Div(l, r)=>replace_two!(Operation::Div, l, r, old, new),
                Operation::Less(l, r)=>replace_two!(Operation::Less, l, r, old, new),
                Operation::LessEq(l, r)=>replace_two!(Operation::LessEq, l, r, old, new),
                Operation::Eq(l, r)=>replace_two!(Operation::Eq, l, r, old, new),
                Operation::Neq(l, r)=>replace_two!(Operation::Neq, l, r, old, new),
                Operation::And(l, r)=>replace_two!(Operation::And, l, r, old, new),
                Operation::Or(l, r)=>replace_two!(Operation::Or, l, r, old, new),
                Operation::Shift(l, r)=>replace_two!(Operation::Shift, l, r, old, new),
                Operation::Phi(l, r)=>replace_two!(Operation::Phi, l, r, old, new),
                Operation::Jump(a) => Operation::Jump(replace!(a, old, new)),
                Operation::Neg(a) => Operation::Neg(replace!(a, old, new)),
                Operation::Exit(a) => Operation::Exit(replace!(a, old, new)),
                Operation::JumpIfElse(a,b,c)=>{
                    Operation::JumpIfElse(replace!(a,old,new),replace!(b,old,new),replace!(c,old,new))
                }
                x => x,

            };
            self.code[index].1 = op;
        }

    }

    pub fn observe_assignments(&mut self) -> Option<PhiCollection> {
        let tmp = self.phi_assignments.take();
        self.phi_assignments = Some(HashMap::new());
        tmp
    }

    pub fn finish_observing(&mut self, old: Option<PhiCollection>) -> Option<PhiCollection> {
        let ret = self.phi_assignments.take();
        self.phi_assignments = old;
        ret
    }

    pub fn new_label(&mut self) -> Address {
        self.counter += 1;
        self.counter
    }
    pub fn push(&mut self, op: Operation) -> Address {
        let lab = self.new_label();
        self.code.push((lab, op));
        lab
    }
    pub fn push_with_label(&mut self, op: Operation, label: Address) {
        self.code.push((label, op));
    }

    pub fn store(&mut self, name: &str, add: Address, create: bool) {
        if let Some(assignments) = &mut self.phi_assignments {
            // if we create new variable then it doesn't go into phi assignemts (local variable).
            if !create {
                let old = self.variables[name];
                assignments.insert(name.to_owned(), (add, old));
                return;
            }
        }
        self.variables.insert(name.to_owned(), add);
    }
    pub fn get(&self, name: &str) -> Address {
        if let Some(assignments) = &self.phi_assignments {
            println!("Getting phi val for {}", name);
            match assignments.get(name) {
                None => {}
                Some(x) => {
                    return x.0;
                }
            }
        };
        *self.variables.get(name).unwrap()
    }

    pub fn get_const_address(&self, value: &ConstantValue) -> Option<Address> {
        self.constants.iter().find(|x| *x.1 == *value).map(|x| *x.0)
    }

    pub fn store_constant(&mut self, val: ConstantValue) -> Address {
        let addr = self.get_const_address(&val);
        match addr {
            Some(x) => return x,
            _ => {}
        }
        use ConstantValue::*;
        let address = match val {
            Float(val) => {
                self.push(Operation::StoreFloat(val))
            }
            Int(val) => {
                self.push(Operation::StoreInt(val))
            }
            Vec2(val) => {
                self.push(Operation::StoreVec2(val))
            }
            Vec3(val) => {
                self.push(Operation::StoreVec3(val))
            }
            Bool(val) => {
                self.push(Operation::StoreBool(val))
            }
        };
        self.make_const(address, val);
        address
    }

    pub fn get_const(&self, addr: Address) -> ConstantValue {
        self.constants[&addr]
    }
    pub fn make_const(&mut self, addr: Address, value: ConstantValue) {
        self.constants.insert(addr, value);
    }

    pub fn is_const(&self, addr: Address) -> bool {
        let res = self.constants.contains_key(&addr);
        res
    }
}


pub fn emit_ir(ast: IncomingIR) -> Vec<Op> {
    let mut code = Code::new();

    for arg in ast.arguments.iter().enumerate() {
        let addr = code.push(Operation::Arg(arg.0));
        code.store(&arg.1.identifier.val, addr, false);
    }

    emit_block(ast.block, &mut code);


    // analyze statements

    code.code
}

fn emit_block(block: ast::Block, code: &mut Code) {
    for statement in block.statements {
        emit_statement(statement, code);
    }
}

fn emit_statement(statement: ast::Statement, code: &mut Code) {
    match statement {
        ast::Statement::Expression(exp) => {
            emit_expression(*exp, code);
        }
        ast::Statement::Return(exp) => {
            let ret_add = emit_expression(*exp, code);
            code.push(Operation::Exit(ret_add));
        }
        ast::Statement::Assignment(var, exp, create) => {
            let addr = emit_expression(*exp, code);
            code.store(&var.identifier.val, addr, create);
        }
        ast::Statement::For(stat, exp1, exp2, block) => {

            // initialization statement
            let initialization = emit_statement(*stat, code);


            let condition_label = code.new_label();
            let content_label = code.new_label();
            let end_label = code.new_label();

            code.push(Operation::Jump(condition_label));

            code.push_with_label(Operation::Label, content_label);
            let old_phi = code.observe_assignments();
            let before_code_size = code.code_size();

            // do loop stuff

            emit_block(block, code);

            // increment

            let increment = emit_statement(*exp2, code);

            let phi_assignments = code.finish_observing(old_phi);
            assert!(phi_assignments.is_some());

            let after_code_size = code.code_size();

            code.push(Operation::Jump(condition_label));

            code.push_with_label(Operation::Label, condition_label);
            // phi all assigned values
            // and replace uses in for block to phi'd values
            for phi in phi_assignments.unwrap() {
                let address = code.push(Operation::Phi((phi.1).0, (phi.1).1));
                code.store(&phi.0, address, false);
                code.replace_label((before_code_size..after_code_size), (phi.1).1,address);
            }


            // eval condition
            let cond = emit_expression(*exp1, code);
            code.push(Operation::JumpIfElse(cond, content_label, end_label));

            code.push_with_label(Operation::Label, end_label);
            // phi variables?

        }
        ast::Statement::IfElse(condition, true_block, false_block) => {
            let cond = emit_expression(*condition, code);

            let if_label = code.new_label();
            let else_label = code.new_label();
            let end_label = code.new_label();


            let label2 = if false_block.is_some() { else_label } else { end_label };
            // jump to first block
            code.push(Operation::JumpIfElse(cond, if_label, label2));

            code.push_with_label(Operation::Label, if_label);
            let old_phi = code.observe_assignments();
            emit_block(true_block, code);
            let true_assignments = code.finish_observing(old_phi).unwrap();
            code.push(Operation::Jump(end_label));


            let mut false_assignments = None;
            if let Some(bl) = false_block {
                code.push_with_label(Operation::Label, else_label);
                let old_phi = code.observe_assignments();
                emit_block(bl, code);
                false_assignments = code.finish_observing(old_phi);
                code.push(Operation::Jump(end_label));
            }

            code.push_with_label(Operation::Label, end_label);

            // emit phi instructions
            for phi in select_phi_operations(true_assignments, false_assignments) {
                let address = code.push(Operation::Phi((phi.1).0, (phi.1).1));
                code.store(&phi.0, address, false);
            }
        }
    }
}

fn select_phi_operations(
    true_block: HashMap<String, (Address, Address)>,
    false_block: Option<HashMap<String, (Address, Address)>>,
) -> HashMap<String, (Address, Address)> {
    let mut results = true_block;

    match false_block {
        None => {}
        Some(x) => {
            for (key, (new, old)) in x {
                match results.get(&key) {
                    None => {
                        results.insert(key, (new, old));
                    }
                    Some(&true_phi) => {
                        results.insert(key, (new, true_phi.0));
                    }
                }
            }
        }
    }
    results
}

fn emit_expression(exp: ast::Expression, code: &mut Code) -> Address {
    use ast::Expression::*;
    match exp {
        Variable(var) => {
            code.get(&var.identifier.val)
        }
        Literal(lit) => {
            match lit {
                ast::Literal::Int(val) => {
                    code.store_constant(ConstantValue::Int(val.val))
                }
                ast::Literal::Float(val) => {
                    code.store_constant(ConstantValue::Float(val.val))
                }
            }
        }
        Negation(exp) => {
            let exp_address = emit_expression(*exp, code);
            code.push(Operation::Neg(exp_address))
        }
        Mul(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Mul(left_address, right_address))
        }
        Div(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Div(left_address, right_address))
        }
        Add(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Add(left_address, right_address))
        }
        Sub(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Sub(left_address, right_address))
        }
        Less(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Less(left_address, right_address))
        }
        MoreEqual(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Less(right_address, left_address))
        }
        LessEqual(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::LessEq(left_address, right_address))
        }
        More(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::LessEq(right_address, left_address))
        }
        Equals(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Eq(left_address, right_address))
        }
        NotEquals(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Neq(left_address, right_address))
        }
        And(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::And(left_address, right_address))
        }
        Or(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Or(left_address, right_address))
        }
        Shift(shifted, shift_by) => {
            let left_address = emit_expression(*shifted, code);
            let right_address = emit_expression(*shift_by, code);
            code.push(Operation::Shift(left_address, right_address))
        }
        Scale(_scaled, _scale_by) => {
            0
        }
    }
}
