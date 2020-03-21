use std::collections::HashMap;

use super::ir::{Address, ConstantValue, Operation, PhiRecord, PipelineIR};

pub struct ConstantPropagationContext
{
    constants: HashMap<Address, ConstantValue>,
}

impl Default for ConstantPropagationContext {
    fn default() -> Self {
        ConstantPropagationContext {
            constants: Default::default(),
        }
    }
}

impl ConstantPropagationContext {
    pub fn get_const(&self, add: &Address) -> Option<ConstantValue> {
        self.constants.get(add).map(|x| *x)
    }

    pub fn store_const(&mut self, add: Address, value: ConstantValue) {
        self.constants.insert(add, value);
    }

    pub fn copy_const(&mut self, add: Address, value: ConstantValue) -> Operation {
        self.store_const(add, value);
        
        use ConstantValue::*;
        let op = match value {
            Int(v) => Operation::StoreInt(v),
            Float(v) => Operation::StoreFloat(v),
            Vec2(v) => Operation::StoreVec2(v),
            Vec3(v) => Operation::StoreVec3(v),
            Vec4(v) => Operation::StoreVec4(v),
            Bool(v) => Operation::StoreBool(v),
        };
        op

    }
}
