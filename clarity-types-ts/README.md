# Clarity types

## Introduction

`clarity-types` is a rust based library/CLI that generates typescript types from a clarity smart contract. It wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation

```bash
npm i clarity-types
```

## Usage

After [generating typescript types](#codegen-usage), import and use like below.

```ts
import { openContractCall } from "@stacks/connect";
import { defineContract } from "clarity-types";
import { Gm } from "../contracts/gm.ts";

const gmContract = defineContract<Gm>({
  contractName, // contract name
  contractAddress, // contract address
});

openContractCall({
  ...gmContract.callOptions({
    functionName, // function name
    functionArgs, // function args
  }),
  // ...other options
});
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
npx clarity-types contracts/gm.clar

# specify output file
npx clarity-types contracts/gm.clar types/gm.ts

# specify traits directory
npx clarity-types contracts/gm.clar --traits-dir="contracts/.cache/requirements"

# specify typescript type name
npx clarity-types contracts/gm.clar --type-name GmContract
```

### Using typescript library

```ts
import { parse, parseMem } from "clarity-types";

// parse contract from filesystem
const result = parse(
  contractPath, // string
  traitDir, // string | undefined | null
  contractName, // string | undefined | null
  clarityVersion // "clarity1" | "clarity2" | undefined
);
console.log(result);

// or parse contract from in-memory string
const result = parseMem(
  contractSource, // string
  traits, // Record<string, string>
  contractName, // string
  clarityVersion // "clarity1" | "clarity2" | undefined
);
console.log(result);
```

## Authors

- [@eriicafes](https://www.github.com/eriicafes)
