use parser::ast::*;

use self::env::Scope;
use errors::SemanticError;
use errors::SemanticWarning;

pub mod env;

use parser::codespan_reporting;
use parser::codespan_reporting::Diagnostic;
use log::{info};

pub fn analyze(program: &mut Program) -> Result<(), (Vec<SemanticError>, Vec<SemanticWarning>)> {
    info!("Analyzing program semantics");


    let global_scope = Scope::global();
    info!("Using global env: {:?}", global_scope);
    // analyzing functions prototypes and adding to env
    let mut program_scope = Scope::child_scope(&global_scope);
    // Adding function definitions to scope.
    #[allow(unused_mut)]
        let mut errors = vec![];
    let mut warnings = vec![];

    for item in &program.items {
        let (mut errs, mut warn) = analyze_gpu_function(item, &program_scope);
        errors.append(&mut errs);
        warnings.append(&mut warn);
        //program_scope.add_function(item);
    }

    {
        let (mut errs, mut warn) = analyze_pipeline(&program.pipeline, &program_scope);
        errors.append(&mut errs);
        warnings.append(&mut warn);
    }

    if errors.len() > 0 || warnings.len() > 0 {
        Result::Err((errors, warnings))
    } else {
        Result::Ok(())
    }
}

fn analyze_gpu_function(function: &GpuFunction, _env: &Scope) -> (Vec<SemanticError>, Vec<SemanticWarning>) {
    // very simple analyze as it's just temporary.
    let mut errors = vec![];
    let mut warnings = vec![];

    for name in &function.arguments {
        if !function.code.val.contains(&name.identifier.val) {
            warnings.push(SemanticWarning::NotUsedArgument(name.identifier.span, name.identifier.val.clone()));
        }
    }

    for name in &function.results {
        if ! function.code.val.contains(&name.identifier.val) {
            errors.push(SemanticError::NotAssignedReturnVariable(name.identifier.span, name.identifier.val.clone()));
        }
    }

    (errors, warnings)
}

fn analyze_pipeline(pipeline: &Pipeline, _env: &Scope) -> (Vec<SemanticError>, Vec<SemanticWarning>) {
    let mut errors = vec![];
    let mut warnings = vec![];


    for statement in &pipeline.block.statements{
//        match statement {
//            Statement::Assignment(var, exp, isCreation)=>{
//
//
//            }
//        }
    }



    (errors, warnings)
}

pub struct WarningWrap(pub SemanticWarning);
pub struct ErrorWrap(pub SemanticError);

impl WarningWrap {
    pub fn new(w: SemanticWarning) -> WarningWrap { WarningWrap(w) }
}
impl ErrorWrap {
    pub fn new(w: SemanticError) -> ErrorWrap { ErrorWrap(w) }
}

impl From<WarningWrap> for Diagnostic {
    fn from(w: WarningWrap) -> Diagnostic {
        match w.0 {
            SemanticWarning::NotUsedArgument(span, name) => {
                Diagnostic::new_warning(format!("Argument \"{}\" is not used.", name)).with_label(
                    codespan_reporting::Label::new_primary(span).with_message("Argument defined here")
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
                    codespan_reporting::Label::new_primary(span).with_message("Unknown identifier used here")
                )
            },
            SemanticError::TypeMismatch(span, type1, type2) => {
                Diagnostic::new_error(format!("Type mismatch. Type \"{}\" was expected, but \"{}\" was found", type1, type2)).with_label(
                    codespan_reporting::Label::new_primary(span)
                )
            }
            SemanticError::OperationTypeMismatch(type1, span1, type2, span2) => {
                Diagnostic::new_error(format!("Type mismatch. Type \"{}\" was expected, but \"{}\" was found", type1, type2)).with_label(
                    codespan_reporting::Label::new_primary(span1).with_message(format!("Of typ \"{}\"", type1))
                ).with_label(
                    codespan_reporting::Label::new_primary(span2).with_message(format!("Cannot be used with type \"{}\"", type2))
                )
            }
            SemanticError::NotAssignedReturnVariable(span, name)=>{
                Diagnostic::new_error(format!("Return variable \"{}\" is not assigned", name)).with_label(
                    codespan_reporting::Label::new_primary(span).with_message("Result defined here")
                )
            },
            SemanticError::VariableRedefinition(name, sp_old, sp_new)=>{
                Diagnostic::new_error(format!("Variable redefinition: \"{}\"", name)).with_label(
                    codespan_reporting::Label::new_primary(sp_old).with_message("Previously defined here")
                )
            }
            _=>{
                Diagnostic::new_error(format!("Return variable is not assigned"))

            }

        }
    }
}
//UnusedArgument,
//// span of conflict, first is expected, second is provided
//TypeMismatch(Sp, String, String),
//ArgumentsNumberMismatch,
