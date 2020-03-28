use super::super::ir::{Address, Op, Operation};
use super::PeekableCode;

#[derive(Debug)]
pub struct IfElseCode {
    pub condition_label: Address,
    pub true_block: Vec<Op>,
    pub false_block: Option<Vec<Op>>,
    pub phi_nodes: Vec<Op>,
    pub if_label: Address,
    pub else_label: Option<Address>,
    pub end_label: Address,
    pub if_jump_end_label: Address,
    pub else_jump_end_label: Option<Address>,
}

pub fn find_if_else<'b, I: std::iter::Iterator<Item = &'b Op>>(
    _ret: Address,
    op: Operation,
    code: &mut PeekableCode<'b, I>,
) -> IfElseCode {
    let (cond, if_label, else_label) = match op {
        Operation::JumpIfElse(x, y, z) => (x, y, z),
        _ => {
            unreachable!();
        }
    };

    let val = code.next().unwrap();
    match val.1 {
        Operation::Label => {
            assert!(if_label == val.0);
        }
        _ => panic!("internal compiler error"),
    }

    let mut true_code = vec![];
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::Label if op.0 == else_label => {
                // found end of block
                break;
            }
            _ => {
                // entry inside a block
                true_code.push(*op);
            }
        }
    }
    assert!(true_code.len() > 0);
    let end_label = match true_code[true_code.len() - 1].1 {
        Operation::Jump(lab) => lab,
        x => {
            println!("error at {:#?}", x);
            panic!();
        }
    };
    let if_jump_end_label = true_code[true_code.len()-1].0;
    true_code.pop();
    println!(
        "labels: start: {}, else: {}, end: {}",
        if_label, else_label, end_label
    );

    let mut else_jump_end_label = None;

    let false_code = if end_label != else_label {
        let mut false_code: Vec<(Address, Operation)> = vec![];
        loop {
            let op = match code.next() {
                None => panic!("internal compiler error"),
                Some(x) => x,
            };
            match op.1 {
                Operation::Label if op.0 == end_label => {
                    break;
                }
                _ => {
                    false_code.push(*op);
                }
            }
        }
        else_jump_end_label = Some(false_code.last().unwrap().0);
        false_code.pop();
        Some(false_code)
    } else {
        None
    };

    let mut phi_nodes = vec![];
    loop {
        let op = code.peek();
        if op.is_none() {
            break;
        }
        let op = op.unwrap();
        match op.1 {
            Operation::Phi(..) => {
                phi_nodes.push(*code.next().unwrap());
            }
            _ => {
                break;
            }
        }
    }

    // println!("end label is {}", end_label);

    // println!("true branch code: {:#?}", true_code);
    // println!("false branch code: {:#?}", false_code);
    // println!("Phi nodes: {:#?}", phi_nodes);

    let ret = IfElseCode {
        condition_label: cond,
        true_block: true_code,
        false_block: false_code,
        phi_nodes,
        if_label,
        else_label: if else_label == end_label {
            None
        } else {
            Some(else_label)
        },
        end_label,
        if_jump_end_label,
        else_jump_end_label,
    };
    println!("{:#?}", ret);
    ret
}
