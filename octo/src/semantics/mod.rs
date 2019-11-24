use errors::SemanticError;
use errors::SemanticWarning;

pub mod env;

use parser::codespan_reporting;
use parser::codespan_reporting::Diagnostic;

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
            SemanticWarning::NotUsedArgument(span, name) => {
                Diagnostic::new_warning(format!("Argument \"{}\" is not used.", name)).with_label(
                    codespan_reporting::Label::new_primary(span)
                        .with_message("Argument defined here"),
                )
            }
            SemanticWarning::UnusedVariable(span, name) => {
                Diagnostic::new_warning(format!("Variable \"{}\" is not used.", name)).with_label(
                    codespan_reporting::Label::new_primary(span)
                        .with_message("variable defined here"),
                )
            }
        }
    }
}

impl From<ErrorWrap> for Diagnostic {
    fn from(w: ErrorWrap) -> Diagnostic {
        match w.0 {
            SemanticError::UndefinedIdentifier(span, name) => {
                Diagnostic::new_error(format!("Unknown variable \"{}\"", name)).with_label(
                    codespan_reporting::Label::new_primary(span)
                        .with_message("Unknown identifier used here"),
                )
            }
            SemanticError::TypeMismatch(span, type1, type2) => Diagnostic::new_error(format!(
                "Type mismatch. Type \"{}\" was expected, but \"{}\" was found",
                type1, type2
            ))
            .with_label(
                codespan_reporting::Label::new_primary(span)
                    .with_message(format!("Problem occured here")),
            ),
            SemanticError::OperationTypeMismatch(type1, span1, type2, span2) => {
                Diagnostic::new_error(format!(
                    "Type mismatch. Type \"{}\" was expected, but \"{}\" was found",
                    type1, type2
                ))
                .with_label(
                    codespan_reporting::Label::new_primary(span1)
                        .with_message(format!("Of typ \"{}\"", type1)),
                )
                .with_label(
                    codespan_reporting::Label::new_primary(span2)
                        .with_message(format!("Cannot be used with type \"{}\"", type2)),
                )
            }
            SemanticError::NotAssignedReturnVariable(span, name) => {
                Diagnostic::new_error(format!("Return variable \"{}\" is not assigned", name))
                    .with_label(
                        codespan_reporting::Label::new_primary(span)
                            .with_message("Result defined here"),
                    )
            }
            SemanticError::VariableRedefinition(name, sp_old, sp_new) => {
                Diagnostic::new_error(format!("Variable redefinition: \"{}\"", name))
                    .with_label(
                        codespan_reporting::Label::new_primary(sp_old)
                            .with_message("Previously defined here"),
                    )
                    .with_label(
                        codespan_reporting::Label::new_primary(sp_new)
                            .with_message("Redeclared here"),
                    )
            }
            SemanticError::LogicTypeMismatch(typ, operator, span) => {
                Diagnostic::new_error(format!(
                    "Operator {} expects operands of type bool but found {}",
                    operator, typ
                ))
                .with_label(
                    codespan_reporting::Label::new_primary(span).with_message(format!(
                        "This expression evaluates to {} instead of bool",
                        typ
                    )),
                )
            }
            _ => Diagnostic::new_error(format!("error is not implemented...")),
        }
    }
}
//UnusedArgument,
//// span of conflict, first is expected, second is provided
//TypeMismatch(Sp, String, String),
//ArgumentsNumberMismatch,
