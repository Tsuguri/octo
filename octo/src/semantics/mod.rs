use errors::SemanticError;
use errors::SemanticWarning;

pub mod env;

use parser::codespan_reporting;
use parser::codespan_reporting::diagnostic::Label;
type Diagnostic = parser::codespan_reporting::diagnostic::Diagnostic<usize>;

pub struct WarningWrap(pub SemanticWarning);

pub struct ErrorWrap(pub SemanticError);

impl WarningWrap {
    pub fn new(w: SemanticWarning) -> WarningWrap {
        WarningWrap(w)
    }
}

impl ErrorWrap {
    pub fn new(w: SemanticError) -> ErrorWrap {
        ErrorWrap(w)
    }
}

impl From<WarningWrap> for Diagnostic {
    fn from(w: WarningWrap) -> Diagnostic {
        match w.0 {
            SemanticWarning::NotUsedArgument(span, name) => Diagnostic::warning()
                .with_message(format!("Argument \"{}\" is not used.", name))
                .with_labels(vec![Label::primary(0, span)]),
            SemanticWarning::UnusedVariable(span, name) => Diagnostic::warning()
                .with_message(format!("Variable \"{}\" is not used.", name))
                .with_labels(vec![Label::primary(0, span)]),
        }
    }
}

impl From<ErrorWrap> for Diagnostic {
    fn from(w: ErrorWrap) -> Diagnostic {
        match w.0 {
            SemanticError::UndefinedIdentifier(span, name) => Diagnostic::error()
                .with_message(format!("Unknown variable \"{}\"", name))
                .with_labels(vec![
                    Label::primary(0, span).with_message("Unknown identifier used here")
                ]),
            SemanticError::TypeMismatch(span, type1, type2) => Diagnostic::error()
                .with_message(format!(
                    "Type mismatch. Type \"{}\" was expected, but \"{}\" was found",
                    type1, type2
                ))
                .with_labels(vec![
                    Label::primary(0, span).with_message(format!("Problem occured here"))
                ]),
            SemanticError::OperationTypeMismatch(type1, span1, type2, span2) => Diagnostic::error()
                .with_message(format!(
                    "Type mismatch. Type \"{}\" was expected, but \"{}\" was found",
                    type1, type2
                ))
                .with_labels(vec![
                    Label::primary(0, span1).with_message(format!("Of typ \"{}\"", type1)),
                    Label::primary(0, span2)
                        .with_message(format!("Cannot be used with type \"{}\"", type2)),
                ]),
            SemanticError::NotAssignedReturnVariable(span, name) => Diagnostic::error()
                .with_message(format!("Return variable \"{}\" is not assigned", name))
                .with_labels(vec![
                    Label::primary(0, span).with_message("Result defined here")
                ]),
            SemanticError::VariableRedefinition(name, sp_old, sp_new) => Diagnostic::error()
                .with_message(format!("Variable redefinition: \"{}\"", name))
                .with_labels(vec![
                    Label::primary(0, sp_old).with_message("Previously defined here"),
                    Label::primary(0, sp_new).with_message("Redeclared here"),
                ]),
            SemanticError::LogicTypeMismatch(typ, operator, span) => Diagnostic::error()
                .with_message(format!(
                    "Operator {} expects operands of type bool but found {}",
                    operator, typ
                ))
                .with_labels(vec![Label::primary(0, span).with_message(format!(
                    "This expression evaluates to {} instead of bool",
                    typ
                ))]),
            SemanticError::UnknownFunction(name, span) => Diagnostic::error()
                .with_message(format!("Function {} was not found in current scope", name))
                .with_labels(vec![
                    Label::primary(0, span).with_message("Faulty invocation found here")
                ]),
            SemanticError::ArgumentsMismatch(name, span, _possible) => {
                // ignore possible prototypes for now...
                Diagnostic::error()
                    .with_message(format!(
                        "No prototype of function {} accepts given argument types.",
                        name
                    ))
                    .with_labels(vec![
                        Label::primary(0, span).with_message("Faulty invocation found here")
                    ])
            }
            SemanticError::NoField(type_name, expression_span, field_name) => Diagnostic::error()
                .with_message(format!(
                    "Expression of type {} has no field named \"{}\"",
                    type_name, field_name
                ))
                .with_labels(vec![Label::primary(0, expression_span)
                    .with_message(format!("Expression has type {}", type_name))]),
            _ => Diagnostic::error().with_message(format!("error is not implemented...")),
        }
    }
}
//UnusedArgument,
//// span of conflict, first is expected, second is provided
//TypeMismatch(Sp, String, String),
//ArgumentsNumberMismatch,
