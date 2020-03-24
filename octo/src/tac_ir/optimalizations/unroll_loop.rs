use super::ir::*;
use super::super::utils::{PeekableCode, find_loop, LoopCode};
use super::ConstantPropagationContext;
use super::constants_propagation::propagate_constant_operation;

use std::collections::HashMap;

pub fn unroll_synced_loop(code: PipelineIR) -> PipelineIR {
    let (code, inputs, outputs, uniforms) = code.take();

    let mut constants = ConstantPropagationContext::default();
    let mut max_id = code.iter().map(|x| x.0).max().unwrap();
    //println!("max id is {}", max_id);

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

    let mut address_map = HashMap::new();


    let mut last_label = 0;

    while let Some((ret, op_code)) = peekable.next().copied() {
        let mut orig = (ret, op_code);
        let original = (ret, op_code);
        for (from, to) in address_map.iter() {
            replace(&mut orig, *from, *to, false);
        }
        //println!("not unroll: mapped {:?} into {:?}", original, orig);
        result_code.push(orig);
        match op_code {
            Operation::Label => last_label = ret,
            Operation::LoopMerge(..) => {
                let loop_data = find_loop(ret, op_code, &mut peekable, last_label);
                result_code.pop();
                if !contains_sync(&loop_data) {
                    result_code.extend(loop_data.emit());
                    continue;
                }
                // current op: LoopMerge
                let mut phi_nodes = Vec::new();
                while let Some((phi_ret, Operation::Phi(record))) = result_code.last() {
                    phi_nodes.push((*phi_ret, *record));
                    result_code.pop();
                }
                assert!(*result_code.last().unwrap() == (loop_data.entry_label, Operation::Label));
                result_code.pop();

                let var_map = unroll_loop(&mut result_code, loop_data, phi_nodes, &mut constants, &mut max_id);
                address_map.extend(var_map);

            }
            _ => (),

        }
    }
    //println!("final address map: {:?}", address_map);

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
        //println!("Inserting phi: {} with value of {}, constant: {}", phi.0, phi.1.old, constants.get_const(&phi.1.old).is_some());

        address_map.insert(phi.0, phi.1.old);
    }

    let mut label = loop_data.entry_label;

    loop {

        if max_iterations <= iter {
            // TODO: proper error handling.
            panic!("Code contains loop with sync that couldn't be statically unrolled.");
        }

        let mut condition_operations : Vec<_> = loop_data.condition.clone().into_iter().rev().collect();

        while let Some(op) = condition_operations.pop() {
            let mut op = op;

            for (from, to) in address_map.iter() {
                replace(&mut op, *from, *to, false);
            }
            let new_addr = new_id();
            address_map.insert(op.0, new_addr);
            op.0 = new_addr;

            let new_op = match propagate_constant_operation(constants, &mut condition_operations, op.1, op.0, &mut address_map, &mut label) {
                None => continue,
                Some(ops) => ops,
            };
            result_code.push((new_addr, new_op));
        }

        let condition_op = result_code.last().unwrap();
        let condition_met = match condition_op.1 {
            Operation::StoreBool(value) => value,
            Operation::JumpIfElse(..) => {
                panic!("For loop with sync couldn't be unrolled");
            }
            _=> {
                //println!("iteration: {}", iter);
                //println!("result: {:#?}", result_code);
                panic!(format!("Unexpected operation: {:?}", condition_op.1));
            },
        };

        if !condition_met {
            //println!("Finished with {} iterations!", iter);
            break;
        }

        let mut operations: Vec<_> = loop_data.body.clone().into_iter().chain(loop_data.continue_code.clone().into_iter()).rev().collect();
        while let Some(op) = operations.pop() {
            let mut op = op;

            for (from, to) in address_map.iter() {
                replace(&mut op, *from, *to, false);
            }
            let new_addr = new_id();
            address_map.insert(op.0, new_addr);

            let new_op = match propagate_constant_operation(constants, &mut operations, op.1, new_addr, &mut address_map, &mut label) {
                None => continue, // should never happen here? maybe?
                Some(ops) => ops,
            };
            result_code.push((new_addr, new_op));
        }

        for phi in &phi_nodes {
            let new_addr = address_map[&phi.1.new];
            address_map.insert(phi.0, new_addr);
        }



        /*

        // emit phi and map in address_map
        for phi in &phi_nodes {
            let new_add = address_map[&phi.1.new];
            println!("Phi {} is now {}", phi.0, new_add);
            address_map.insert(phi.0, new_add);
            
        }

        label = current_label;
        */
        iter+=1;
    }

    //println!("label on exit: {} into {}", loop_data.exit_label, label);
    address_map.insert(loop_data.exit_label, label);

    return address_map;
}

