use super::ir::*;
use super::super::utils::{PeekableCode, find_loop, LoopCode};
use super::ConstantPropagationContext;
use super::constants_propagation::propagate_constant_operation;

use std::collections::HashMap;

pub fn unroll_synced_loop(code: PipelineIR) -> PipelineIR {
    let (code, inputs, outputs, uniforms) = code.take();

    let mut constants = ConstantPropagationContext::default();
    let mut max_id = code.iter().map(|x| x.0).max().unwrap();

    for (ret, op) in code.iter() {
        let ret = *ret;
        let op = *op;
        use Operation::*;
        match op {
            StoreInt(val) => {
                constants.store_const(ret, ConstantValue::Int(val));
            }
            StoreFloat(val) => {
                constants.store_const(ret, ConstantValue::Float(val));
            }
            StoreVec2(val) => {
                constants.store_const(ret, ConstantValue::Vec2(val));
            }
            StoreVec3(val) => {
                constants.store_const(ret, ConstantValue::Vec3(val));
            }
            StoreVec4(val) => {
                constants.store_const(ret, ConstantValue::Vec4(val));
            }
            StoreBool(val) => {
                constants.store_const(ret, ConstantValue::Bool(val));
            }
            Store(addr) => {
                match constants.get_const(&addr) {
                    None => (),
                    Some(val) => {
                        constants.copy_const(ret, val);
                    }
                }
            },
            _ => (),
        }
    }
    
    let mut peekable = PeekableCode::new(code.iter());

    let mut result_code = Vec::new();


    let mut last_label = 0;

    while let Some((ret, op_code)) = peekable.next().copied() {
        //println!("heheszki");
        result_code.push((ret, op_code));
        match op_code {
            Operation::Label => last_label = ret,
            Operation::LoopMerge(..) => {
                let loop_data = find_loop(ret, op_code, &mut peekable, last_label);
                if !contains_sync(&loop_data) {
                    continue;
                }
                // current op: LoopMerge
                result_code.pop();
                let mut phi_nodes = Vec::new();
                while let Some((phi_ret, Operation::Phi(record))) = result_code.last() {
                    phi_nodes.push((*phi_ret, *record));
                    result_code.pop();
                }
                assert!(*result_code.last().unwrap() == (loop_data.entry_label, Operation::Label));
                result_code.pop();

                let var_map = unroll_loop(&mut result_code, loop_data, phi_nodes, &mut constants, &mut max_id);

            }
            _ => (),

        }
    }

    PipelineIR::construct(result_code, inputs, outputs, uniforms)
}

fn contains_sync(loop_code: &LoopCode) -> bool {
    loop_code.body.iter().any(|x| match x.1 {
        Operation::Sync(_) => true,
        _ => false
    })
}

fn unroll_loop(
    result_code: &mut Vec<Op>,
    loop_data: LoopCode,
    phi_nodes: Vec<(Address, PhiRecord)>,
    constants: &mut ConstantPropagationContext,
    max_id: &mut Address,
) -> HashMap<Address, Address> {

    result_code.push((loop_data.entry_label, Operation::Label));

    let mut new_id = || {
        *max_id+=1;
        *max_id
    };
    let max_iterations = 100;

    let mut address_map = HashMap::new();
    let mut iter = 0;

    // emit first loop phi nodes
    for phi in &phi_nodes {
        address_map.insert(phi.0, phi.1.old);
    }
        for op in &loop_data.body {
            address_map.insert(op.0, op.0);
        }
        for op in &loop_data.continue_code {
            address_map.insert(op.0, op.0);
        }

    loop {

        if max_iterations <= iter {
            // TODO: proper error handling.
            panic!("Code contains loop with sync that couldn't be statically unrolled.");
        }

        let mut condition_operations : Vec<_> = loop_data.condition.clone().into_iter().rev().collect();
        let mut current_label = loop_data.condition_label;

        while let Some(op) = condition_operations.pop() {
            let mut op = op;
            for (from, to) in address_map.iter() {
                replace(&mut op, *from, *to);
            }
            let result_operation = match propagate_constant_operation(constants, &mut condition_operations, op.1, op.0, &mut address_map, &mut current_label) {
                Some(val) => val,
                None => continue,
            };
            result_code.push((op.0, result_operation));
        }

        let condition_op = result_code.last();
        let condition = match condition_op {
            None => false,
            Some(x) => {
                match x.1 {
                    Operation::StoreBool(value) => {
                        !value
                    },
                    Operation::JumpIfElse(..) => {
                        panic!("For loop with sync couldn't be unrolled");
                    }
                    _=> {
                        //println!("iteration: {}", iter);
                        //println!("result: {:#?}", result_code);
                        panic!(format!("Unexpected operation: {:?}", x.1));
                    },
                }
            }
        };

        if condition {
            println!("Finished with {} iterations!", iter);
            break;
        }

        for op in &loop_data.body {
            address_map.insert(op.0, new_id());
        }
        for op in &loop_data.continue_code {
            address_map.insert(op.0, new_id());
        }
        
        // emit body and increment with mapped addresses
        // extremely not efficient: should check only values that are actually in operation as they are known
        let mut body: Vec<_> = loop_data.body.clone().into_iter().chain(loop_data.continue_code.clone().into_iter()).rev().collect();
        let mut current_label = loop_data.body_label;
        while let Some(op) = body.pop() {
            let mut op = op;
            for (from, to) in address_map.iter() {
                replace(&mut op, *from, *to);
            }
            let result_operation = match propagate_constant_operation(constants, &mut body, op.1, op.0, &mut address_map, &mut current_label) {
                Some(val) => val,
                None => continue,
            };
            result_code.push((op.0, result_operation));

        }

        // emit phi and map in address_map
        for phi in &phi_nodes {
            let new_add = address_map[&phi.1.new];
            address_map.insert(phi.0, new_add);
        }

        iter+=1;
    }



    //result_code.extend(phi_nodes.into_iter().rev().map(|(x, y)| (x, Operation::Phi(y))));
    //result_code.extend(loop_data.emit());

    return address_map;
}

