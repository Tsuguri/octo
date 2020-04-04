use super::super::ir::{Address, Op, Operation};
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

    pub loop_merge_label: Address,
    pub condition_jump_label: Address,
    pub condition_check_label: Address,
    pub jump_cont_label: Address,
    pub jump_start_label: Address,
    pub entry_label_jump: Address,
}

impl LoopCode {
    pub fn emit(self) -> impl std::iter::Iterator<Item = Op> {
        let mut ops = Vec::new();
        ops.push((self.loop_merge_label, Operation::LoopMerge(self.continue_label, self.exit_label)));
        ops.push((self.condition_jump_label, Operation::Jump(self.condition_label)));
        ops.push((self.condition_label, Operation::Label));
        ops.extend(self.condition.into_iter());
        ops.push((self.condition_check_label, Operation::JumpIfElse(self.condition_value, self.body_label, self.exit_label)));
        ops.push((self.body_label, Operation::Label));
        ops.extend(self.body.into_iter());
        ops.push((self.jump_cont_label, Operation::Jump(self.continue_label)));
        ops.push((self.continue_label, Operation::Label));
        ops.extend(self.continue_code.into_iter());
        ops.push((self.jump_start_label, Operation::Jump(self.entry_label)));
        ops.push((self.exit_label, Operation::Label));

        ops.into_iter()
    }
}

pub fn find_loop<'b, I: std::iter::Iterator<Item = &'b Op>>(
    ret: Address,
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

    let (condition_label, cond_jump_label) = match code.peek() {
        None => unreachable!(),
        Some(x) => match x.1 {
            Operation::Jump(lab) => (lab, x.0),
            _ => unreachable!(),
        },
    };
    code.next(); // skip jump to cond
    code.next(); // skip label of cond

    let mut condition_code = vec![];
    let body_label;
    let condition_value;
    let condition_check_label;
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::JumpIfElse(cond, body_lab, exit_lab) if exit_lab == exit_label => {
                body_label = body_lab;
                condition_value = cond;
                condition_check_label = op.0;
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
    println!("v: {:?}, body_label: {}",v, body_label);
    match v.1 {
        Operation::Label => {
            assert!(v.0 == body_label);
        }
        _ => (),
    }

    assert!(condition_code.len() > 0);

    let mut body_code = vec![];
    let mut jump_cont_label;
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::Jump(x) if x == cont_label => {
                jump_cont_label = op.0;
                break;
            },
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

    let jump_start_label;
    loop {
        let op = match code.next() {
            None => panic!("internal compiler error"),
            Some(x) => x,
        };
        match op.1 {
            Operation::Jump(x) if x == current_block => {
                jump_start_label = op.0;
                break;
            },
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
        loop_merge_label: ret,
        condition_jump_label: cond_jump_label,
        condition_check_label,
        jump_cont_label,
        jump_start_label,
        entry_label_jump: 0,
    };
    //println!("{:#?}", ret);

    ret
}
