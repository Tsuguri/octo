
use super::ir::{Address, ConstantValue, Op, Operation, PhiRecord, PipelineIR, replace};

use std::collections::HashMap;

pub type PhiCollection = HashMap<String, PhiRecord>;

pub struct PhiObserver {
    outer_label: Address,
    collection: PhiCollection,
}

impl PhiObserver {
    pub fn store(&mut self, name: &str, new: Address, new_label: Address, old: Address) {
        //println!("storing assignment in label: {}", new_label);
        let record = PhiRecord {
            new,
            label: new_label,
            old,
            old_label: self.outer_label,
        };
        self.collection.insert(name.to_owned(), record);
    }

    pub fn get(&self, name: &str) -> Option<&PhiRecord> {
        self.collection.get(name)
    }
}

pub struct Code {
    pub code: Vec<Op>,
    variables: HashMap<String, Address>,
    constants: HashMap<Address, ConstantValue>,

    phi_observer: Option<PhiObserver>,
    synchronized_nodes: HashMap<Address, Address>,

    counter: usize,
    last_label: Address,
}

impl Code {
    pub fn new() -> Self {
        let mut code = Self::empty();
        let label = code.new_label();
        code.push_with_label(Operation::Label, label);
        code
    }
    pub fn empty() -> Self {
        Code {
            code: vec![],
            variables: HashMap::new(),
            constants: HashMap::new(),
            counter: 0,
            phi_observer: None,
            synchronized_nodes: HashMap::new(),
            last_label: 0,
        }
    }

    pub fn last_label(&self) -> Address {
        self.last_label
    }

    pub fn finish(self) -> PipelineIR {
        PipelineIR::new(self.code)
    }

    pub fn finish_with(self, previous: &PipelineIR) -> PipelineIR {
        PipelineIR::with(self.code, previous)
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }

    pub fn exit(&mut self, value: Address) {
        self.push(Operation::Exit(value, self.last_label));
    }

    pub fn replace_label(&mut self, range: std::ops::Range<usize>, old: Address, new: Address) {
        //println!("replacing in {:#?}, from {} to {}", range, old, new);
        //return;
        for index in range {
            let operation = &mut self.code[index];
            replace(operation, old, new, true);
        }
    }

    pub fn observe_assignments(&mut self) -> Option<PhiObserver> {
        //let tmp = self.phi_assignments.take();
        let tmp2 = self.phi_observer.take();
        //self.phi_assignments = Some(HashMap::new());
        //println!("observing in label: {}", self.last_label);
        self.phi_observer = Some(PhiObserver {
            outer_label: self.last_label,
            collection: HashMap::new(),
        });
        tmp2
    }

    pub fn finish_observing(&mut self, old: Option<PhiObserver>) -> Option<PhiCollection> {
        //let ret = self.phi_assignments.take();
        let ret = self.phi_observer.take();
        //self.phi_assignments = old;
        self.phi_observer = old;
        ret.map(|x| x.collection)
    }

    pub fn new_label(&mut self) -> Address {
        self.counter += 1;
        self.counter
    }
    pub fn push(&mut self, op: Operation) -> Address {
        let lab = self.new_label();
        self.push_with_label(op, lab);
        lab
    }

    pub fn insert_at(&mut self, op: Operation, index: usize) -> Address {
        let lab = self.new_label();
        self.insert_with_label((lab, op), index);
        lab
    }

    pub fn insert_with_label(&mut self, op: Op, index: usize) {
        self.code.insert(index, op);
    }
    pub fn push_with_label(&mut self, op: Operation, label: Address) {
        match &op {
            Operation::Label => {
                self.last_label = label;
            }
            _ => (),
        }
        self.code.push((label, op));
    }

    pub fn store(&mut self, name: &str, add: Address, create: bool) {
        if let Some(assignments) = &mut self.phi_observer {
            // if we create new variable then it doesn't go into phi assignemts (local variable).
            if !create {
                let old = self.variables[name];
                //assignments.insert(name.to_owned(), (add, old));
                assignments.store(name, add, self.last_label, old);

                return;
            }
        }
        self.variables.insert(name.to_owned(), add);
    }
    pub fn get(&self, name: &str) -> Address {
        if let Some(assignments) = &self.phi_observer {
            //println!("Getting phi val for {}", name);
            match assignments.get(name) {
                None => {}
                Some(x) => {
                    return x.new;
                }
            }
        };
        *self.variables.get(name).unwrap()
    }

    pub fn synchronize(&mut self, address: Address) -> Address {
        if let Some(adr) = self.synchronized_nodes.get(&address) {
            *adr
        } else {
            let new_addr = self.push(Operation::Sync(address));
            self.synchronized_nodes.insert(address, new_addr);

            new_addr
        }
    }

    pub fn get_const_address(&self, value: &ConstantValue) -> Option<Address> {
        self.constants.iter().find(|x| *x.1 == *value).map(|x| *x.0)
    }

    pub fn store_constant(&mut self, val: ConstantValue) -> Address {
        let addr = self.get_const_address(&val);
        match addr {
            Some(x) => return x,
            _ => {}
        }
        use ConstantValue::*;
        let address = match val {
            Float(val) => self.push(Operation::StoreFloat(val)),
            Int(val) => self.push(Operation::StoreInt(val)),
            Vec2(val) => self.push(Operation::StoreVec2(val)),
            Vec3(val) => self.push(Operation::StoreVec3(val)),
            Vec4(val) => self.push(Operation::StoreVec4(val)),
            Bool(val) => self.push(Operation::StoreBool(val)),
        };
        self.make_const(address, val);
        address
    }

    pub fn get_const(&self, addr: Address) -> ConstantValue {
        self.constants[&addr]
    }
    pub fn make_const(&mut self, addr: Address, value: ConstantValue) {
        self.constants.insert(addr, value);
    }

    pub fn is_const(&self, addr: Address) -> bool {
        let res = self.constants.contains_key(&addr);
        res
    }
}
