pub mod optimalizations;

// tac with SSA
use std::collections::HashMap;
use super::ast;
use ast::Pipeline as IncomingIR;

pub type Address = usize;

pub type Op = (Address, Operation);

pub use optimalizations::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ConstantValue {
    Int(i64),
    Float(f64),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Bool(bool),
}

#[derive(Debug)]
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
        print!("storing {} to {}: ", name, add);
        if let Some(assignments) = &mut self.phi_assignments {
            print!("phi store");
            // if we create new variable then it doesn't go into phi assignemts (local variable).

            if !create {
                let old = self.variables[name];
                assignments.insert(name.to_owned(), (add, old));
            }
        } else {
            self.variables.insert(name.to_owned(), add);
            print!("normal  store");
        }
        println!();
    }
    pub fn get(&self, name: &str) -> Address {
        if let Some(assignments) = &self.phi_assignments {
            println!("Getting phi val for {}", name);
            match assignments.get(name) {
                None => {},
                Some(x) => {
                    return x.0;
                },
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
        //println!("Storing {} as const", address);
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
        //println!("{} is {}", addr, if res {"const"} else {"mut"});


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
        ast::Statement::For(_stat, _exp1, _exp2, _block) => {}
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
        Shift(shifted, shift_by)=> {
            let left_address = emit_expression(*shifted, code);
            let right_address = emit_expression(*shift_by, code);
            code.push(Operation::Shift(left_address, right_address))

        }
        Scale(_scaled, _scale_by) => {
            0
        }
    }
}
