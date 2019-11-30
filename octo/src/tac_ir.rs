mod code;
mod emit_ir_from_ast;
mod emit_spirv;
mod ir;
mod optimalizations;
mod split_passes;
mod emit_builtins;

use super::ast;

pub use emit_ir_from_ast::emit as emit_ir;
pub use emit_spirv::emit_spirv;
pub use optimalizations::*;
pub use split_passes::split as split_passes;
pub use split_passes::{PipelineDef, ShaderDef};

use std::collections::HashMap;

use std::io::Write;

use ir::{Address, Operation, PipelineIR};

/// debug data flow graph
pub fn emit_graph(code: &PipelineIR, path: &str) {
    let mut graph = petgraph::Graph::<String, &str>::new();
    let mut nodes: HashMap<Address, petgraph::graph::NodeIndex<u32>> = HashMap::new();

    let id = graph.add_node("bad node".to_string());
    nodes.insert(0, id);

    for node in code.operations() {
        let id = graph.add_node(node.1.to_string());
        nodes.insert(node.0, id);
    }
    let mut last_label = 0;

    for node in code.operations() {
        let node_idx = nodes[&node.0];
        use Operation::*;
        match &node.1 {
            Store(a) => {
                let l = nodes[a];
                graph.add_edge(l, node_idx, "");
            }
            Add(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Sub(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Mul(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Div(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Less(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            LessEq(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Eq(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Neq(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            And(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Or(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Shift(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Phi(rec) => {
                let label_idx = nodes[&last_label];

                let l = nodes[&rec.new];
                let r = nodes[&rec.old];

                let left_node = graph.add_node("".to_string());
                let right_node = graph.add_node("".to_string());

                let l_label = nodes[&rec.label];
                let r_label = nodes[&rec.old_label];

                graph.add_edge(l, left_node, "");
                graph.add_edge(l_label, left_node, "");
                graph.add_edge(left_node, node_idx, "");

                graph.add_edge(r, right_node, "");
                graph.add_edge(r_label, right_node, "");
                graph.add_edge(right_node, node_idx, "");

                graph.add_edge(label_idx, node_idx, "contains");
            }
            Neg(l) => {
                let l = nodes[l];
                graph.add_edge(l, node_idx, "");
            }
            Exit(l, r) => {
                let l = nodes[l];
                let r = nodes[r];
                graph.add_edge(l, node_idx, "");
                graph.add_edge(r, node_idx, "");
            }
            Sync(l) => {
                let l = nodes[l];
                graph.add_edge(l, node_idx, "");
            }
            JumpIfElse(a, b, c) => {
                let cond = nodes[a];
                let tru = nodes[b];
                let fals = nodes[c];

                graph.add_edge(cond, node_idx, "condition");
                graph.add_edge(node_idx, tru, "true");
                graph.add_edge(node_idx, fals, "false");
            }
            Label => {
                last_label = node.0;
            }
            _ => {}
        }
    }

    let dot = petgraph::dot::Dot::with_config(&graph, &[]);

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(format!("{}", dot).as_bytes()).unwrap();
}
