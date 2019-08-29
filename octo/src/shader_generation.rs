pub use askama::Template;

use log::{trace, error};
use parser::ast;
pub use shaderc::ShaderKind as Shader;


static VERTEX: &str = include_str!("basic_vertex.glsl");

#[derive(Template)] // this will generate the code...
#[template(path = "shader.glsl", escape = "none")] // using the template in this path, relative
// to the templates dir in the crate root
struct ShaderTemplate<'a> {
    pub code: &'a str,
    pub input: &'a [ast::Variable],
    pub output: &'a [ast::Variable],
}
pub fn construct_function(function: &ast::GpuFunction) -> Vec<u32> {

    let template = ShaderTemplate {
        input: &function.arguments,
        output: &function.results,
        code: &function.code.val,
    };

    let fragment_code = template.render().unwrap();
    trace!("{}", fragment_code);
    process_glsl(&fragment_code, &function.name.val, Shader::Fragment)
}

pub fn basic_vertex() -> Vec<u32> {
    process_glsl(VERTEX, "basic vertex", Shader::Vertex)
}

fn process_glsl(code: &str, path: &str, shader_type: Shader) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!").unwrap();
    let compilation_result = compiler
        .compile_into_spirv(
            &code,
            shader_type,
            &path,
            "main",
            None,
        )
        .map_err(|e| {
            error!("{}", e);
            "Couldn't compile fragment shader!"
        }).unwrap();
    compilation_result.as_binary().to_vec()
}
