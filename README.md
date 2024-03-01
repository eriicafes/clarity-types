# Clarity types CLI

## Introduction

`clarity-types` is a rust CLI that generates typescript types from a clarity smart contract. This CLI wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation

```bash
# install typescript binary/library
npm i clarity-types

# install rust binary
cargo install https://github.com/eriicafes/clarity-types

# install rust library
cargo install https://github.com/eriicafes/clarity-types
```

> The rust crate is not published on crates.io because the active version of it's upstream dependency [clarity](https://github.com/stacks-network/stacks-core) is also not published.

## Usage

After [generating typescript types](#codegen-usage), import and use like below.

```ts
import { openContractCall } from "@stacks/connect"
import { defineContract } from "clarity-types"
import { Gm } from "../contracts/gm.clar"

const gmContract = defineContract<Gm>({
    contractName: // contract name,
    contractAddress: // contract address,
})

openContractCall({
    ...gmContract.callOptions({
        functionName: // function name;
        functionArgs: // function args;
    })
    // ...other options
})
```

callOptions returns the below shape and can be safely spread into openContractCall options.
```ts
{
    contractName: string;
    contractAddress: string;
    functionName: string;
    functionArgs: ClarityValue[];
}
```

You can also import and use each individual method type. Below is the signature of each contract method.

```ts
type ContractDef = {
  functionName: string;
  functionArgs: ClarityValue[];
  functionReturn: ClarityValue;
};
```

    The contract name export is a union of all methods.

## Codegen Usage

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

### Using typescript library

```ts
import { parse, parseMem } from "clarity-types"

// parse contract from filesystem
const result = parse(
    contractPath    // string,
    traitDir        // string | undefined | null,
    contractName    // string | undefined | null,
    clarityVersion  // "clarity1" | "clarity2" | undefined,
)
console.log(result)

// or parse contract from in-memory string
const result = parseMem(
    contractSource  // string,
    traits          // Record<string, string>,
    contractName    // string,
    clarityVersion  // "clarity1" | "clarity2" | undefined,
)
console.log(result)
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
