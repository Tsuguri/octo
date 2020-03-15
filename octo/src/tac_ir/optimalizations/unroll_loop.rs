use super::ir::*;
use super::super::utils::{PeekableCode, find_loop, LoopCode};

pub fn unroll_synced_loop(code: PipelineIR) -> PipelineIR {
    let (code, inputs, outputs, uniforms) = code.take();

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
                if contains_sync(&loop_data) {
                    println!("woo");
                }
                result_code.pop();
                result_code.extend(loop_data.emit());

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
