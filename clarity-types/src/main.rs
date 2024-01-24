use clap::{
  builder::{PossibleValuesParser, TypedValueParser},
  Parser, ValueHint,
};
use std::{fs, path::PathBuf, process};

use clarity_types::{self, ClarityVersion};

mod utils;

#[derive(Parser)]
#[command(
  version,
  name = "Clarity Gen",
  about = "Generate typescript types from clarity contract"
)]
#[command(arg_required_else_help = true)]
struct Cli {
  /// Input clarity contract file
  #[arg(name="input", value_hint = ValueHint::FilePath)]
  input: PathBuf,

  /// Output typescript file (defaults to input with a .ts extension)
  #[arg(name="output", value_hint = ValueHint::FilePath)]
  output: Option<PathBuf>,

  /// Output contract type name
  #[arg(long, value_name = "name")]
  type_name: Option<String>,

  /// Clarity version
  #[arg(
        long = "clarity-version",
        value_name = "version",
        value_parser = PossibleValuesParser::new(["clarity1", "clarity2"]).map(|s| s.parse::<ClarityVersion>().expect("Unknown clarity version provided")),
    )]
  clarity_version: Option<ClarityVersion>,

  /// Path to trait requirement (defaults to .cache/requirements relative from src parent dir as generated by clarinet)
  #[arg(long, name="path", value_hint = ValueHint::DirPath)]
  trait_dir: Option<PathBuf>,
}

fn main() {
  let args = Cli::parse();

  // check file extension
  if args.input.extension().and_then(|ext| ext.to_str()) != Some("clar") {
    eprintln!("File must be a clarity contract with a .clar extension");
    process::exit(1)
  }

  let dest = args
    .output
    .unwrap_or_else(|| args.input.with_extension("ts"));

  let contract_name = args.type_name.unwrap_or_else(|| {
        args.input.file_stem().and_then(|s| s.to_str()).map(utils::to_pascal_case)
            .expect("Failed to get contract name! try passing the contract name with an option flag eg: `--type-name=Contract`")
    });

  // build typescript types
  let result = clarity_types::parse(
    args.input,
    args.trait_dir,
    Some(contract_name),
    args.clarity_version,
  )
  .unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1)
  });

  // write typescript file
  match fs::write(&dest, result) {
    Ok(_) => println!("Created typescript file at {dest:?}"),
    Err(err) => {
      eprintln!("Failed to create typescript file: {}", err);
      process::exit(1)
    }
  };
}
