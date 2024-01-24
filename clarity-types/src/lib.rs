use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

pub mod analysis;
pub mod errors;
pub mod source;
pub mod typescript;

pub use analysis::{ClarityVersion, ContractAnalysis};

mod utils;
use errors::Error;

pub fn parse(
  contract_path: PathBuf,
  trait_dir: Option<PathBuf>,
  contract_name: Option<String>,
  clarity_version: Option<ClarityVersion>,
) -> Result<String, Error> {
  let contract_name = contract_name
    .or_else(|| {
      contract_path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(utils::to_pascal_case)
    })
    .ok_or(Error::ParseContractName {
      path: contract_path.clone(),
    })?;

  let trait_dir = trait_dir.unwrap_or_else(|| {
    let mut dir = contract_path
      .parent()
      .and_then(Path::parent)
      .map(Path::to_owned)
      .unwrap_or_default();
    dir.push(".cache/requirements");
    dir
  });

  let clarity_version = clarity_version.unwrap_or(ClarityVersion::Clarity2);

  // run contract analysis
  let contract_analysis = analysis::run_with_fs(contract_path, trait_dir, clarity_version)?;

  // build typescript types
  let ts_types = typescript::build_types(
    &contract_name,
    &contract_analysis.read_only_function_types,
    &contract_analysis.public_function_types,
  );

  Ok(ts_types)
}

pub fn parse_mem(
  contract_source: String,
  traits: HashMap<String, String>,
  contract_name: String,
  clarity_version: Option<ClarityVersion>,
) -> Result<String, Error> {
  let clarity_version = clarity_version.unwrap_or(ClarityVersion::Clarity2);

  // run contract analysis
  let contract_analysis = analysis::run_with_mem(contract_source, traits, clarity_version)?;

  // build typescript types
  let ts_types = typescript::build_types(
    &contract_name,
    &contract_analysis.read_only_function_types,
    &contract_analysis.public_function_types,
  );

  Ok(ts_types)
}
