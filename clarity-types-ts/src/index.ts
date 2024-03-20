import { ClarityValue } from "@stacks/transactions";

type ContractOrDef<T extends ContractDef | Contract<any>> = T extends ContractDef ? T : T extends Contract<infer C> ? C : never

export interface ContractBaseOptions {
    contractName: string;
    contractAddress: string;
}

export type ContractDef = {
    functionName: string,
    functionArgs: ClarityValue[],
    functionReturn: ClarityValue,
}

export type ContractReturnType<T extends ContractDef | Contract<any>, Name extends ContractOrDef<T>["functionName"]> = Extract<ContractOrDef<T>, { functionName: Name }>["functionReturn"]

export type ContractCallOptions<T extends ContractDef> = { [K in keyof T as Exclude<K, "functionReturn">]: T[K] }

export interface Contract<T extends ContractDef> {
    callOptions(options: ContractCallOptions<T>): {
        contractName: string;
        contractAddress: string;
        functionName: string;
        functionArgs: ClarityValue[];
    }
}

export function defineContract<T extends ContractDef>(baseOptions: ContractBaseOptions): Contract<T> {
    return {
        callOptions(options) {
            return { ...baseOptions, ...options }
        },
    }
}
