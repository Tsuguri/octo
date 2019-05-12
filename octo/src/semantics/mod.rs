use parser::ast::*;

use self::env::Scope;
use errors::SemanticError;
use errors::SemanticWarning;

mod env;

use parser::codespan_reporting;
use parser::codespan_reporting::Diagnostic;

pub fn analyze(program: &mut Program) -> Result<(), (Vec<SemanticError>, Vec<SemanticWarning>)> {
    println!("Analyzing program semantics");
    let global_scope = Scope::global();
    println!("Using global env: {:?}", global_scope);
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
    }
    if errors.len() > 0 || warnings.len() > 0 {
        Result::Err((errors, warnings))
    } else {
        Result::Ok(())
    }
}

fn analyze_gpu_function(function: &GpuFunction, _env: &Scope) -> (Vec<SemanticError>, Vec<SemanticWarning>) {
    println!("-- Analyzing function {}", function.name.val);
    let mut errors = vec![];
    let mut warnings = vec![];

    for name in &function.arguments {
        if !function.code.val.contains(&name.identifier.val) {
            warnings.push(SemanticWarning::NotUsedArgument(name.identifier.span, name.identifier.val.clone()));
        }
    }

    (errors, warnings)
}

pub struct WarningWrap(pub SemanticWarning);

impl WarningWrap {
    pub fn new(w: SemanticWarning) -> WarningWrap { WarningWrap(w) }
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