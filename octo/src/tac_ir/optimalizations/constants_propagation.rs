use std::collections::HashMap;

use super::ir::{Address, ConstantValue, Operation, PhiRecord, PipelineIR};

use super::code::Code;
use super::ConstantPropagationContext;


pub fn propagate_constants(code: PipelineIR) -> PipelineIR {
    let mut ctx = ConstantPropagationContext::default();
    let mut code = code;

    let mut operations: Vec<(Address, Operation)> = code.operations().rev().map(|x| *x).collect();
    let mut result_code = Vec::with_capacity(operations.len());
    let mut label_map: HashMap<Address, Address> = HashMap::new();
    let mut current_label = 0;

    while let Some(x) = operations.pop() {
    //let result_code: Vec<(Address, Operation)> = code.operations().map(|x| {
        let result_address = x.0;
        let x = x.1;

        let result_operation = match propagate_constant_operation(&mut ctx, &mut operations, x, result_address, &mut label_map, &mut current_label) {
            Some(val) => val,
            None => continue,
        };
        result_code.push((result_address, result_operation));
    }



    PipelineIR::with(result_code, &code)
}

pub fn propagate_constant_operation(
    ctx: &mut ConstantPropagationContext,
    operations: &mut Vec<(Address, Operation)>,
    x: Operation,
    result_address: Address,
    label_map: &mut HashMap<Address, Address>,
    current_label: &mut Address,
    ) -> Option<Operation> {
    use Operation::*;
    let result_operation = match x {
        Arg(id) => Some(x),
        Uniform(id) => Some(x),
        StoreInt(val) => {
            ctx.store_const(result_address, ConstantValue::Int(val));
            Some(x)
        }
        StoreFloat(val) => {
            ctx.store_const(result_address, ConstantValue::Float(val));
            Some(x)
        }
        StoreVec2(val) => {
            ctx.store_const(result_address, ConstantValue::Vec2(val));
            Some(x)
        }
        StoreVec3(val) => {
            ctx.store_const(result_address, ConstantValue::Vec3(val));
            Some(x)
        }
        StoreVec4(val) => {
            ctx.store_const(result_address, ConstantValue::Vec4(val));
            Some(x)
        }
        StoreBool(val) => {
            ctx.store_const(result_address, ConstantValue::Bool(val));
            Some(x)
        }
        Store(addr) => {
            match ctx.get_const(&addr) {
                None => Some(x),
                Some(val) => {
                    let op = ctx.copy_const(result_address, val);
                    Some(op)
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
                    Some(op)
                },
                _ => Some(x),
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
                    Some(Operation::StoreVec3([f1, f2, f3]))
                },
                _ => Some(x),
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
                    Some(Operation::StoreVec4([f1, f2, f3, f4]))
                },
                _ => Some(x),
            }
        },
        ExtractComponent(addr, id) => {
            match ctx.get_const(&addr) {
                Some(val) => {
                    match val {
                        ConstantValue::Vec2(v) => {
                            assert!(id<2);
                            ctx.store_const(result_address, ConstantValue::Float(v[id]));
                            Some(Operation::StoreFloat(v[id]))
                        },
                        ConstantValue::Vec3(v) => {
                            assert!(id<3);
                            ctx.store_const(result_address, ConstantValue::Float(v[id]));
                            Some(Operation::StoreFloat(v[id]))
                        },
                        ConstantValue::Vec4(v) => {
                            assert!(id<4);
                            ctx.store_const(result_address, ConstantValue::Float(v[id]));
                            Some(Operation::StoreFloat(v[id]))
                        },
                        _ => unreachable!(),
                    }
                },
                None => Some(x)
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
                            Some(Operation::StoreVec2(p))
                        },
                        ConstantValue::Vec3(v) => {
                            assert!(id<2);
                            let mut p = v;
                            p[id] = f;
                            ctx.store_const(result_address, ConstantValue::Vec3(p));
                            Some(Operation::StoreVec3(p))
                        },
                        ConstantValue::Vec4(v) => {
                            assert!(id<2);
                            let mut p = v;
                            p[id] = f;
                            ctx.store_const(result_address, ConstantValue::Vec4(p));
                            Some(Operation::StoreVec4(p))
                        },
                        _ => unreachable!(),
                    }

                },
                _ => Some(x)
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))
                }
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))
                }
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
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
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
            }
        },
        Neg(addr) => {
            match ctx.get_const(&addr) {
                Some(v) => {
                    match v {
                        ConstantValue::Bool(val) => ConstantValue::Bool(!val),
                        _ => unreachable!(),
                    };
                    Some(ctx.copy_const(result_address, v))
                },
                _ => Some(x),
            }
        }
        And(addr1, addr2) => {
            match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                (Some(v1), Some(v2)) => {
                    let val = match (v1, v2) {
                        (ConstantValue::Bool(x1), ConstantValue::Bool(x2)) => ConstantValue::Bool(x1 && x2),
                        _ => unreachable!(),
                    };
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
            }
        },
        Or(addr1, addr2) => {
            match (ctx.get_const(&addr1), ctx.get_const(&addr2)) {
                (Some(v1), Some(v2)) => {
                    let val = match (v1, v2) {
                        (ConstantValue::Bool(x1), ConstantValue::Bool(x2)) => ConstantValue::Bool(x1 || x2),
                        _ => unreachable!(),
                    };
                    Some(ctx.copy_const(result_address, val))

                },
                _ => Some(x),
            }
        },
        Exit(a, b) => {
            let lab = if label_map.contains_key(&b) {
                label_map[&b]
            } else {
                b
            };
            Some(Exit(a,lab))
        },
        Invoke(..) => Some(x), //ignore now for simplicity
        Sync(addr) => {
            // syncing const value seems useless
            match ctx.get_const(&addr) {
                Some(y) => {
                    Some(ctx.copy_const(result_address, y))
                },
                None => Some(x)
            }
        }
        Shift(..) => Some(x),
        Phi(rec) => {
            let mut modified_rec = rec;
            if label_map.contains_key(&modified_rec.label) {
                modified_rec.label = label_map[&modified_rec.label];
            }
            if label_map.contains_key(&modified_rec.old_label) {
                modified_rec.old_label = label_map[&modified_rec.old_label];
            }

            Some(Phi(modified_rec))
        },
        Jump(lab) => {
            Some(if label_map.contains_key(&lab) {
                Operation::Jump(label_map[&lab])
            } else {
                x
            })

        },
        JumpIfElse(cond, true_label, false_label) => {
            if propagate_jump_if_else(ctx, cond, true_label, false_label, operations, label_map, current_label) {
                Some(x)
            } else {
                None
            }
        }
        LoopMerge(..) => Some(x),
        Label => {
            *current_label = result_address;
            Some(x)
        },
        //_=> x,
    };

    return result_operation;
}

fn propagate_jump_if_else(
    ctx: &mut ConstantPropagationContext,
    cond: Address,
    true_label: Address,
    false_label: Address,
    operations: &mut Vec<(Address, Operation)>,
    label_map: &mut HashMap<Address, Address>,
    current_label: &mut Address
) -> bool {
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
            label_map.insert(end_label, *current_label);

            if end_label!=false_label {
                while let Some(x) = operations.pop() {
                    if x.0 == end_label {
                        break;
                    }
                }
            }

            while let Some(x) = operations.pop() {
                match x.1 {
                    Operation::Phi(rec) => {
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
            label_map.insert(end_label, *current_label);
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
                    Operation::Phi(rec) => {
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

        return false;
    }
    return true;
}
