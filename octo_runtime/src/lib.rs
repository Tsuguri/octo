use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
}

impl OctoModule {
    pub fn new() -> Self {
        OctoModule{ name: "test_module".to_owned(), version: 0u32}
    }
}

pub fn lolz() {
    println!("lolz");
}
