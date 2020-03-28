use super::super::ir::*;
use super::*;
use spirv_headers::Word as SpirvAddress;
use parser::ast::Type;

pub fn emit_std_function<'a, I: std::iter::Iterator<Item=&'a Op>>(func: StdFunction, ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress
{
    use StdFunction::*;
    return match func { {% for func in data %}{% let params = func.params() %}
        {{func.name|capitalize}}({% for i in 0..params%}data_{{i}},{%endfor%}) => emit_{{func.name}}({% for i in 0..params%}data_{{i}},{%endfor%} ret_addr, emitter),{% endfor %}
    };
}

{% for func in data%}
{% if func.pass_through.len() > 0%}
const PASS_THROUGH_{{func.name|upper}}: [Type; {{func.pass_through.len()}}] = [
{% for typ in func.pass_through %}
        Type::{{typ}},{% endfor %}
];
{% endif %}
{% if func.prototypes.len() > 0%}
lazy_static::lazy_static! {
    static ref PROTOTYPES_{{func.name|upper}}: Vec<(Type, Vec<Type>)> = {
        let mut m = Vec::with_capacity({{func.prototypes.len()}});
{% for protos in func.prototypes%}
        {
            let p = vec![
                {% for input in protos.i %}Type::{{input}},{% endfor %}
            ];
            m.push((Type::{{protos.o}}, p));
}{% endfor %}
        m
    };
}
{% endif %}
{% let params = func.params() %}
fn emit_{{func.name}}<'a, I: std::iter::Iterator<Item=&'a Op>>({% for i in 0..params%}data_{{i}}: Address,{%endfor%} ret_addr: Address, emitter: &mut MainEmitter<'a, I>) -> SpirvAddress{
    {% for i in 0..params %}
    let data_{{i}}_type = emitter.get_single_type(data_{{i}});{% endfor %}
    
    let args =[{% for i in 0..params %}data_{{i}}_type, {% endfor %} ];
    println!("emitting {{func.name}} of {:?}", data_0_type);


    {% if func.pass_through.len() > 0  && params == 1 %}
    let id = {% match func.comm %}
        {% when SpirvCommand::Single with (id) %}
            {{id}};
        {% when SpirvCommand::Dual with (int_id, float_id)%}
            if data_0_type == ValueType::Int {
                {{int_id}}
            } else {
                {{float_id}}
            };
    {% endmatch %}
    if PASS_THROUGH_{{func.name|upper}}.contains(&data_0_type) {
        return emitter.emit_passthrough(id, data_0, ret_addr);
    }
    {% endif %}
    {% if func.prototypes.len() > 0 %}
    for proto in PROTOTYPES_{{func.name|upper}}.iter() {
        if proto.1.len() != {{params}} {
            panic!();
        }

        if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
            continue;
        }

        // this is the one
        let result_type = proto.0;
        let id = {% match func.comm %}
            {% when SpirvCommand::Single with (id) %}
            {{id}};
            {% when SpirvCommand::Dual with (int_id, float_id)%}
            if result_type == ValueType::Int {
                {{int_id}}
            } else {
                {{float_id}}
            };
        {% endmatch %}
        {% if func.is_dot() %}
        return emitter.emit_dot_instruction(data_0, data_1, ret_addr);
        {% else %}
        return emitter.emit_prototyped(id, &[{% for i in 0..params%}data_{{i}},{%endfor%}],ret_addr, result_type);
        {% endif%}
    }
    panic!();
    {% endif %}
    return 0;
}
{% endfor%}
