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
// ignore normal prototypes for now xD
fn match_{{func.name}}(args: &Vec<Type>)-> Result<Type, PrototypeMatchError> {
    if args.len() == 1 && PASS_THROUGH_{{func.name|upper}}.iter().find(|x| **x==args[0]).is_some() {
        return Result::Ok(args[0]);
    }
    Result::Err(PrototypeMatchError::NoMatchingPrototype)
}

fn prototypes_{{func.name}}() -> Vec<Vec<Type>> {
    PASS_THROUGH_{{func.name|upper}}.iter().map(|x| vec![*x]).collect()
}
{% endfor%}
