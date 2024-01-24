use clarity::vm::{
    types::{FunctionType, SequenceSubtype, StringSubtype, TypeSignature},
    ClarityName,
};
use std::collections::{BTreeMap, BTreeSet};

use crate::utils;

#[derive(Default)]
struct Reporter {
    fn_names: Vec<String>,
    imports: BTreeSet<String>,
}

impl Reporter {
    fn add_fn(&mut self, name: String) {
        self.fn_names.push(name);
    }

    fn add_import(&mut self, name: String) {
        self.imports.insert(name);
    }
}

trait IntoTypescriptType {
    fn into_ts_type(&self, reporter: &mut Reporter) -> String;
}

pub fn build_types(
    name: &str,
    readonly_fns: &BTreeMap<ClarityName, FunctionType>,
    public_fns: &BTreeMap<ClarityName, FunctionType>,
) -> String {
    let mut reporter = Reporter::default();

    let readonly_fns = readonly_fns
        .iter()
        .map::<String, _>(|fn_def| fn_def.into_ts_type(&mut reporter))
        .collect::<Vec<String>>()
        .join("\n");
    let public_fns = public_fns
        .iter()
        .map::<String, _>(|fn_def| fn_def.into_ts_type(&mut reporter))
        .collect::<Vec<String>>()
        .join("\n");

    // build import statement
    let mut contents = match reporter.imports.len() {
        0 => String::new(),
        len => {
            let imports = reporter.imports.into_iter().collect::<Vec<String>>();
            match len {
                len if len > 8 => {
                    format!(
                        "import {{\n\t{},\n}} from \"@stacks/transactions\"\n\n",
                        imports.join(",\n\t")
                    )
                }
                _ => {
                    format!(
                        "import {{ {} }} from \"@stacks/transactions\"\n\n",
                        imports.join(", ")
                    )
                }
            }
        }
    };

    // add readonly functions
    if !readonly_fns.is_empty() {
        contents.push_str("// readonly functions\n");
        contents.push_str(&readonly_fns);
        contents.push_str("\n");
    }
    // add public function
    if !public_fns.is_empty() {
        contents.push_str("// public functions\n");
        contents.push_str(&public_fns);
        contents.push_str("\n");
    }
    // add union type export
    let union_type = format!(
        "export type {}Contract = {}",
        name,
        reporter
            .fn_names
            .is_empty()
            .then_some("never")
            .unwrap_or(&reporter.fn_names.join(" | ")),
    );
    contents.push_str("// contract type\n");
    contents.push_str(&union_type);
    contents.push_str("\n");

    contents
}

impl IntoTypescriptType for (&ClarityName, &FunctionType) {
    fn into_ts_type(&self, reporter: &mut Reporter) -> String {
        let (fn_name, fn_type) = self;

        match fn_type {
            FunctionType::Fixed(fixed_fn) => {
                let fn_type_name = utils::to_pascal_case(fn_name);
                let fn_return_type = fixed_fn.returns.into_ts_type(reporter);
                let fn_args = fixed_fn
                    .args
                    .iter()
                    .map::<String, _>(|arg| {
                        format!(
                            "{}: {}",
                            arg.name.replace("-", "_"),
                            arg.signature.into_ts_type(reporter)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                reporter.add_fn(fn_type_name.clone());

                format!(
                    "export type {fn_type_name} = {{
    functionName: \"{fn_name}\",
    functionArgs: [{fn_args}],
    functionReturn: {fn_return_type},
}}\n"
                )
            }
            // return an empty string for unsupported function types
            _ => String::new(),
        }
    }
}

impl IntoTypescriptType for TypeSignature {
    fn into_ts_type(&self, reporter: &mut Reporter) -> String {
        match self {
            TypeSignature::BoolType => {
                reporter.add_import("BooleanCV".to_owned());
                "BooleanCV".to_owned()
            }
            TypeSignature::IntType => {
                reporter.add_import("IntCV".to_owned());
                "IntCV".to_owned()
            }
            TypeSignature::UIntType => {
                reporter.add_import("UIntCV".to_owned());
                "UIntCV".to_owned()
            }
            TypeSignature::PrincipalType => {
                reporter.add_import("PrincipalCV".to_owned());
                "PrincipalCV".to_owned()
            }
            TypeSignature::OptionalType(inner) => {
                reporter.add_import("OptionalCV".to_owned());
                format!("OptionalCV<{}>", inner.into_ts_type(reporter))
            }
            TypeSignature::ResponseType(inner) => {
                reporter.add_import("ResponseOkCV".to_owned());
                reporter.add_import("ResponseErrorCV".to_owned());
                format!(
                    "ResponseOkCV<{}> | ResponseErrorCV<{}>",
                    inner.0.into_ts_type(reporter),
                    inner.1.into_ts_type(reporter)
                )
            }
            TypeSignature::SequenceType(inner) => match inner {
                SequenceSubtype::StringType(inner) => match inner {
                    StringSubtype::ASCII(_) => {
                        reporter.add_import("StringAsciiCV".to_owned());
                        "StringAsciiCV".to_owned()
                    }
                    StringSubtype::UTF8(_) => {
                        reporter.add_import("StringUtf8CV".to_owned());
                        "StringUtf8CV".to_owned()
                    }
                },
                SequenceSubtype::BufferType(_) => {
                    reporter.add_import("BufferCV".to_owned());
                    "BufferCV".to_owned()
                }
                SequenceSubtype::ListType(data) => {
                    reporter.add_import("ListCV".to_owned());
                    format!(
                        "ListCV<{}>",
                        data.get_list_item_type().into_ts_type(reporter)
                    )
                }
            },
            TypeSignature::TupleType(inner) => {
                reporter.add_import("TupleCV".to_owned());
                let fields_iter = inner
                    .get_type_map()
                    .iter()
                    .map(|(name, value)| format!("{}: {}", name, value.into_ts_type(reporter)));
                format!(
                    "TupleCV<{{ {} }}>",
                    fields_iter.collect::<Vec<_>>().join(", ")
                )
            }
            TypeSignature::CallableType(_) => "unknown".to_owned(),
            TypeSignature::TraitReferenceType(_) => "unknown".to_owned(),
            TypeSignature::ListUnionType(_) => "unknown[]".to_owned(),
            TypeSignature::NoType => "never".to_owned(),
        }
    }
}
