use clarity::vm::types::QualifiedContractIdentifier;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::errors::Error;

pub enum Source {
  FS {
    contract_path: PathBuf,
    traits_dir: PathBuf,
  },
  Mem {
    contract_source: String,
    traits: HashMap<String, String>,
  },
}

impl Source {
  pub fn nest(&self, identifier: &QualifiedContractIdentifier) -> Result<Self, Error> {
    let contract_source = self.read_trait(identifier)?.to_string();
    Ok(Self::Mem {
      contract_source,
      // nested source is unlikely to implement other traits
      traits: Default::default(),
    })
  }

  pub fn read_contract(&self) -> Result<String, Error> {
    match self {
      Self::FS { contract_path, .. } => {
        fs::read_to_string(&contract_path).map_err(|_| Error::ReadContract {
          path: contract_path.clone(),
        })
      }
      Self::Mem {
        contract_source, ..
      } => Ok(contract_source.clone()),
    }
  }

  pub fn read_trait(&self, identifier: &QualifiedContractIdentifier) -> Result<String, Error> {
    match self {
      Self::FS { traits_dir, .. } => {
        let mut trait_path = traits_dir.clone();
        trait_path.push(format!("{}.clar", identifier));
        fs::read_to_string(&trait_path).map_err(|_| Error::ReadTrait {
          path: Some(trait_path),
          identifier: identifier.clone(),
        })
      }
      Self::Mem { traits, .. } => traits
        .get(&identifier.to_string())
        .ok_or(Error::ReadTrait {
          path: None,
          identifier: identifier.clone(),
        })
        .map(|s| s.clone()),
    }
  }
}
