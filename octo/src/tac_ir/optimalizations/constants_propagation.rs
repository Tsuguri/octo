use std::collections::HashMap;

use super::ir::{Address, ConstantValue, Operation, PhiRecord, PipelineIR};

use super::code::Code;

struct ConstantPropagationContext
{
    constants: HashMap<Address, ConstantValue>,

}

impl ConstantPropagationContext {
    pub fn get_const(&self, add: &Address) -> Option<ConstantValue> {
        self.constants.get(add).map(|x| *x)
    }

    pub fn store_const(&mut self, add: Address, value: ConstantValue) {
        self.constants.insert(add, value);
    }

    pub fn copy_const(&mut self, add: Address, value: ConstantValue) -> Operation {
        self.store_const(add, value);
        
        use ConstantValue::*;
        let op = match value {
            Int(v) => Operation::StoreInt(v),
            Float(v) => Operation::StoreFloat(v),
            Vec2(v) => Operation::StoreVec2(v),
            Vec3(v) => Operation::StoreVec3(v),
            Vec4(v) => Operation::StoreVec4(v),
            Bool(v) => Operation::StoreBool(v),
        };
        op

    }
}

pub fn propagate_constants(code: PipelineIR) -> PipelineIR {
    let mut ctx = ConstantPropagationContext {
        constants: Default::default(),
    };
    let mut code = code;

    let mut operations: Vec<(Address, Operation)> = code.operations().rev().map(|x| *x).collect();
    let mut result_code = Vec::with_capacity(operations.len());
    let mut label_map: HashMap<Address, Address> = HashMap::new();
    let mut current_label = 0;

    while let Some(x) = operations.pop() {
    //let result_code: Vec<(Address, Operation)> = code.operations().map(|x| {
        let result_address = x.0;
        use Operation::*;
        let x = x.1;
        let result_operation = match x {
            Arg(id) => x,
            Uniform(id) => x,
            StoreInt(val) => {
                ctx.store_const(result_address, ConstantValue::Int(val));
                x
            }
            StoreFloat(val) => {
                ctx.store_const(result_address, ConstantValue::Float(val));
                x
            }
            StoreVec2(val) => {
                ctx.store_const(result_address, ConstantValue::Vec2(val));
                x
            }
            StoreVec3(val) => {
                ctx.store_const(result_address, ConstantValue::Vec3(val));
                x
            }
            StoreVec4(val) => {
                ctx.store_const(result_address, ConstantValue::Vec4(val));
                x
            }
            StoreBool(val) => {
                ctx.store_const(result_address, ConstantValue::Bool(val));
                x
            }
            Store(addr) => {
                match ctx.get_const(&addr) {
                    None => x,
                    Some(val) => {
                        let op = ctx.copy_const(result_address, val);
                        op
                    }
                }
            },
            ConstructVec2(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        let f1 = match v1 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f2 = match v2 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let op = Operation::StoreVec2([f1, f2]);
                        ctx.store_const(result_address, ConstantValue::Vec2([f1, f2]));
                        op
                    },
                    _ => x,
                }
            },
            ConstructVec3(addr1, addr2, addr3) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2), ctx.get_const(&addr3)) {
                    (Some(v1), Some(v2), Some(v3)) => {
                        let f1 = match v1 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f2 = match v2 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f3 = match v3 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        ctx.store_const(result_address, ConstantValue::Vec3([f1, f2, f3]));
                        Operation::StoreVec3([f1, f2, f3])
                    },
                    _ => x,
                }
            },
            ConstructVec4(addr1, addr2, addr3, addr4) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2), ctx.get_const(&addr3), ctx.get_const(&addr4)) {
                    (Some(v1), Some(v2), Some(v3), Some(v4)) => {
                        let f1 = match v1 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f2 = match v2 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f3 = match v3 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        let f4 = match v4 {
                            ConstantValue::Float(v) => v,
                            _ => unreachable!(),
                        };
                        ctx.store_const(result_address, ConstantValue::Vec4([f1, f2, f3, f4]));
                        Operation::StoreVec4([f1, f2, f3, f4])
                    },
                    _ => x,
                }
            },
            ExtractComponent(addr, id) => {
                match ctx.get_const(&addr) {
                    Some(val) => {
                        match val {
                            ConstantValue::Vec2(v) => {
                                assert!(id<2);
                                ctx.store_const(result_address, ConstantValue::Float(v[id]));
                                Operation::StoreFloat(v[id])
                            },
                            ConstantValue::Vec3(v) => {
                                assert!(id<3);
                                ctx.store_const(result_address, ConstantValue::Float(v[id]));
                                Operation::StoreFloat(v[id])
                            },
                            ConstantValue::Vec4(v) => {
                                assert!(id<4);
                                ctx.store_const(result_address, ConstantValue::Float(v[id]));
                                Operation::StoreFloat(v[id])
                            },
                            _ => unreachable!(),
                        }
                    },
                    None => x
                }
            },
            StoreComponent(addr, id, addr2) => {
                match (ctx.get_const(&addr), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        let f = match v2 {
                            ConstantValue::Float(val) => val,
                            _ => unreachable!(),
                        };
                        match v1 {
                            ConstantValue::Vec2(v) => {
                                assert!(id<2);
                                let mut p = v;
                                p[id] = f;
                                ctx.store_const(result_address, ConstantValue::Vec2(p));
                                Operation::StoreVec2(p)
                            },
                            ConstantValue::Vec3(v) => {
                                assert!(id<2);
                                let mut p = v;
                                p[id] = f;
                                ctx.store_const(result_address, ConstantValue::Vec3(p));
                                Operation::StoreVec3(p)
                            },
                            ConstantValue::Vec4(v) => {
                                assert!(id<2);
                                let mut p = v;
                                p[id] = f;
                                ctx.store_const(result_address, ConstantValue::Vec4(p));
                                Operation::StoreVec4(p)
                            },
                            _ => unreachable!(),
                        }

                    },
                    _ => x
                }
            },
            Add(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Float(a1 + a2),
                            (Int(a1), Int(a2)) => ConstantValue::Int(a1 + a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Vec2([a1[0] + a2[0], a1[1] + a2[1]]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Vec3([a1[0] + a2[0], a1[1] + a2[1], a1[2] + a2[2]]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Vec4([a1[0] + a2[0], a1[1] + a2[1], a1[2] + a2[2], a1[3] + a2[3]]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Sub(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Float(a1 - a2),
                            (Int(a1), Int(a2)) => ConstantValue::Int(a1 - a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Vec2([a1[0] - a2[0], a1[1] - a2[1]]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Vec3([a1[0] - a2[0], a1[1] - a2[1], a1[2] - a2[2]]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Vec4([a1[0] - a2[0], a1[1] - a2[1], a1[2] - a2[2], a1[3] - a2[3]]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            }
            Mul(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Float(a1 * a2),
                            (Int(a1), Int(a2)) => ConstantValue::Int(a1 * a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Vec2([a1[0] * a2[0], a1[1] * a2[1]]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Vec3([a1[0] * a2[0], a1[1] * a2[1], a1[2] * a2[2]]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Vec4([a1[0] * a2[0], a1[1] * a2[1], a1[2] * a2[2], a1[3] * a2[3]]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            }
            Div(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Float(a1 / a2),
                            (Int(a1), Int(a2)) => ConstantValue::Int(a1 / a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Vec2([a1[0] / a2[0], a1[1] / a2[1]]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Vec3([a1[0] / a2[0], a1[1] / a2[1], a1[2] / a2[2]]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Vec4([a1[0] / a2[0], a1[1] / a2[1], a1[2] / a2[2], a1[3] / a2[3]]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Less(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Bool(a1<a2),
                            (Int(a1), Int(a2)) => ConstantValue::Bool(a1 < a2),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)
                    }
                    _ => x,
                }
            },
            LessEq(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Bool(a1<=a2),
                            (Int(a1), Int(a2)) => ConstantValue::Bool(a1 <= a2),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)
                    }
                    _ => x,
                }
            },
            Eq(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Bool(a1 == a2),
                            (Int(a1), Int(a2)) => ConstantValue::Bool(a1 == a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Bool(a1[0] == a2[0] && a1[1] == a2[1]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Bool(a1[0] == a2[0] && a1[1] == a2[1] && a1[2] == a2[2]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Bool(a1[0] == a2[0] && a1[1] == a2[1] && a1[2] == a2[2] && a1[3] == a2[3]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Neq(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        use ConstantValue::*;
                        let val = match (v1, v2) {
                            (Float(a1), Float(a2)) => ConstantValue::Bool(a1 != a2),
                            (Int(a1), Int(a2)) => ConstantValue::Bool(a1 != a2),
                            (Vec2(a1), Vec2(a2)) => ConstantValue::Bool(a1[0] != a2[0] || a1[1] != a2[1]),
                            (Vec3(a1), Vec3(a2)) => ConstantValue::Bool(a1[0] != a2[0] || a1[1] != a2[1] || a1[2] != a2[2]),
                            (Vec4(a1), Vec4(a2)) => ConstantValue::Bool(a1[0] != a2[0] || a1[1] != a2[1] || a1[2] != a2[2] || a1[3] != a2[3]),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Neg(addr) => {
                match ctx.get_const(&addr) {
                    Some(v) => {
                        match v {
                            ConstantValue::Bool(val) => ConstantValue::Bool(!val),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, v)
                    },
                    _ => x,
                }
            }
            And(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        let val = match (v1, v2) {
                            (ConstantValue::Bool(x1), ConstantValue::Bool(x2)) => ConstantValue::Bool(x1 && x2),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Or(addr1, addr2) => {
                match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                    (Some(v1), Some(v2)) => {
                        let val = match (v1, v2) {
                            (ConstantValue::Bool(x1), ConstantValue::Bool(x2)) => ConstantValue::Bool(x1 || x2),
                            _ => unreachable!(),
                        };
                        ctx.copy_const(result_address, val)

                    },
                    _ => x,
                }
            },
            Exit(a, b) => {
                let lab = if label_map.contains_key(&b) {
                    label_map[&b]
                } else {
                    b
                };
                Exit(a,lab)
            },
            Invoke(..) => x, //ignore now for simplicity
            Sync(addr) => {
                // syncing const value seems useless
                match ctx.get_const(&addr) {
                    Some(y) => {
                        ctx.copy_const(result_address, y)
                    },
                    None => x
                }
            }
            Shift(..) => x,
            Phi(rec) => {
                let mut modified_rec = rec;
                if label_map.contains_key(&modified_rec.label) {
                    modified_rec.label = label_map[&modified_rec.label];
                }
                if label_map.contains_key(&modified_rec.old_label) {
                    modified_rec.old_label = label_map[&modified_rec.old_label];
                }

                Phi(modified_rec)
            },
            Jump(lab) => {
                if label_map.contains_key(&lab) {
                    Operation::Jump(label_map[&lab])
                } else {
                    x
                }

            },
            JumpIfElse(cond, true_label, false_label) => {
                if let Some(val) = ctx.get_const(&cond) {
                    let val = match val {
                        ConstantValue::Bool(cond_value) => cond_value,
                        _ => unreachable!(),
                    };
                    if val {
                        // true label
                        operations.pop();
                        let mut last_jump = 0;
                        let mut temp_ops: Vec<(Address, Operation)> = Vec::new();
                        while let Some(x) = operations.pop() {
                            match x.1 {
                                Operation::Jump(dest) => {last_jump = dest;},
                                _ => (),
                            }
                            if x.0 == false_label {
                                break;
                            }
                            temp_ops.push(x);
                        }
                        // jump from true to end
                        temp_ops.pop();
                        let end_label = last_jump;
                        label_map.insert(end_label, current_label);

                        if end_label!=false_label {
                            while let Some(x) = operations.pop() {
                                if x.0 == end_label {
                                    break;
                                }
                            }
                        }

                        while let Some(x) = operations.pop() {
                            match x.1 {
                                Phi(rec) => {
                                    if rec.label == true_label {
                                        temp_ops.push((x.0, Operation::Store(rec.new)));
                                    } else {
                                        temp_ops.push((x.0, Operation::Store(rec.old)));
                                    }
                                },
                                _ => {
                                    temp_ops.push(x);
                                    break;
                                }
                            }
                        }

                        while let Some(op) = temp_ops.pop() {
                            operations.push(op);
                        }

                    } else {
                        let mut last_jump = 0;

                        while let Some(x) = operations.pop() {
                            match x.1 {
                                Operation::Jump(dest) => {last_jump = dest;},
                                _ => (),
                            }
                            if x.0 == false_label {
                                assert!(last_jump != 0);
                                break;
                            }
                        }
                        let end_label = last_jump;
                        label_map.insert(end_label, current_label);
                        let mut temp_ops: Vec<(Address, Operation)> = Vec::new();

                        while let Some(x) = operations.pop() {
                            if x.0 == end_label {
                                break;
                            }
                            temp_ops.push(x);
                        }
                        // jump from else to end
                        temp_ops.pop();
                        while let Some(x) = operations.pop() {
                            match x.1 {
                                Phi(rec) => {
                                    if rec.label == false_label {
                                        temp_ops.push((x.0, Operation::Store(rec.new)));
                                    } else {
                                        temp_ops.push((x.0, Operation::Store(rec.old)));
                                    }
                                },
                                _ => {
                                    temp_ops.push(x);
                                    break;
                                }
                            }
                        }
                        while let Some(op) = temp_ops.pop() {
                            operations.push(op);
                        }
                    }

                    continue;
                } else {
                    x
                }
            }
            LoopMerge(..) => x,
            Label => {
                current_label = result_address;
                x
            },
            //_=> x,
        };

        result_code.push((result_address, result_operation));
    }



    PipelineIR::with(result_code, &code)
}