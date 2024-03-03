# Clarity types

## Introduction

`@clarity-types/core` is a rust based library that generates typescript types from a clarity smart contract. It wraps the clarity vm and will automatically check your clarity contract for correctness before outputting types.

## Installation

```bash
npm i @clarity-types/core
```

## Usage

```ts
import { parse, parseMem } from "@clarity-types/core";

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
