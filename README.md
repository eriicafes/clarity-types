# Clarity types CLI

## Introduction
`clarity-types` is a rust CLI that generates typescript types from a clarity smart contract. This CLI wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation
```bash
# install rust binary
cargo install https://github.com/eriicafes/clarity-types

# install rust library
cargo install https://github.com/eriicafes/clarity-types

# install typescript binary/library
npm i clarity-types
```
> The rust crate is not published on crates.io because it's upstream dependency [clarity-vm](https://github.com/stacks-network/stacks-core) is not published on crates.io

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

### Use rust library
```rs
```

### Use typescript library
```ts
```

## Authors

- [@eriicafes](https://www.github.com/eriicafes)
