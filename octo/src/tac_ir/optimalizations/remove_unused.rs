use std::collections::{HashMap, HashSet};

use super::super::*;
use super::ir::*;
//use super::super::super::ast;

fn is_label(op: &Operation) -> bool {
    match op {
        Operation::Label => true,
        _ => false,
    }
}

pub fn remove_unused_operations(code: PipelineIR) -> PipelineIR {

    let (code, inputs, outputs, uniforms) = code.take();

    let mut usage: HashMap<Address, Vec<Address>> = HashMap::new();

    for (ret_addr, op) in &code {
        let ret_addr = *ret_addr;
        let op = *op;
        use Operation::*;
        match op {
            Add(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Sub(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Mul(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Div(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Less(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            LessEq(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            And(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Or(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Eq(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Neq(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            Neg(a) => {usage.insert(ret_addr, vec![a]);},
            Sync(a) => {usage.insert(ret_addr,vec![a]);},
            Store(a) => {usage.insert(ret_addr, vec![a]);},
            Shift(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            ExtractComponent(a,..) => {usage.insert(ret_addr,vec![a]);},
            StoreComponent(a, .., b) => {usage.insert(ret_addr,vec![a, b]);},
            ConstructVec2(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            ConstructVec3(a,b,c) => {usage.insert(ret_addr,vec![a,b,c]);},
            ConstructVec4(a,b,c,d) => {usage.insert(ret_addr,vec![a,b,c,d]);},

            Jump(..) => (),
            JumpIfElse(a, ..) => {usage.insert(ret_addr, vec![a]);},
            LoopMerge(..) => (),
            Invoke(op) => {usage.insert(ret_addr,op.deps());},
            Phi(PhiRecord{new, label, old, old_label}) =>{usage.insert(ret_addr, vec![new, old]);},

            Exit(a, ..) => {usage.insert(ret_addr, vec![a]);},

            Label => (),
            Arg(..) => (),
            Uniform(..) => (),
            StoreInt(..) => (),
            StoreFloat(..) => (),
            StoreVec2(..) => (),
            StoreVec3(..) => (),
            StoreVec4(..) => (),
            StoreBool(..) => (),

        }
    }

    println!("usage generated");

    let mut used = HashSet::new();
    let mut to_check = Vec::new();
    to_check.push(code[code.len() -1].0);
    let usage = usage;

    while let Some(elem) = to_check.pop() {
        println!("checking id {}", elem);
        if used.contains(&elem) {
            continue;
        }
        used.insert(elem);

        if usage.contains_key(&elem) {
            for dependency in &usage[&elem] {
                to_check.push(*dependency);
            }
        }
    }

    let mut res_code = Vec::with_capacity(code.len());
    println!("emitting code without unused");

    for (ret_addr, op) in code {
        use Operation::*;
        let should_copy = match op {
            Jump(..) => true,
            JumpIfElse(..) => true,
            LoopMerge(..) => true,
            Exit(..) => true,
            Label => true,
            _ => {
                used.contains(&ret_addr)
            }
        };

        if should_copy {
            res_code.push((ret_addr, op));
        }

    }



    PipelineIR::construct(res_code, inputs, outputs, uniforms)
}
