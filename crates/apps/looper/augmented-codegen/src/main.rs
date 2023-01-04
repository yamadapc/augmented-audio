// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::env;
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process;

use complexity::Complexity;
use syn::{
    Attribute, File, FnArg, Ident, ImplItem, ImplItemMethod, Item, ItemImpl, Pat, PatType,
    ReturnType, Type,
};

use crate::generate_swift_enum::generate_swift_enum;
use crate::generate_swift_opaque_value::{
    generate_opaque_method, generate_opaque_value, OpaqueValueInput, OpaqueValueMethod,
};

mod generate_swift_enum;
mod generate_swift_opaque_value;

fn lint_file(file: &syn::File) {
    for item in &file.items {
        match item {
            // An impl block like `impl Struct { .. }` or `impl Display for Struct { .. }`
            syn::Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let ImplItem::Method(method) = impl_item {
                        if let Type::Path(syn::TypePath { qself: None, path }) = &*item_impl.self_ty
                        {
                            let name = format!(
                                "{}::{}",
                                path.segments.last().unwrap().ident,
                                method.sig.ident
                            );
                            let complexity = method.complexity();
                            if complexity >= 15 {
                                log::warn!("name={} complexity={}", name, complexity);
                            }
                        }
                    }
                }
            }
            // A bare function like `fn function(arg: Arg) -> Result { .. }`
            syn::Item::Fn(item_fn) => {
                let name = item_fn.sig.ident.to_string();
                let complexity = item_fn.complexity();
                if complexity > 4 {
                    log::warn!("name={} complexity={}", name, complexity);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    wisual_logger::init_from_env();

    let mut args = env::args();
    let _ = args.next(); // executable name

    let crate_dir = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: augmented-codegen <DIRECTORY>");
            process::exit(1);
        }
    };

    for filename in glob::glob(&format!("{}/src/**/*.rs", crate_dir)).unwrap() {
        let filename = filename.unwrap();
        let mut file = std::fs::File::open(&filename).expect("Unable to open file");
        log::debug!("Processing {}", filename.to_str().unwrap());

        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

        let file = syn::parse_file(&src).expect("Unable to parse file");
        lint_file(&file);

        run_codegen(filename, file)
    }
}

fn run_codegen(filename: PathBuf, file: File) {
    let mut c_api_module = "".to_string();
    let mut swift_module = "".to_string();

    for item in file.items.iter() {
        collect_item(&filename, &mut c_api_module, &mut swift_module, item);
    }

    let prefix = if filename.file_name().unwrap() == "mod.rs" {
        filename.with_file_name("")
    } else {
        filename.with_extension("")
    };
    let c_api_filename = prefix.join("generated/c_api.rs");
    if !c_api_module.is_empty() {
        println!("// {}", c_api_filename.to_str().unwrap());
        println!("{}", c_api_module);
    }
    let swift_filename = prefix.join(format!(
        "generated/{}.swift",
        prefix.file_name().unwrap().to_str().unwrap()
    ));
    if !swift_module.is_empty() {
        println!("// {}", swift_filename.to_str().unwrap());
        println!("{}", swift_module);
    }
}

fn collect_item(
    filename: &Path,
    c_api_module: &mut String,
    swift_module: &mut String,
    item: &Item,
) {
    match item {
        Item::Enum(en) => {
            if !should_codegen(&en.attrs) {
                return;
            }

            log::info!("Running over {:?} / {}", filename, en.ident);

            let result = generate_swift_enum(en);
            combine_result(c_api_module, swift_module, result)
        }
        Item::Fn(_) => {}
        Item::Impl(i) => {
            if !should_codegen(&i.attrs) {
                return;
            }
            for item in &i.items {
                if let ImplItem::Method(method) = item {
                    if let Some(result) = process_impl_method(i, method) {
                        combine_result(c_api_module, swift_module, result)
                    }
                }
            }
        }
        Item::Struct(str) => {
            if !should_codegen(&str.attrs) {
                return;
            }

            if let Some(Ok(repr)) = str
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("repr"))
                .map(|attr| attr.parse_args::<Ident>())
            {
                if repr == "C" || repr == "transparent" {
                    return;
                }
            }

            let result = generate_opaque_value(OpaqueValueInput {
                identifier: str.ident.to_string(),
            });
            combine_result(c_api_module, swift_module, result)
        }
        Item::Type(_) => {}
        _ => {}
    }
}

fn should_codegen(attrs: &[Attribute]) -> bool {
    attrs.iter().for_each(|attr| {
        log::debug!("ident={:?}", attr.path);
    });

    let has_codegen_attr = attrs.iter().any(|attr| {
        let idents: Vec<String> = attr
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        idents == vec!["augmented_codegen".to_string(), "ffi_export".to_string()]
    });
    has_codegen_attr
}

fn process_impl_method(i: &ItemImpl, method: &ImplItemMethod) -> Option<CodegenOutput> {
    let parent = match i.self_ty.deref() {
        Type::Path(path) => path.path.get_ident()?.to_string(),
        _ => return None,
    };

    if method.sig.inputs.is_empty() {
        // Check if this is a constructor we can support
        return None;
    }

    let first_input = &method.sig.inputs[0];
    if matches!(first_input, FnArg::Receiver(_)) {
        let arguments = method
            .sig
            .inputs
            .iter()
            .skip(1)
            .filter_map(|input| {
                if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                    match (pat.deref(), ty.deref()) {
                        (Pat::Ident(ident), Type::Path(pth)) => {
                            Some((ident.ident.to_string(), pth.path.get_ident()?.to_string()))
                        }
                        // todo bail out of the whole function
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        let return_value = match &method.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => match ty.deref() {
                Type::Path(pth) => Some(pth.path.get_ident()?.to_string()),
                // todo support other patterns
                _ => return None,
            },
        };
        let result = generate_opaque_method(OpaqueValueMethod {
            parent,
            identifier: method.sig.ident.to_string(),
            arguments,
            return_value,
        });
        Some(result)
    } else {
        // Check if this is a constructor
        None
    }
}

fn combine_result(c_api_module: &mut String, swift_module: &mut String, result: CodegenOutput) {
    if !result.rust_code.is_empty() {
        *c_api_module += "\n";
        *c_api_module += &*result.rust_code;
    }
    if !result.swift_code.is_empty() {
        *swift_module += "\n";
        *swift_module += &*result.swift_code;
    }
}

#[derive(Default)]
pub struct CodegenOutput {
    rust_code: String,
    swift_code: String,
}
