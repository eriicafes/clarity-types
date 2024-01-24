use clarity::types::StacksEpochId;
use clarity::vm::{
  analysis::run_analysis, analysis::AnalysisDatabase, ast::build_ast_with_diagnostics,
  costs::LimitedCostTracker, database::MemoryBackingStore, diagnostic::Diagnostic,
  types::QualifiedContractIdentifier,
};
pub use clarity::vm::{analysis::ContractAnalysis, ClarityVersion};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::errors::Error;
use crate::source::Source;

pub fn run_with_fs(
  contract_path: PathBuf,
  traits_dir: PathBuf,
  clarity_version: ClarityVersion,
) -> Result<ContractAnalysis, Error> {
  let source = Source::FS {
    contract_path,
    traits_dir,
  };
  run(&source, clarity_version)
}

pub fn run_with_mem(
  contract_source: String,
  traits: HashMap<String, String>,
  clarity_version: ClarityVersion,
) -> Result<ContractAnalysis, Error> {
  let source = Source::Mem {
    contract_source,
    traits,
  };
  run(&source, clarity_version)
}

pub fn run(source: &Source, clarity_version: ClarityVersion) -> Result<ContractAnalysis, Error> {
  let contract_identifier = QualifiedContractIdentifier::transient();
  let epoch = StacksEpochId::latest();
  let cost_tracker = LimitedCostTracker::new_free();
  let mut datastore = MemoryBackingStore::new();
  let mut analysis_db = datastore.as_analysis_db();

  // must have a nested context to update analysis_db store for saved contracts
  // create the root context here
  analysis_db.execute(|db| {
    run_with_impl_traits(
      source,
      clarity_version,
      &contract_identifier,
      db,
      cost_tracker,
      epoch,
      false,
    )
  })
}

fn run_with_impl_traits(
  source: &Source,
  clarity_version: ClarityVersion,
  contract_identifier: &QualifiedContractIdentifier,
  analysis_db: &mut AnalysisDatabase,
  mut cost_tracker: LimitedCostTracker,
  epoch: StacksEpochId,
  is_impl_trait: bool,
) -> Result<ContractAnalysis, Error> {
  // read contract source
  let source_code = source.read_contract()?;

  // build contract ast
  let (mut ast, mut diagnostics, success) = build_ast_with_diagnostics(
    contract_identifier,
    &source_code,
    &mut cost_tracker,
    clarity_version,
    epoch,
  );

  if !success {
    if is_impl_trait {
      return Err(Error::ParseTrait {
        diagnostics,
        identifier: contract_identifier.clone(),
      });
    }
    return Err(Error::ParseContract { diagnostics });
  }

  // load implemented traits
  for trait_identifier in ast.implemented_traits.clone() {
    let nested_source = source.nest(&trait_identifier.contract_identifier)?;

    run_with_impl_traits(
      &nested_source,
      clarity_version,
      &trait_identifier.contract_identifier,
      analysis_db,
      cost_tracker.clone(),
      epoch,
      true,
    )?;
  }

  // run contract analysis
  run_analysis(
    &contract_identifier,
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
    if is_impl_trait {
      Error::ParseTrait {
        diagnostics,
        identifier: contract_identifier.clone(),
      }
    } else {
      Error::ParseContract { diagnostics }
    }
  })
}
