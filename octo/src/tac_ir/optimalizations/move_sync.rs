use super::ir::{PipelineIR, Operation};

pub fn move_sync_operations(ir: PipelineIR) -> PipelineIR{

    let (mut code, inputs, outputs, uniforms) = ir.take();

    let syncs: Vec<_> = code.iter().enumerate().filter_map(|x| {
        match (x.1).1 {
            Operation::Sync(val) => Some((x.0,val)),
            _ => None,
        }
    }).collect();

    for (id, synced_value) in syncs {
        let mut id = id;
        println!("Syncing value: {}, starting from: {}", synced_value, id);

        while id >=1  && code[id-1].0 != synced_value {
            println!("not equal: {}, op: {}", synced_value, code[id-1].0);
            code.swap(id, id-1);
            id-=1;
        }
        if id == 0 {
            panic!("synced value was not found... Internal compiler error");
        }

        println!("finished at line: {}", id);
    }

    let mut ir = PipelineIR::new(code);
    ir.outputs = outputs;
    ir.inputs = inputs;
    ir.uniforms = uniforms;
    ir
}
