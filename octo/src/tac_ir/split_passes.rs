use octo_runtime as or;

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

impl std::convert::Into<or::InputType> for InputTexture {
    fn into(self) -> or::InputType {
        match self {
            InputTexture::Arg(x)=> or::InputType::ProvidedTexture(x),
            InputTexture::Generated(x)=> or::InputType::PipelineTexture(x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutputTexture {
    Result,
    Generated(Vec<usize>),
}

impl std::convert::Into<or::OutputType> for OutputTexture {
    fn into(self) -> or::OutputType {
        match self {
            OutputTexture::Result=> or::OutputType::Result,
            OutputTexture::Generated(x)=> or::OutputType::Textures(x),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderPass {
    pub shader_id: usize,
    pub input: Vec<InputTexture>,
    pub output: OutputTexture,
    pub dependencies: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct PipelineDef {
    pub shaders: Vec<ShaderDef>,
    pub passes: Vec<ShaderPass>,
    pub textures: Vec<ValueType>,
    pub args: Vec<(ValueType, String)>,
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
        input_type: inputs.iter().map(|x| x.0).collect(),
        output_type: outputs,
    };
    println!("outputs: {}", outputs_num);
    let the_only_pass = ShaderPass{
        shader_id: 0,
        input: (0..inputs_num).map(|x| InputTexture::Arg(x)).collect(),
        output: OutputTexture::Result,
        dependencies: Option::None,
    };


    PipelineDef{
        shaders: vec![the_only_shader],
        passes: vec![the_only_pass],
        textures: vec![],
        args: inputs,
    }
}