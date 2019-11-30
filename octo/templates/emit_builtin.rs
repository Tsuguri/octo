use super::ir::{
    Address, 
    ConstantValue, 
    Operation, 
    StdFunction,
};
use super::code::{Code};

#[derive(Debug)]
pub enum BuiltinEmitError {
    NameNotFound,
    CompilerError,
}

pub fn emit_builtin(name: &str, args: &Vec<Address>, code: &mut Code) -> Result<Address, BuiltinEmitError> {
    match name { {% for func in data %}
        "{{ func.name }}"=> emit_{{func.name}}(args, code),{% endfor %}
        _=>{
            Result::Err(BuiltinEmitError::NameNotFound)
        }
    }
}

{% for func in data%}
// ignore normal prototypes for now xD
fn emit_{{func.name}}(args: &Vec<Address>, code: &mut Code)-> Result<Address, BuiltinEmitError> {
    {% if func.pass_through.len() > 0 %}
    if args.len() == 1 {
        return Result::Ok(code.push(Operation::Invoke(StdFunction::{{func.name|capitalize}}(args[0]))))
    }
    {% endif %}
    Result::Err(BuiltinEmitError::CompilerError)
}
{% endfor%}
