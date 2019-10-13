use std::collections::HashMap;


use super::super::*;
use super::super::super::ast;

struct ConstantsContext {
    id_map: HashMap<Address, Address>,
}

impl ConstantsContext {
    pub fn new() -> Self {
        ConstantsContext {
            id_map: HashMap::new(),
        }
    }

    pub fn map(&self, old: Address) -> Address {
        self.id_map[&old]
    }

    pub fn insert(&mut self, old: Address, new: Address) {
        self.id_map.insert(old, new);
    }
}

pub fn propagate_constants(code: Vec<Op>) -> Vec<Op> {
    let mut new_code = Code::new();
    let mut cons = ConstantsContext::new();

    for (res, op) in code {
        use super::Operation::*;
        match op {
            Arg(id) => {
                let addr = new_code.push(op);
                cons.insert(res, addr);
            }
            StoreInt(val) => {
                let addr = new_code.store_constant(ConstantValue::Int(val));
                cons.insert(res, addr);
            }
            StoreFloat(val) => {
                let addr = new_code.store_constant(ConstantValue::Float(val));
                cons.insert(res, addr);
            }
            Add(left_addr, right_addr) => {
                fold_add(&mut new_code, &mut cons, left_addr, right_addr, res);
            }
            Sub(left, right) => {
                fold_sub(&mut new_code, &mut cons, left, right, res);
            }
            Mul(left, right) => {
                fold_mul(&mut new_code, &mut cons, left, right, res);
            }
            Div(left, right) => {
                fold_div(&mut new_code, &mut cons, left, right, res);
            }
            Exit(addr) => {
                let new_addr = cons.map(addr);
                new_code.push(Operation::Exit(new_addr));
            }
            _ => {}
        }
    }
    new_code.code
}

fn fold<
    F: Fn(ConstantValue, ConstantValue) -> ConstantValue,
    F2: Fn(Address, Address) -> Operation,
>(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address, f1: F, f2: F2) {

    let left = cons.map(left_addr);
    let right = cons.map(right_addr);
//    println!("left: {}, right: {}", left, right);
    if code.is_const(left) && code.is_const(right) {
        let left_val = code.get_const(left);
        let right_val = code.get_const(right);
        let val = f1(left_val, right_val);

        let addr = code.store_constant(val);
        code.make_const(addr, val);
        cons.insert(old_address, addr);
    } else {
        let addr = code.push(f2(left, right));
        cons.insert(old_address, addr);
    }
}

fn fold_div(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;

        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Float(l / r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Int(l / r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Vec2([l[0] / r[0], l[1] / r[1]])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Vec3([l[0] / r[0], l[1] / r[1], l[2] / r[2]])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }

    }, |x, y|{
        Operation::Div(x, y)
    });
}

fn fold_mul(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;

        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Float(l * r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Int(l * r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Vec2([l[0] * r[0], l[1] * r[1]])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Vec3([l[0] * r[0], l[1] * r[1], l[2] * r[2]])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }

    }, |x, y|{
        Operation::Mul(x, y)
    });
}
fn fold_sub(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;

        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Float(l - r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Int(l - r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Vec2([l[0] - r[0], l[1] - r[1]])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Vec3([l[0] - r[0], l[1] - r[1], l[2] - r[2]])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }

    }, |x, y|{
        Operation::Sub(x, y)
    });
}

fn fold_add(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;

        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Float(l + r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Int(l + r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Vec2([l[0] + r[0], l[1] + r[1]])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Vec3([l[0] + r[0], l[1] + r[1], l[2] + r[2]])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }
    }, |x,y| {
        Operation::Add(x,y)
    });
}




