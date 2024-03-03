# Clarity types CLI

## Introduction

`clarity-types` is a rust CLI that generates typescript types from a clarity smart contract. This CLI wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation

```bash
# install rust binary
cargo install https://github.com/eriicafes/clarity-types

# install rust library
cargo install https://github.com/eriicafes/clarity-types
```

> The rust crate is not published on crates.io because the active version of it's upstream dependency [clarity](https://github.com/stacks-network/stacks-core) is also not published.

## Usage

### Execute binary

```bash
clarity-types contracts/gm.clar

# specify output file
clarity-types contracts/gm.clar types/gm.ts

# specify traits directory
clarity-types contracts/gm.clar --traits-dir="contracts/.cache/requirements"

# specify typescript type name
clarity-types contracts/gm.clar --type-name GmContract
```

### Using rust library

```rs
use clarity_types::{parse, parse_mem};

// parse contract from filesystem
let result = parse(
    contract_path   // PathBuf,
    trait_dir       // Option<PathBuf>,
    contract_name   // Option<String>,
    clarity_version // Option<ClarityVersion>,
);
println!("{result}");

// or parse contract from in-memory string
let result = parse_mem(
    contract_source // String,
    traits          // HashMap<String, String>,
    contract_name   // String,
    clarity_version // Option<ClarityVersion>,
);
println!("{result}");
```

## Authors

- [@eriicafes](https://www.github.com/eriicafes)
