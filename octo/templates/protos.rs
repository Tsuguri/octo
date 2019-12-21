use parser::ast::Type;

pub enum PrototypeMatchError{
    NameNotFound,
    NoMatchingPrototype
}
pub fn match_prototype(name: &str, args: &Vec<Type>) -> Result<Type, PrototypeMatchError> {
    match name { {% for func in data %}
        "{{ func.name }}"=> match_{{func.name}}(args),{% endfor %}
        _=>{
            Result::Err(PrototypeMatchError::NameNotFound)
        }
    }
}
pub fn get_prototypes(name: &str) -> Vec<Vec<Type>> {
    match name { {% for func in data %}
        "{{ func.name }}"=> prototypes_{{func.name}}(),{% endfor %}
        _=>{
            return vec![];
        }
    }
}
{% for func in data%}
const PASS_THROUGH_{{func.name|upper}}: [Type; {{func.pass_through.len()}}] = [
{% for typ in func.pass_through %}
        Type::{{typ}},{% endfor %}
];
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
fn match_{{func.name}}(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_{{func.name|upper}}.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }
{% if func.prototypes.len()>0 %}
    'outer: for proto in PROTOTYPES_{{func.name|upper}}.iter() {
        if args.len() == proto.1.len() {
            if args.iter().zip(proto.1.iter()).any(|(x, y)| x!=y){
                continue 'outer;
            }
            return Result::Ok(proto.0);
        }
    }
{% endif %}
    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}
fn prototypes_{{func.name}}() -> Vec<Vec<Type>> {
    let tmp =PASS_THROUGH_{{func.name|upper}}.iter().map(|x| vec![*x]);
{% if func.prototypes.len() > 0 %}
    let tmp = tmp.chain(PROTOTYPES_{{func.name|upper}}.iter().map(|x| x.1.clone()));
{% endif %}
    tmp.collect()
}
{% endfor%}
