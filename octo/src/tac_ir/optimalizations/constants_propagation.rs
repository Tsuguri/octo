use std::collections::HashMap;


use super::super::*;

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
        match self.try_map(old) {
            None => 0,
            Some(x) => x,
        }
    }

    pub fn try_map(&self, old: Address) -> Option<Address> {
        self.id_map.get(&old).map(|x| *x)
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
            Arg(_) => {
                let addr = new_code.push(op);
                cons.insert(res, addr);
            }
            StoreVec3(val) => {
                let addr = new_code.store_constant(ConstantValue::Vec3(val));
                cons.insert(res, addr);
            }
            StoreVec2(val) => {
                let addr = new_code.store_constant(ConstantValue::Vec2(val));
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
            StoreBool(val) => {
                let addr = new_code.store_constant(ConstantValue::Bool(val));
                cons.insert(res, addr);
            }
            Neg(addr) => {
                let new_addr = cons.map(addr);
                if new_code.is_const(new_addr) {
                    let c = new_code.get_const(new_addr);
                    use ConstantValue::*;
                    let c = match c {
                        Float(val) => Float(-val),
                        Int(val) => Int(-val),
                        Vec2([v1, v2]) => Vec2([-v1, -v2]),
                        Vec3([v1, v2, v3]) => Vec3([-v1, -v2, -v3]),
                        Bool(val) => Bool(!val),
                    };
                    let addr = new_code.store_constant(c);
                    cons.insert(res, addr);
                }
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
            Eq(left, right) => {
                fold_eq(&mut new_code, &mut cons, left, right, res);
            }
            Neq(left, right) => {
                fold_neq(&mut new_code, &mut cons, left, right, res);
            }
            Less(left, right) => {
                // TODO: implement
                let left = cons.map(left);
                let right = cons.map(right);
                let addr = new_code.push(Operation::Less(left, right));
                cons.insert(res, addr);
            }
            LessEq(left, right) => {
                // TODO: implement
                let left = cons.map(left);
                let right = cons.map(right);
                let addr = new_code.push(Operation::LessEq(left, right));
                cons.insert(res, addr);
            }
            And(left_addr, right_addr) => {
                let left = cons.map(left_addr);
                let right = cons.map(right_addr);
                let left_const = new_code.is_const(left);
                let right_const = new_code.is_const(right);
                match (left_const, right_const) {
                    (false, false) => {
                        let new_addr = new_code.push(Operation::And(left, right));
                        cons.insert(res, new_addr);
                    }
                    (true, false) => {
                        if let ConstantValue::Bool(false) = new_code.get_const(left) {
                            let addr = new_code.store_constant(ConstantValue::Bool(false));
                            cons.insert(res, addr);
                        }
                    }
                    (false, true) => {
                        if let ConstantValue::Bool(false) = new_code.get_const(right) {
                            let addr = new_code.store_constant(ConstantValue::Bool(false));
                            cons.insert(res, addr);
                        }
                    }
                    (true, true) => {
                        let val = match (new_code.get_const(left), new_code.get_const(right)) {
                            (ConstantValue::Bool(true), ConstantValue::Bool(true)) => true,
                            _ => false
                        };
                        let addr = new_code.store_constant(ConstantValue::Bool(val));
                        cons.insert(res, addr);
                    }
                }
            }
            Or(left_addr, right_addr) => {
                let left = cons.map(left_addr);
                let right = cons.map(right_addr);
                let left_const = new_code.is_const(left);
                let right_const = new_code.is_const(right);
                match (left_const, right_const) {
                    (false, false) => {
                        let new_addr = new_code.push(Operation::Or(left, right));
                        cons.insert(res, new_addr);
                    }
                    (true, false) => {
                        if let ConstantValue::Bool(true) = new_code.get_const(left) {
                            let addr = new_code.store_constant(ConstantValue::Bool(true));
                            cons.insert(res, addr);
                        }
                    }
                    (false, true) => {
                        if let ConstantValue::Bool(true) = new_code.get_const(right) {
                            let addr = new_code.store_constant(ConstantValue::Bool(true));
                            cons.insert(res, addr);
                        }
                    }
                    (true, true) => {
                        let val = match (new_code.get_const(left), new_code.get_const(right)) {
                            (ConstantValue::Bool(false), ConstantValue::Bool(false)) => false,
                            _ => true
                        };
                        let addr = new_code.store_constant(ConstantValue::Bool(val));
                        cons.insert(res, addr);
                    }
                }
            }
            Exit(addr) => {
                let new_addr = cons.map(addr);
                new_code.push(Operation::Exit(new_addr));
            }
            Phi(record) => {
                let new = cons.map(record.new);
                let label = cons.map(record.label);
                let old = cons.map(record.old);
                let old_label = cons.map(record.old_label);
                let record = PhiRecord{
                    new,
                    label,
                    old,
                    old_label
                };
                let addr = new_code.push(Operation::Phi(record));
                cons.insert(res, addr);
            }
            JumpIfElse(cond, true_address, false_address) => {
                let cond_addr = cons.map(cond);
                let true_label_addr = map_label(&mut cons, &mut new_code, true_address);
                let false_label_addr = map_label(&mut cons, &mut new_code, false_address);

                let ret_addr = new_code.push(Operation::JumpIfElse(cond_addr, true_label_addr, false_label_addr));
                cons.insert(res, ret_addr);
            }
            Jump(addr) => {
                let x = map_label(&mut cons, &mut new_code, addr);
                let addr = new_code.push(Operation::Jump(x));
                cons.insert(res, addr);
            }
            Sync(addr) => {
                let new_addr = cons.map(addr);
                if new_code.is_const(new_addr) {
                    // no need to sync constant value as it's the same in all threads.
                    // TODO: think about warning for that
                    cons.insert(res, new_addr);
                } else {
                    let addr = new_code.push(Operation::Sync(new_addr));
                    cons.insert(res, addr);
                }
            }
            Shift(shifted, shift_by) => (|| {
                let shifted = cons.map(shifted);
                let shift_by = cons.map(shift_by);

                if new_code.is_const(shifted) {
                    cons.insert(res, shifted);
                    return;
                }
                if new_code.is_const(shift_by) {
                    if let ConstantValue::Vec2([x, y]) = new_code.get_const(shift_by) {
                        if x == 0.0 && y == 0.0 {
                            cons.insert(res, shifted);
                            // only exits this lambda function
                            return;
                        }
                    }
                }
                let addr = new_code.push(Operation::Shift(shifted, shift_by));
                cons.insert(res, addr);
            })(),
            Label => {
                match cons.try_map(res) {
                    Some(x) => {
                        new_code.push_with_label(Operation::Label, x);
                    }
                    None => {
                        let new_addr = new_code.push(Operation::Label);
                        cons.insert(res, new_addr);
                    }
                }
            }
        }
    }
    new_code.
        code
}

fn map_label(cons: &mut ConstantsContext, code: &mut Code, addr: Address) -> Address {
    match cons.try_map(addr) {
        Some(x) => x,
        None => {
            let new_label_addr = code.new_label();
            cons.insert(addr, new_label_addr);
            new_label_addr
        }
    }
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
    }, |x, y| {
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
    }, |x, y| {
        Operation::Mul(x, y)
    });
}

fn fold_neq(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;
        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Bool(l != r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Bool(l != r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Bool(l[0] != r[0] || l[1] != r[1])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Bool(l[0] != r[0] || l[1] != r[1] || l[2] != r[2])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }
    }, |x, y| {
        Operation::Neq(x, y)
    });
}

fn fold_eq(code: &mut Code, cons: &mut ConstantsContext, left_addr: Address, right_addr: Address, old_address: Address) {
    fold(code, cons, left_addr, right_addr, old_address, |x, y| {
        use ConstantValue::*;
        match (x, y) {
            (Float(l), Float(r)) => {
                ConstantValue::Bool(l == r)
            }
            (Int(l), Int(r)) => {
                ConstantValue::Bool(l == r)
            }
            (Vec2(l), Vec2(r)) => {
                ConstantValue::Bool(l[0] == r[0] && l[1] == r[1])
            }
            (Vec3(l), Vec3(r)) => {
                ConstantValue::Bool(l[0] == r[0] && l[1] == r[1] && l[2] == r[2])
            }
            _ => {
                // panic
                unreachable!("wow");
            }
        }
    }, |x, y| {
        Operation::Eq(x, y)
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
    }, |x, y| {
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
    }, |x, y| {
        Operation::Add(x, y)
    });
}




