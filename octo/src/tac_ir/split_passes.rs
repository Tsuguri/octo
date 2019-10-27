use super::ir::{
    PipelineIR,
    Operation,
    Op,
    Address,
    ValueType,
};

#[derive(Debug, Clone)]
pub struct ShaderDef {
    input_type: Vec<ValueType>,
    output_type: Vec<ValueType>,
    code: Vec<Op>,
}

#[derive(Debug, Copy, Clone)]
pub enum InputTexture {
    Arg(usize),
    Generated(usize),
}

#[derive(Debug, Copy, Clone)]
pub enum OutputTexture {
    Result(usize),
    Generated(usize),
}

#[derive(Debug, Clone)]
pub struct ShaderPass {
    shader_id: usize,
    input: Vec<InputTexture>,
    output: Vec<OutputTexture>,
    dependencies: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct PipelineDef {
    pub shaders: Vec<ShaderDef>,
    pub passes: Vec<ShaderPass>,
}

pub fn split(program: PipelineIR) -> PipelineDef {
    let (operations, inputs, outputs) = program.take();

    // for now sync is not supported for simplicity
    // it means that there is only one output shader
    for instruction in &operations {
        match instruction.1 {
            Operation::Sync(..) | Operation::Shift(..)=> {
                panic!("Octo is not yet able to emit code containing synchronization");
            }
            _=>(),
        }
    }

    // define required input
    // define passes -> take from IR input?
    // dependencies: not now
    let inputs_num = inputs.len();
    let outputs_num = outputs.len();

    let the_only_shader = ShaderDef{
        code: operations,
        input_type: inputs,
        output_type: outputs,
    };
    let the_only_pass = ShaderPass{
        shader_id: 0,
        input: (0..inputs_num).map(|x| InputTexture::Arg(x)).collect(),
        output: (0..outputs_num).map(|x| OutputTexture::Result(x)).collect(),
        dependencies: Option::None,
    };


    PipelineDef{
        shaders: vec![the_only_shader],
        passes: vec![the_only_pass],
    }
}