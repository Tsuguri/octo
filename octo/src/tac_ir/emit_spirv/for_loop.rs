use super::ir::{Address, Op, Operation};
use super::PeekableCode;

#[derive(Debug)]
pub struct LoopCode {
    pub body: Vec<Op>,
    pub condition: Vec<Op>,
    pub continue_code: Vec<Op>,

    pub body_label: Address,
    pub exit_label: Address,
    pub continue_label: Address,
    pub condition_label: Address,
    pub entry_label: Address,
    pub condition_value: Address,
}

pub fn find_loop<'b, I: std::iter::Iterator<Item = &'b Op>>(
    _ret: Address,
    op: Operation,
    code: &mut PeekableCode<'b, I>,
    current_block: Address,
) -> LoopCode {
    let (cont_label, exit_label) = match op {
        Operation::LoopMerge(x, y) => (x, y),
        _ => {
            unreachable!();
        }
    };

    // jump to cond

    let condition_label = match code.peek() {
        None => unreachable!(),
        Some(x) => match x.1 {
            Operation::Jump(lab) => lab,
            _ => unreachable!(),
        },
    };
    code.next(); // skip jump to cond
    code.next(); // skip label of cond

    let mut condition_code = vec![];
    let mut body_label = Default::default();
    let condition_value;
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::JumpIfElse(cond, body_lab, exit_lab) if exit_lab == exit_label => {
                body_label = body_lab;
                condition_value = cond;
                break;
            }
            _ => {
                // entry inside a block
                condition_code.push(*op);
            }
        }
    }
    let body_label = body_label;

    let v = code.next().unwrap();
    match v.1 {
        Operation::Label => {
            assert!(v.0 == body_label);
        }
        _ => (),
    }

    assert!(condition_code.len() > 0);

    let mut body_code = vec![];
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::Jump(x) if x == cont_label => break,
            _ => {
                // entry inside a block
                body_code.push(*op);
            }
        }
    }

    println!(
        "labels: content {}, continue {}, end {}",
        body_label, cont_label, exit_label
    );
    let next = code.next().unwrap();
    let next_label = match next.1 {
        Operation::Label => next.0,
        _ => {
            panic!("internal compiler error");
        }
    };
    assert!(next_label == cont_label);

    let mut cont_code = vec![];

    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::Jump(x) if x == current_block => break,
            _ => {
                // entry inside a block
                cont_code.push(*op);
            }
        }
    }

    let next = code.next().unwrap();
    let ex_lab = match next.1 {
        Operation::Label => next.0,
        _ => {
            panic!("internal compiler error");
        }
    };

    assert!(exit_label == ex_lab);

    // load continue stuff content here
    // until jump back to loop_merge block

    let ret = LoopCode {
        body: body_code,
        condition: condition_code,
        continue_code: cont_code,

        body_label,
        exit_label,
        continue_label: cont_label,
        condition_label,
        entry_label: current_block,
        condition_value,
    };
    //println!("{:#?}", ret);

    ret
}
