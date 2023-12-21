use std::{fs, path::PathBuf};

use clarity::vm::{
    analysis::{run_analysis, ContractAnalysis},
    ast::build_ast_with_diagnostics,
    costs::LimitedCostTracker,
    database::MemoryBackingStore,
    diagnostic::Diagnostic,
    types::QualifiedContractIdentifier,
    ClarityVersion,
};
use clarity::{types::StacksEpochId, vm::analysis::AnalysisDatabase};

enum Failure {
    Read,
    Parse,
}

pub struct Error {
    failure: Failure,
    pub name: String,
    pub message: String,
    pub path: PathBuf,
    pub diagnostics: Option<Vec<Diagnostic>>,
}

pub fn run(
    contract_name: &str,
    path: &PathBuf,
    impl_trait_dir: Option<PathBuf>,
    clarity_version: ClarityVersion,
) -> Result<ContractAnalysis, Error> {
    let contract_identifier = QualifiedContractIdentifier::transient();
    let epoch = StacksEpochId::latest();
    let cost_tracker = LimitedCostTracker::new_free();
    let mut datastore = MemoryBackingStore::new();
    let mut analysis_db = datastore.as_analysis_db();

    // must have a nested context to update analysis_db store for saved contracts
    // create the root context here
    analysis_db.execute(|db| {
        run_with_impl_traits(
            contract_name,
            path,
            impl_trait_dir,
            &contract_identifier,
            db,
            cost_tracker,
            clarity_version,
            epoch,
        )
    })
}

fn run_with_impl_traits(
    name: &str,
    path: &PathBuf,
    impl_trait_dir: Option<PathBuf>,
    identifier: &QualifiedContractIdentifier,
    analysis_db: &mut AnalysisDatabase,
    mut cost_tracker: LimitedCostTracker,
    clarity_version: ClarityVersion,
    epoch: StacksEpochId,
) -> Result<ContractAnalysis, Error> {
    // read contract file
    let contents = fs::read_to_string(path).map_err(|e| Error {
        failure: Failure::Read,
        name: name.to_string(),
        message: format!("Unable to read contract: {}", e),
        path: path.clone(),
        diagnostics: None,
    })?;

    // build contract ast
    let (mut ast, mut diagnostics, success) = build_ast_with_diagnostics(
        identifier,
        &contents,
        &mut cost_tracker,
        clarity_version,
        epoch,
    );

    if !success {
        return Err(Error {
            failure: Failure::Parse,
            name: name.to_string(),
            message: "Unable to parse contract".to_owned(),
            path: path.clone(),
            diagnostics: Some(diagnostics),
        });
    }

    // load implemented traits
    if let Some(impl_trait_dir) = impl_trait_dir {
        for trait_identifier in ast.implemented_traits.clone() {
            let mut trait_path = impl_trait_dir.clone();
            trait_path.push(format!("{}.clar", &trait_identifier.contract_identifier));

            if let Err(mut err) = run_with_impl_traits(
                &trait_identifier.name,
                &trait_path,
                // assumption is traits do not implement other traits
                None,
                &trait_identifier.contract_identifier,
                analysis_db,
                cost_tracker.clone(),
                clarity_version,
                epoch,
            ) {
                err.message = match err.failure {
                    Failure::Read => "Unable to read implemented trait contract".to_owned(),
                    Failure::Parse => "Unable to parse implemented trait contract".to_owned(),
                };
                return Err(err);
            }
        }
    }

    // run contract analysis
    run_analysis(
        &identifier,
        &mut ast.expressions,
        analysis_db,
        // save implemented contracts otherwise analysis would fail during trait check
        true,
        cost_tracker,
        epoch,
        clarity_version,
    )
    .map_err(|(e, _)| {
        diagnostics.push(Diagnostic::err(&e.err));
        Error {
            failure: Failure::Parse,
            name: name.to_string(),
            message: "Unable to parse contract".to_owned(),
            path: path.clone(),
            diagnostics: Some(diagnostics),
        }
    })
}
