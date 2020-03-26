use serde::{Deserialize, Serialize};
use serde_yaml;
use std::io::Write;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    Bool,
    Void,
    Unknown,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        use Type::*;
        match *self {
            Float => write!(f, "Float"),
            Vec2 => write!(f, "Vec2"),
            Vec3 => write!(f, "Vec3"),
            Vec4 => write!(f, "Vec4"),
            Int => write!(f, "Int"),
            Bool => write!(f, "Bool"),
            Void => write!(f, "Void"),
            Unknown => write!(f, "Unknown"),

        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Void
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Prototype {
    i: Vec<Type>,
    #[serde(default)]
    o: Type,
}

#[derive(Debug, Serialize, Deserialize)]
enum SpirvCommand {
    Single(u32),
    Dual(u32, u32)
}

#[derive(Debug, Serialize, Deserialize)]
struct Function {
    #[serde(default)]
    prototypes: Vec<Prototype>,
    #[serde(default)]
    pass_through: Vec<Type>,
    name: String,
    #[serde(default)]
    special: bool,
    comm: SpirvCommand,
}

impl Function {
    pub fn params(&self) -> u32 {
        if self.pass_through.len() > 0{
            return 1u32;
        }
        if self.prototypes.len()>0 {
            let first = &self.prototypes[0];
            return first.i.len() as u32;
        }
        return 0;
    }

}
type Funcs = Vec<Function>;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "protos.rs", escape="none")]
struct PrototypesTemplate<'a> {
    data: &'a Funcs,
}

#[derive(Template)] // this will generate the code...
#[template(path = "emit_builtin.rs", escape="none")]
struct BuiltinEmitTemplates<'a> {
    data: &'a Funcs,
}

#[derive(Template)]
#[template(path = "builtin_spirv.rs", escape="none")]
struct SpirvEmitTemplates<'a> {
    data: &'a Funcs,
}

fn main() {
    let in_path = "src/prototypes/protos.yaml";

    let en = SpirvCommand::Dual(2,3);
    println!("{}",serde_yaml::to_string(&en).unwrap());


    let prototypes_list_data = std::fs::read_to_string(&in_path).unwrap();
    let protos: Funcs = serde_yaml::from_str(&prototypes_list_data).unwrap();

    let out_path = "src/prototypes/protos.rs";
    let template = PrototypesTemplate {data: &protos};
    std::fs::write(&out_path, &template.render().unwrap());

    let out_path = "src/tac_ir/emit_builtins.rs";
    let template = BuiltinEmitTemplates{data: &protos};
    std::fs::write(&out_path, &template.render().unwrap());

    let out_path = "src/tac_ir/emit_spirv/emit_std.rs";
    let template = SpirvEmitTemplates{data: &protos};
    std::fs::write(&out_path, &template.render().unwrap());

    println!("cargo:rerun-if-changed=src/prototypes/protos.yaml");
}
