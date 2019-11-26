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
struct Function {
    #[serde(default)]
    prototypes: Vec<Prototype>,
    #[serde(default)]
    pass_through: Vec<Type>,
    name: String,

}
type Funcs = Vec<Function>;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "protos.rs", escape="none")]
struct PrototypesTemplate {
    data: Funcs,
}
fn main() {
    let out_path = "src/prototypes/protos.rs";
    let in_path = "src/prototypes/protos.yaml";


    let prototypes_list_data = std::fs::read_to_string(&in_path).unwrap();
    let protos: Funcs = serde_yaml::from_str(&prototypes_list_data).unwrap();

    let template = PrototypesTemplate {data: protos};

    // write stuff
    std::fs::write(&out_path, &template.render().unwrap());

}