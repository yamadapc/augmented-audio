use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::process;
use std::process::id;

use complexity::Complexity;
use syn::visit::Visit;
use syn::{
    FnArg, Ident, ImplItem, ImplItemMethod, Item, ItemImpl, ItemMod, ItemStruct, ItemUse, Pat,
    PatType, ReturnType, Type, UseName, UsePath, UseTree, VisPublic, Visibility,
};

use crate::generate_swift_enum::generate_swift_enum;
use crate::generate_swift_opaque_value::{
    generate_opaque_method, generate_opaque_value, OpaqueValueInput, OpaqueValueMethod,
};

mod generate_swift_enum;
mod generate_swift_opaque_value;

struct MethodSummary {
    ident: Ident,
}

struct StructSummary {
    ident: Ident,
    methods: Vec<MethodSummary>,
}

#[derive(Default)]
struct CodegenVisitor {
    public_structs: Vec<Ident>,
    summaries: Vec<StructSummary>,
    current_impl: Option<StructSummary>,
}

impl<'ast> Visit<'ast> for CodegenVisitor {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        if i.vis
            == Visibility::Public(VisPublic {
                pub_token: Default::default(),
            })
        {
            println!("Public struct: {}", i.ident);
            self.public_structs.push(i.ident.clone());
        }
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        match i.self_ty.deref() {
            Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    println!("enter impl {}", ident);
                    self.current_impl = Some(StructSummary {
                        ident: ident.clone(),
                        methods: vec![],
                    });
                    syn::visit::visit_item_impl(self, i);
                    self.summaries.push(self.current_impl.take().unwrap());
                    self.current_impl = None;
                }
            }
            _ => {}
        }
    }

    fn visit_impl_item_method(&mut self, i: &'ast ImplItemMethod) {
        let mut current_struct = self.current_impl.as_mut().unwrap();
        let arguments = i
            .sig
            .inputs
            .iter()
            .filter_map(|inp| match inp {
                FnArg::Receiver(_) => None,
                FnArg::Typed(ty) => match ty {
                    PatType { ty, .. } => match ty.deref() {
                        Type::Path(path) => path.path.get_ident(),
                        _ => None,
                    },
                },
            })
            .cloned()
            .collect::<Vec<Ident>>();
        println!("Inputs: {:?}", i.sig.inputs);
        println!(
            "Method: {}::{}({:?})",
            current_struct.ident, i.sig.ident, arguments
        );
        current_struct.methods.push(MethodSummary {
            ident: i.sig.ident.clone(),
        });
    }

    fn visit_use_path(&mut self, i: &'ast UsePath) {
        if i.ident == "crate" {
            // println!("use {:?}", i);
            let path = get_filepath(i);
            println!("  --> {:?}", path);
        }
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {}
}

fn get_filepath(i: &UsePath) -> Option<PathBuf> {
    // println!("  --> {:?}", i);
    match i.tree.deref() {
        UseTree::Path(path) => {
            let rest = get_filepath(path);
            let path = PathBuf::from(path.ident.to_string());
            if let Some(r) = rest {
                Some(path.join(r))
            } else {
                Some(path)
            }
        }
        _ => None,
    }
}

fn lint_file(file: &syn::File) {
    for item in &file.items {
        match item {
            // An impl block like `impl Struct { .. }` or `impl Display for Struct { .. }`
            syn::Item::Impl(item_impl) => {
                for impl_item in &item_impl.items {
                    if let ImplItem::Method(method) = impl_item {
                        match &*item_impl.self_ty {
                            Type::Path(syn::TypePath { qself: None, path }) => {
                                let name = format!(
                                    "{}::{}",
                                    path.segments.last().unwrap().ident,
                                    method.sig.ident
                                );
                                let complexity = method.complexity();
                                if complexity > 4 {
                                    log::warn!("name={} complexity={}", name, complexity);
                                }
                            }
                            _ => {}
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

    for filename in glob::glob(&*format!("{}/src/**/*.rs", crate_dir)).unwrap() {
        let filename = filename.unwrap();
        let mut file = File::open(&filename).expect("Unable to open file");
        log::debug!("Processing {}", filename.to_str().unwrap());

        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

        let file = syn::parse_file(&src).expect("Unable to parse file");
        lint_file(&file);

        let mut c_api_module = "".to_string();
        let mut swift_module = "".to_string();
        for item in file.items.iter() {
            match item {
                Item::Enum(en) => {
                    log::info!("Running over {:?} / {}", filename, en.ident);
                    let result = generate_swift_enum(en);
                    combine_result(&mut c_api_module, &mut swift_module, result)
                }
                Item::Fn(_) => {}
                Item::Impl(i) => {
                    for item in &i.items {
                        match item {
                            ImplItem::Method(method) => {
                                if let Some(result) = process_impl_method(i, method) {
                                    combine_result(&mut c_api_module, &mut swift_module, result)
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Item::Struct(str) => {
                    let result = generate_opaque_value(OpaqueValueInput {
                        identifier: str.ident.to_string(),
                    });
                    combine_result(&mut c_api_module, &mut swift_module, result)
                }
                Item::Type(_) => {}
                _ => {}
            }
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

        // let mut visitor = CodegenVisitor::default();
        // visitor.visit_file(&file);
        // for struct_name in visitor.summaries {
        //     println!("class Native${} {{", struct_name.ident);
        //     println!("  var __nativePtr: OpaquePointer");
        //     println!("  init() {{");
        //     println!("      self.__nativePtr = __nativeInit__x1234()");
        //     println!("  }}");
        //     println!();
        //     for method in struct_name.methods {
        //         println!("  func {}() {{", method.ident);
        //         println!("  }}");
        //     }
        //     println!();
        //     println!("}}");
        // }
    }
}

fn process_impl_method(i: &ItemImpl, method: &ImplItemMethod) -> Option<CodegenOutput> {
    let parent = match i.self_ty.deref() {
        Type::Path(path) => path.path.get_ident()?.to_string(),
        _ => return None,
    };

    if method.sig.inputs.len() == 0 {
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
