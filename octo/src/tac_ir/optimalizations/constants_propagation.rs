use std::collections::HashMap;

use super::ir::{Address, ConstantValue, Operation, PhiRecord, PipelineIR};

use super::code::Code;

struct ConstantsContext {
    id_map: HashMap<Address, Address>,
    code: Code,
}

impl ConstantsContext {
    pub fn new() -> Self {
        ConstantsContext {
            code: Code::empty(),
            id_map: HashMap::new(),
        }
    }

    pub fn finish(self) -> Code {
        self.code
    }

    pub fn map(&mut self, old: Address) -> Address {
        let mapped = self.try_map(old);
        match mapped {
            None => {
                let id = self.code.new_label();
                self.insert(old, id);
                id
            }
            Some(x) => x,
        }
    }

    pub fn push(&mut self, operation: (Address, Operation)) {
        let new_addr = self.map(operation.0);
        self.code.push_with_label(operation.1, new_addr);
    }

    pub fn push_constant(&mut self, old_address: Address, value: ConstantValue) {
        let addr = self.code.store_constant(value);
        self.insert(old_address, addr);
    }

    pub fn try_map(&self, old: Address) -> Option<Address> {
        self.id_map.get(&old).map(|x| *x)
    }

    pub fn insert(&mut self, old: Address, new: Address) {
        self.id_map.insert(old, new);
    }

    pub fn is_const(&self, address: Address) -> bool {
        self.code.is_const(address)
    }

    pub fn get_const(&self, address: Address) -> ConstantValue {
        self.code.get_const(address)
    }
}

pub fn propagate_constants(code: PipelineIR) -> PipelineIR {
    //let mut cons = Code::new();
    let mut cons = ConstantsContext::new();

    for (res, op) in code.operations() {
        let res = *res;
        let op = *op;
        use Operation::*;
        match op {
            Arg(_) => {
                //let addr = cons.push(op);
                //cons.insert(res, addr);
                cons.push((res, op));
            }
            LoopMerge(..) => {
                cons.push((res, op));
            }
            StoreVec3(val) => {
                cons.push_constant(res, ConstantValue::Vec3(val));
            }
            StoreVec2(val) => {
                cons.push_constant(res, ConstantValue::Vec2(val));
            }
            StoreInt(val) => {
                cons.push_constant(res, ConstantValue::Int(val));
            }
            StoreFloat(val) => {
                cons.push_constant(res, ConstantValue::Float(val));
            }
            StoreBool(val) => {
                cons.push_constant(res, ConstantValue::Bool(val));
            }
            ConstructVec2(..) => {
                panic!("not implemented");
            }
            ConstructVec3(..) => {
                panic!("not implemented");
            }
            ExtractComponent(..)=>{
                panic!("not implemented");
            }
            StoreComponent(..) => {
                panic!("not implemented");
            }
            Store(addr) => {
                let new_addr = cons.map(addr);
                cons.insert(res, new_addr);
            }
            Neg(addr) => {
                let new_addr = cons.map(addr);
                if cons.is_const(new_addr) {
                    let c = cons.get_const(new_addr);
                    use ConstantValue::*;
                    let c = match c {
                        Float(val) => Float(-val),
                        Int(val) => Int(-val),
                        Vec2([v1, v2]) => Vec2([-v1, -v2]),
                        Vec3([v1, v2, v3]) => Vec3([-v1, -v2, -v3]),
                        Bool(val) => Bool(!val),
                    };
                    cons.push_constant(res, c);
                }
            }
            Add(left_addr, right_addr) => {
                fold_add(&mut cons, left_addr, right_addr, res);
            }
            Sub(left, right) => {
                fold_sub(&mut cons, left, right, res);
            }
            Mul(left, right) => {
                fold_mul(&mut cons, left, right, res);
            }
            Div(left, right) => {
                fold_div(&mut cons, left, right, res);
            }
            Eq(left, right) => {
                fold_eq(&mut cons, left, right, res);
            }
            Neq(left, right) => {
                fold_neq(&mut cons, left, right, res);
            }
            Less(left, right) => {
                // TODO: implement
                let left = cons.map(left);
                let right = cons.map(right);
                cons.push((res, Operation::Less(left, right)));
            }
            LessEq(left, right) => {
                // TODO: implement
                let left = cons.map(left);
                let right = cons.map(right);
                cons.push((res, Operation::LessEq(left, right)));
            }
            And(left_addr, right_addr) => {
                let left = cons.map(left_addr);
                let right = cons.map(right_addr);
                let left_const = cons.is_const(left);
                let right_const = cons.is_const(right);
                match (left_const, right_const) {
                    (false, false) => {
                        cons.push((res, Operation::And(left, right)));
                    }
                    (true, false) => {
                        if let ConstantValue::Bool(false) = cons.get_const(left) {
                            cons.push_constant(res, ConstantValue::Bool(false));
                        }
                    }
                    (false, true) => {
                        if let ConstantValue::Bool(false) = cons.get_const(right) {
                            cons.push_constant(res, ConstantValue::Bool(false));
                        }
                    }
                    (true, true) => {
                        let val = match (cons.get_const(left), cons.get_const(right)) {
                            (ConstantValue::Bool(true), ConstantValue::Bool(true)) => true,
                            _ => false,
                        };
                        cons.push_constant(res, ConstantValue::Bool(val));
                    }
                }
            }
            Or(left_addr, right_addr) => {
                let left = cons.map(left_addr);
                let right = cons.map(right_addr);
                let left_const = cons.is_const(left);
                let right_const = cons.is_const(right);
                match (left_const, right_const) {
                    (false, false) => {
                        cons.push((res, Operation::Or(left, right)));
                    }
                    (true, false) => {
                        if let ConstantValue::Bool(true) = cons.get_const(left) {
                            cons.push_constant(res, ConstantValue::Bool(true));
                        }
                    }
                    (false, true) => {
                        if let ConstantValue::Bool(true) = cons.get_const(right) {
                            cons.push_constant(res, ConstantValue::Bool(true));
                        }
                    }
                    (true, true) => {
                        let val = match (cons.get_const(left), cons.get_const(right)) {
                            (ConstantValue::Bool(false), ConstantValue::Bool(false)) => false,
                            _ => true,
                        };
                        cons.push_constant(res, ConstantValue::Bool(val));
                    }
                }
            }
            Exit(addr, label) => {
                let new_addr = cons.map(addr);
                let new_label = cons.map(label);
                cons.push((res, Operation::Exit(new_addr, new_label)));
            }
            Phi(record) => {
                let new = cons.map(record.new);
                let label = cons.map(record.label);
                let old = cons.map(record.old);
                let old_label = cons.map(record.old_label);
                let record = PhiRecord {
                    new,
                    label,
                    old,
                    old_label,
                };
                cons.push((res, Operation::Phi(record)));
            }
            JumpIfElse(cond, true_address, false_address) => {
                let cond_addr = cons.map(cond);
                let true_label_addr = cons.map(true_address);
                let false_label_addr = cons.map(false_address);

                cons.push((
                    res,
                    Operation::JumpIfElse(cond_addr, true_label_addr, false_label_addr),
                ));
            }
            Jump(addr) => {
                let x = cons.map(addr);
                cons.push((res, Operation::Jump(x)));
            }
            Sync(addr) => {
                let new_addr = cons.map(addr);
                if cons.is_const(new_addr) {
                    // no need to sync constant value as it's the same in all threads.
                    // TODO: think about warning for that
                    cons.insert(res, new_addr);
                } else {
                    cons.push((res, Operation::Sync(new_addr)));
                }
            }
            Invoke(..) => {
                panic!("not implemented");
            }
            Shift(shifted, shift_by) => (|| {
                let shifted = cons.map(shifted);
                let shift_by = cons.map(shift_by);

                if cons.is_const(shifted) {
                    cons.insert(res, shifted);
                    return;
                }
                if cons.is_const(shift_by) {
                    if let ConstantValue::Vec2([x, y]) = cons.get_const(shift_by) {
                        if x == 0.0 && y == 0.0 {
                            cons.insert(res, shifted);
                            // only exits this lambda function
                            return;
                        }
                    }
                }
                cons.push((res, Operation::Shift(shifted, shift_by)));
            })(),
            Label => {
                cons.push((res, Operation::Label));
            }
        }
    }
    cons.finish().finish_with(&code)
}

fn fold<
    F: Fn(ConstantValue, ConstantValue) -> ConstantValue,
    F2: Fn(Address, Address) -> Operation,
>(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
    f1: F,
    f2: F2,
) {
    let left = cons.map(left_addr);
    let right = cons.map(right_addr);
    //    println!("left: {}, right: {}", left, right);
    if cons.is_const(left) && cons.is_const(right) {
        let left_val = cons.get_const(left);
        let right_val = cons.get_const(right);
        let val = f1(left_val, right_val);

        cons.push_constant(old_address, val);
    } else {
        cons.push((old_address, f2(left, right)));
    }
}

fn fold_div(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;

            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Float(l / r),
                (Int(l), Int(r)) => ConstantValue::Int(l / r),
                (Vec2(l), Vec2(r)) => ConstantValue::Vec2([l[0] / r[0], l[1] / r[1]]),
                (Vec3(l), Vec3(r)) => ConstantValue::Vec3([l[0] / r[0], l[1] / r[1], l[2] / r[2]]),
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Div(x, y),
    );
}

fn fold_mul(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;

            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Float(l * r),
                (Int(l), Int(r)) => ConstantValue::Int(l * r),
                (Vec2(l), Vec2(r)) => ConstantValue::Vec2([l[0] * r[0], l[1] * r[1]]),
                (Vec3(l), Vec3(r)) => ConstantValue::Vec3([l[0] * r[0], l[1] * r[1], l[2] * r[2]]),
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Mul(x, y),
    );
}

fn fold_neq(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;
            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Bool(l != r),
                (Int(l), Int(r)) => ConstantValue::Bool(l != r),
                (Vec2(l), Vec2(r)) => ConstantValue::Bool(l[0] != r[0] || l[1] != r[1]),
                (Vec3(l), Vec3(r)) => {
                    ConstantValue::Bool(l[0] != r[0] || l[1] != r[1] || l[2] != r[2])
                }
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Neq(x, y),
    );
}

fn fold_eq(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;
            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Bool(l == r),
                (Int(l), Int(r)) => ConstantValue::Bool(l == r),
                (Vec2(l), Vec2(r)) => ConstantValue::Bool(l[0] == r[0] && l[1] == r[1]),
                (Vec3(l), Vec3(r)) => {
                    ConstantValue::Bool(l[0] == r[0] && l[1] == r[1] && l[2] == r[2])
                }
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Eq(x, y),
    );
}

fn fold_sub(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;

            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Float(l - r),
                (Int(l), Int(r)) => ConstantValue::Int(l - r),
                (Vec2(l), Vec2(r)) => ConstantValue::Vec2([l[0] - r[0], l[1] - r[1]]),
                (Vec3(l), Vec3(r)) => ConstantValue::Vec3([l[0] - r[0], l[1] - r[1], l[2] - r[2]]),
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Sub(x, y),
    );
}

fn fold_add(
    cons: &mut ConstantsContext,
    left_addr: Address,
    right_addr: Address,
    old_address: Address,
) {
    fold(
        cons,
        left_addr,
        right_addr,
        old_address,
        |x, y| {
            use ConstantValue::*;

            match (x, y) {
                (Float(l), Float(r)) => ConstantValue::Float(l + r),
                (Int(l), Int(r)) => ConstantValue::Int(l + r),
                (Vec2(l), Vec2(r)) => ConstantValue::Vec2([l[0] + r[0], l[1] + r[1]]),
                (Vec3(l), Vec3(r)) => ConstantValue::Vec3([l[0] + r[0], l[1] + r[1], l[2] + r[2]]),
                _ => {
                    // panic
                    unreachable!("wow");
                }
            }
        },
        |x, y| Operation::Add(x, y),
    );
}
