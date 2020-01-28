use std::collections::HashMap;

use super::ir::{Address, ConstantValue, Operation, PhiRecord, PipelineIR};

use super::code::Code;

struct ConstantPropagationContext
{
    constants: HashMap<Address, ConstantValue>,

}

impl ConstantPropagationContext {
    pub fn get_const(&self, add: &Address) -> Option<ConstantValue> {
        self.constants.get(add).map(|x| *x)
    }

    pub fn store_const(&mut self, add: Address, value: ConstantValue) {
        self.constants.insert(add, value);
    }
}

pub fn propagate_constants(code: PipelineIR) -> PipelineIR {
    let ctx = ConstantPropagationContext {
        constants: Default::default(),
    };

    let result_code: Vec<(Address, Operation)> = code.operations().map(|x| {
        let result_address = x.0;
        use Operation::*;
        let x = x.1;
        let result_operation = match x {
            Arg(id) => x,
            Uniform(id) => x,
            StoreInt(val) => {
                ctx.store_const(result_address, ConstantValue::Int(val));
                x
            }
            StoreFloat(val) => {
                ctx.store_const(result_address, ConstantValue::Float(val));
                x
            }
            StoreVec2(val) => {
                ctx.store_const(result_address, ConstantValue::Vec2(val));
                x
            }
            StoreVec3(val) => {
                ctx.store_const(result_address, ConstantValue::Vec3(val));
                x
            }
            StoreVec4(val) => {
                ctx.store_const(result_address, ConstantValue::Vec4(val));
                x
            }
            StoreBool(val) => {
                ctx.store_const(result_address, ConstantValue::Bool(val));
                x
            }
            Store(addr) => {
                match ctx.get_const(addr) {
                    None => x,
                    Some(val) => {
                        ctx.store_const(result_address, val);
                        
                    }
                }


            },
            _=> x,
        };

        (result_address, result_operation)
    }).collect();



    code
}