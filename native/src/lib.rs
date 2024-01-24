#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;

use napi::bindgen_prelude::*;

#[napi]
pub fn parse(
  contract_path: String,
  trait_dir: Option<String>,
  contract_name: Option<String>,
  #[napi(ts_arg_type = "\"clarity1\" | \"clarity2\" | undefined")] clarity_version: Option<String>,
) -> Result<String> {
  let contract_path = contract_path.into();
  let trait_dir = trait_dir.map(|d| d.into());
  let clarity_version = clarity_version.map(|x| x.parse().unwrap());

  clarity_types::parse(contract_path, trait_dir, contract_name, clarity_version)
    .map_err(|err: clarity_types::errors::Error| Error::from_reason(err.to_string()))
}

#[napi]
pub fn parse_mem(
  contract_source: String,
  traits: HashMap<String, String>,
  contract_name: String,
  #[napi(ts_arg_type = "\"clarity1\" | \"clarity2\" | undefined")] clarity_version: Option<String>,
) -> Result<String> {
  let clarity_version = clarity_version.map(|x| x.parse().unwrap());

  clarity_types::parse_mem(contract_source, traits, contract_name, clarity_version)
    .map_err(|err| Error::from_reason(err.to_string()))
}
