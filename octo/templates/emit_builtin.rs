use super::ir::{
    Address, 
    ConstantValue, 
    Operation, 
    StdFunction,
};
use super::code::{Code};
use super::special_builtins as sb;
use sb::BuiltinEmitError;

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
    {% if func.special %}
    return sb::emit_{{func.name}}_special(args, code);
    {% else %}
        {% if func.pass_through.len() > 0 %}
        if args.len() == 1 {
            return Result::Ok(code.push(Operation::Invoke(StdFunction::{{func.name|capitalize}}(args[0]))))
        }
        {% endif %}
        {% if func.prototypes.len() > 0%}
        {% let first = func.prototypes[0] %}
        if args.len() == {{first.i.len()}} {
           return Result::Ok(code.push(Operation::Invoke(StdFunction::{{func.name|capitalize}}(
               {%for arg in 0..first.i.len()%}args[{{arg}}],{% endfor %}
           ))))
        }
        {% endif %}
    {% endif %}
    Result::Err(BuiltinEmitError::CompilerError)
}
{% endfor%}
