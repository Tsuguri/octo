use octo_runtime as or;
use std::collections::{HashMap, HashSet};

use super::ir::{Address, Op, Operation, PipelineIR, ValueType, PhiRecord};
use super::utils::{find_loop, LoopCode, PeekableCode};

#[derive(Debug, Clone)]
pub struct ShaderDef {
    pub input_type: Vec<ValueType>,
    pub output_type: Vec<ValueType>,
    pub code: Vec<Op>,
}

#[derive(Debug, Copy, Clone)]
pub enum InputTexture {
    Arg(usize),
    Generated(usize),
}

impl std::convert::Into<or::InputType> for InputTexture {
    fn into(self) -> or::InputType {
        match self {
            InputTexture::Arg(x) => or::InputType::ProvidedTexture(x),
            InputTexture::Generated(x) => or::InputType::PipelineTexture(x),
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
            OutputTexture::Result => or::OutputType::Result,
            OutputTexture::Generated(x) => or::OutputType::Textures(x),
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
    pub uniforms: Vec<(ValueType, String)>,
}

pub fn split(program: PipelineIR) -> PipelineDef {
    let (operations, inputs, outputs, uniforms) = program.take();

    let mut syncs = find_syncs(&operations);

    if syncs.len() == 0 {
        let inputs_num = inputs.len();
        let outputs_num = outputs.len();

        let the_only_shader = ShaderDef {
            code: operations,
            input_type: inputs.iter().map(|x| x.0).collect(),
            output_type: outputs,
        };

        //println!("outputs: {}", outputs_num);
        let the_only_pass = ShaderPass {
            shader_id: 0,
            input: (0..inputs_num).map(|x| InputTexture::Arg(x)).collect(),
            output: OutputTexture::Result,
            dependencies: Option::None,
        };

        return PipelineDef {
            shaders: vec![the_only_shader],
            passes: vec![the_only_pass],
            textures: vec![],
            args: inputs,
            uniforms
        };
    }

    let dependencies = prepare_dependencies(&operations);

    let types = check_types(&operations, &inputs, &uniforms);

    let last_op = operations.last().unwrap();
    let exit_value = match last_op.1 {
        Operation::Exit(val, _) => val,
        x => {
            panic!(format!("Last operation is not exit, but: {:?}", x));
        }
    };

    syncs.push((last_op.0, exit_value, operations.len()-1));

    let mut programs: Vec<Vec<(Address, Operation)>> = Vec::with_capacity(syncs.len() + 1);

    for (sync_operation, synced_value, index) in &syncs {
        //println!("generating program for syncing {}", synced_value);
        let mut used = HashSet::new();
        let mut to_check = Vec::new();
        to_check.push(*synced_value);

        while let Some(addr) = to_check.pop() {
            if used.contains(&addr) {
                continue;
            }
            used.insert(addr);
            //println!("checking {}", addr);
            match dependencies.get(&addr) {
                None => (),
                Some(val) => to_check.extend_from_slice(val),
            }
        }

        let new_program = operations.iter().filter(|x| used.contains(&x.0)).cloned().collect();

        //println!("program generating {}: {:?}", synced_value, new_program);
        programs.push(new_program);
    }
    //syncs.pop();

    let mut shaders: Vec<ShaderDef> = Vec::with_capacity(programs.len());
    let mut shader_passes: Vec<ShaderPass> = Vec::with_capacity(programs.len());

    for (id, program) in programs.iter().enumerate() {
        let mut deps = Vec::new();
        let program_inputs: Vec<_> = program.iter().filter_map(|x| {
            match x.1 {
                Operation::Arg(y) => {
                    println!("looking for type for: {}", y);
                    let t = types[&x.0];
                    Some((InputTexture::Arg(y), t))
                },
                Operation::Sync(val) => {
                    let sc = syncs.iter().enumerate().find(|(id, s)| s.1 == val);
                    match sc {
                        None => panic!("Internal compiler error"),
                        Some(t) => {
                            let typ = types[&(t.1).1];
                            deps.push(t.0);
                            Some((InputTexture::Generated(t.0), typ))
                        }
                    }
                }
                _=> None
            }
        }).collect();
        let (t, ret) = if id ==syncs.len()-1 {
            (*outputs.first().unwrap(), OutputTexture::Result)
        } else {
            let sc = syncs[id];
            (types[&sc.1], OutputTexture::Generated(vec![id]))
        };

        let shader_inputs: Vec<_> = program_inputs.iter().map(|x| x.1).collect();
        let program_in: Vec<_> = program_inputs.iter().map(|x| x.0).collect();
        let mut last_label = 0;

        let mut shader_code: Vec<_> = std::iter::once((1, Operation::Label)).chain(program.iter().map(|x| {
            let op = match x.1 {
                Operation::Label => {
                    last_label = x.0;
                    x.1
                }
                Operation::Arg(num)=>{
                    let id = program_in.iter().enumerate().find(|(id, value)| {
                        match value {
                            InputTexture::Arg(num2) if num==*num2 => true,
                            _ => false,
                        }
                    }).unwrap();
                    Operation::Arg(id.0)

                },
                Operation::Sync(addr) =>{
                    let sc = syncs.iter().enumerate().find(|(id, s)| s.1 == addr);
                    let sc_id = match sc {
                        None => panic!("Internal compiler error"),
                        Some(t) => {
                            t.0
                        }
                    };
                    let id = program_in.iter().enumerate().find(|(id, value)| {
                        match value {
                            InputTexture::Generated(num2) if sc_id==*num2 => true,
                            _ => false,
                        }
                    }).unwrap();
                    Operation::Arg(id.0)
                },
                a => a
            };
            (x.0, op)
        })).collect();
        shader_code.push((shader_code.last().unwrap().0 + 1, Operation::Exit(syncs[id].1, last_label)));

        println!("Generated shader: {:#?}", shader_code);
        


        shaders.push(ShaderDef{
            code: shader_code,
            input_type: shader_inputs,
            output_type: vec![t],
        });

        shader_passes.push(ShaderPass{
            shader_id: id,
            input: program_inputs.iter().map(|x| x.0).collect(),
            output: ret,
            dependencies: if deps.len() == 0 {None} else {Some(deps)},
        });

    }
    syncs.pop();

    let textures = syncs.iter().map(|x| types[&x.1]).collect();
    let outputs_num = outputs.len();

    //println!("outputs: {}", outputs_num);

    return PipelineDef {
        shaders: shaders,
        passes: shader_passes,
        textures: textures,
        args: inputs,
        uniforms
    };
}

// Address of operation, address of synced value and index in operations vector
fn find_syncs(program: &Vec<(Address, Operation)>) -> Vec<(Address, Address, usize)> {
    program.iter().enumerate().filter_map(|(id, elem)| {
        match elem.1 {
            Operation::Sync(x) => Some((elem.0, x, id)),
            _ => None
        }
    }).collect()
}

fn prepare_dependencies(program: &Vec<(Address, Operation)>) -> HashMap<Address, Vec<Address>> {
    let mut usage = HashMap::new();

    for op in program.iter().rev() {
        use Operation::*;
        let ret_addr = op.0;
        let op = op.1;
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
            Sync(a) => {
                // doing nothing as we have Sync nodes specified and will be building dependency trees starting from synced values
            },
            Store(a) => {usage.insert(ret_addr, vec![a]);},
            Shift(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            ExtractComponent(a,..) => {usage.insert(ret_addr,vec![a]);},
            StoreComponent(a, .., b) => {usage.insert(ret_addr,vec![a, b]);},
            ConstructVec2(a,b) => {usage.insert(ret_addr,vec![a,b]);},
            ConstructVec3(a,b,c) => {usage.insert(ret_addr,vec![a,b,c]);},
            ConstructVec4(a,b,c,d) => {usage.insert(ret_addr,vec![a,b,c,d]);},
            Jump(..) => (),
            JumpIfElse(a, ..) => {
                
                usage.insert(ret_addr, vec![a]);
            },
            LoopMerge(..) => {
            },
            Invoke(op) => {usage.insert(ret_addr,op.deps());},
            Phi(PhiRecord{new, old, ..}) =>{usage.insert(ret_addr, vec![new, old]);},

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

    find_loops_dependencies(program, &mut usage);
    find_if_else_dependencies(program, &mut usage);

    usage
}

fn find_loops_dependencies(program: &Vec<(Address, Operation)>, deps: &mut HashMap<Address, Vec<Address>>) {
    let mut peekable = PeekableCode::new(program.iter());
    let mut last_label = 0;
    let mut result_code = Vec::new();
    while let Some((ret, op_code)) = peekable.next().copied() {
        result_code.push((ret, op_code));
        match op_code {
            Operation::Label => last_label = ret,
            Operation::LoopMerge(..) => {
                let loop_data = find_loop(ret, op_code, &mut peekable, last_label);

                let mut phi_nodes = Vec::new();
                while let Some((phi_ret, Operation::Phi(record))) = result_code.last() {
                    phi_nodes.push((*phi_ret, *record));
                    result_code.pop();
                }
                assert!(*result_code.last().unwrap() == (loop_data.entry_label, Operation::Label));
                result_code.pop();
                phi_nodes.iter().for_each(|elem| {
                    deps.get_mut(&elem.0).unwrap().push(loop_data.loop_merge_label);
                });
                mark_loop_dependencies(ret, &loop_data, deps);
                find_loops_dependencies(&loop_data.body, deps);
                result_code.clear();
            }
            _ => (),

        }
    }
}

fn mark_loop_dependencies(loop_address: Address, loop_data: &LoopCode, deps: &mut HashMap<Address, Vec<Address>>) {

    let dependencies = vec![
        loop_data.entry_label,
        loop_data.exit_label,
        loop_data.condition_jump_label,
        loop_data.condition_label,
        loop_data.condition_check_label,
        loop_data.body_label,
        loop_data.jump_cont_label,
        loop_data.continue_label,
        loop_data.jump_start_label,
        loop_data.exit_label,
    ];
    
    deps.insert(loop_address, dependencies);
}

fn find_if_else_dependencies(program: &Vec<(Address, Operation)>, deps: &mut HashMap<Address, Vec<Address>>) {
}

fn check_types(operations: &Vec<(Address, Operation)>, input_types: &Vec<(ValueType, String)>, uniforms: &Vec<(ValueType, String)>) -> HashMap<Address, ValueType> {
    let mut types: HashMap<Address, ValueType> = HashMap::new();

    for (ret_addr, op) in operations {
        use Operation::*;
        let typ = match op {
            Arg(num) => input_types[*num].0,
            Uniform(num) => uniforms[*num].0,
            StoreInt(_) => ValueType::Int,
            StoreFloat(_) => ValueType::Float,
            StoreVec2(_) => ValueType::Vec2,
            StoreVec3(_) => ValueType::Vec3,
            StoreVec4(_) => ValueType::Vec4,
            StoreBool(_) => ValueType::Bool,
            Store(addr) => types[&addr],
            ConstructVec2(..) => ValueType::Vec2,
            ConstructVec3(..) => ValueType::Vec3,
            ConstructVec4(..) => ValueType::Vec4,
            StoreComponent(to, ..) => types[&to],
            ExtractComponent(..) => ValueType::Float,
            Add(a, b) => types[&a],
            Sub(a, b) => types[&a],
            Div(a, b) => types[&a],
            Mul(a, b) => {
                let a_type = types[&a];
                let b_type = types[&b];
                use ValueType::*;
                match (a_type, b_type) {
                    (a,b) if a == b => a,
                    (Vec2, Float) => Vec2,
                    (Vec3, Float) => Vec3,
                    (Vec4, Float) => Vec4,
                    (Mat3, Vec3) => Vec3,
                    (Mat4, Vec4) => Vec4,
                    _ => panic!(),
                }
            },
            Less(..) => ValueType::Bool,
            LessEq(..) => ValueType::Bool,
            Eq(..) => ValueType::Bool,
            Neq(..) => ValueType::Bool,
            And(..) => ValueType::Bool,
            Or(..) => ValueType::Bool,
            Neg(val) => types[&val],
            Label => continue,
            Exit(val,..) => types[&val],
            Sync(a) => types[&a],
            Shift(a, ..) => types[&a],
            Phi(record) => types[&record.old],
            Jump(..) => continue,
            JumpIfElse(..) => continue,
            LoopMerge(..) => continue,
            Invoke(func) => {
                let deps = func.deps();
                types[&deps[0]]
            }

        };
        types.insert(*ret_addr, typ);

    }

    types
}
