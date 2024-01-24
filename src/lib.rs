pub use clarity::vm::{analysis::ContractAnalysis, ClarityVersion};
use std::path::{Path, PathBuf};

pub mod analysis;
pub mod typescript;
mod utils;

pub fn parse(
    path: &PathBuf,
    contract_name: &str,
    clarity_version: ClarityVersion,
    trait_dir: Option<PathBuf>,
) -> Result<String, analysis::Error> {
    let trait_dir = trait_dir.or_else(|| {
        let mut dir = path
            .parent()
            .and_then(Path::parent)
            .map(Path::to_owned)
            .unwrap_or_default();
        dir.push(".cache/requirements");
        Some(dir)
    });

    // run contract analysis
    let contract_analysis = analysis::run(path, contract_name, clarity_version, trait_dir)?;

    // build typescript types
    let ts_types = typescript::build_types(
        contract_name,
        &contract_analysis.read_only_function_types,
        &contract_analysis.public_function_types,
    );

    Ok(ts_types)
}
