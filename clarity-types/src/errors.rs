use clarity::vm::{diagnostic::Diagnostic, types::QualifiedContractIdentifier};
use std::{
  fmt::{self, Display},
  path::PathBuf,
};

#[derive(Debug)]
pub enum Error {
  ParseContractName {
    path: PathBuf,
  },
  ReadContract {
    path: PathBuf,
  },
  ReadTrait {
    path: Option<PathBuf>,
    identifier: QualifiedContractIdentifier,
  },
  ParseContract {
    diagnostics: Vec<Diagnostic>,
  },
  ParseTrait {
    diagnostics: Vec<Diagnostic>,
    identifier: QualifiedContractIdentifier,
  },
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ParseContractName { path } => {
        write!(f, "Failed to get contract name from path: {path:?}")
      }

      Self::ReadContract { path } => {
        write!(f, "Unable to read contract at: {path:?}")
      }

      Self::ReadTrait { path, identifier } => {
        write!(f, "Unable to read implemented trait {identifier}")?;
        if let Some(path) = path {
          write!(f, " at: {path:?}")?;
        }
        Ok(())
      }

      Self::ParseContract { diagnostics } => {
        for diagnostic in diagnostics {
          writeln!(f, "{diagnostic}")?;
        }
        write!(f, "Unable to parse contract")
      }

      Self::ParseTrait {
        diagnostics,
        identifier,
      } => {
        for diagnostic in diagnostics {
          writeln!(f, "{diagnostic}")?
        }
        write!(f, "Unable to parse implemented trait {identifier}")
      }
    }
  }
}
