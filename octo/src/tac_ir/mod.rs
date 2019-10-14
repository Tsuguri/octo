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
    Vec2([f64;2]),
    Vec3([f64;3]),
    Bool(bool),
}

#[derive(Debug)]
pub enum Operation {
    Arg(usize),
    StoreInt(i64),
    StoreFloat(f64),
    StoreVec2([f64;2]),
    StoreVec3([f64;3]),
    StoreBool(bool),
    Add(Address, Address),
    Sub(Address, Address),
    Mul(Address, Address),
    Div(Address, Address),
    Less(Address, Address),
    LessEq(Address, Address),
    Neg(Address),
    Shift(Address, Address),
    Exit(Address),
}


struct Code {
    pub code: Vec<Op>,
    variables: HashMap<String, Address>,
    constants: HashMap<Address, ConstantValue>,

    counter: usize,
}


impl Code {
    pub fn new() -> Self {
        Code {
            code: vec![],
            variables: HashMap::new(),
            constants: HashMap::new(),
            counter:0,
        }
    }
    pub fn new_label(&mut self) -> Address {
        self.counter += 1;
        self.counter
    }
    pub fn push(&mut self, op: Operation) -> Address{
        let lab = self.new_label();
        self.code.push((lab, op));
        lab
    }
    pub fn store(&mut self, name: &str, add: Address) {
        self.variables.insert(name.to_owned(), add);
    }
    pub fn get(&self, name: &str) -> Address {
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
        let address =match val {
            Float(val) => {
                self.push(Operation::StoreFloat(val))

            },
            Int(val) => {
                self.push(Operation::StoreInt(val))
            },
            Vec2(val) => {
                self.push(Operation::StoreVec2(val))
            },
            Vec3(val) => {
                self.push(Operation::StoreVec3(val))
            },
            Bool(val) => {
                self.push(Operation::StoreBool(val))
            },
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
        let res =self.constants.contains_key(&addr);
        //println!("{} is {}", addr, if res {"const"} else {"mut"});


        res
    }
}


pub fn emit_ir(ast: IncomingIR) -> Vec<Op> {
    let mut code = Code::new();

    for arg in ast.arguments.iter().enumerate() {
        let addr = code.push(Operation::Arg(arg.0));
        code.store(&arg.1.identifier.val, addr);
    }

    for statement in ast.block.statements {
        emit_statement(statement, &mut code);
    }


    // analyze statements

    code.code
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
            code.store(&var.identifier.val, addr);
        }
        ast::Statement::For(stat, exp1, exp2, block) => {

        }
        ast::Statement::IfElse(condition, true_block, false_block) => {

        }
    }
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
        _ => { 0 }
    }
}
