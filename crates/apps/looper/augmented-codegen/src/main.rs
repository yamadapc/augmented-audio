mod generate_swift_enum;
mod generate_swift_opaque_value;

use complexity::Complexity;
use std::env;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::process;
use std::process::id;
use syn::visit::Visit;
use syn::{
    FnArg, Ident, ImplItem, ImplItemMethod, ItemImpl, ItemMod, ItemStruct, ItemUse, PatType, Type,
    UseName, UsePath, UseTree, VisPublic, Visibility,
};

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
                                    println!("name={} complexity={}", name, complexity);
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
                    println!("name={} complexity={}", name, complexity);
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
        log::info!("Processing {}", filename.to_str().unwrap());

        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

        let file = syn::parse_file(&src).expect("Unable to parse file");
        lint_file(&file);
        let mut visitor = CodegenVisitor::default();
        visitor.visit_file(&file);

        for struct_name in visitor.summaries {
            println!("class Native${} {{", struct_name.ident);
            println!("  var __nativePtr: OpaquePointer");
            println!("  init() {{");
            println!("      self.__nativePtr = __nativeInit__x1234()");
            println!("  }}");
            println!();
            for method in struct_name.methods {
                println!("  func {}() {{", method.ident);
                println!("  }}");
            }
            println!();
            println!("}}");
        }
    }
}

pub struct CodegenOutput {
    rust_code: String,
    swift_code: String,
}
