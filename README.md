# Clarity types CLI

## Introduction
`clarity-types` is a rust CLI that generates typescript types from a clarity smart contract. This CLI wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation
```bash
cargo install clarity-types
```

## Usage
```bash
clarity-types contracts/gm.clar

# specify output file
clarity-types contracts/gm.clar types/gm.ts

# specify traits directory
clarity-types contracts/gm.clar --traits-dir="contracts/.cache/requirements"

# specify typescript type name
clarity-types contracts/gm.clar --type-name GmContract
```

## Authors

- [@eriicafes](https://www.github.com/eriicafes)
