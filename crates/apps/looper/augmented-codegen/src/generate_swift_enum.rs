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
use std::collections::HashMap;
use syn::{Type, Variant};

use super::CodegenOutput;

pub fn generate_swift_enum(i: &syn::ItemEnum) -> CodegenOutput {
    let mut swift_code = String::new();
    let ident = i.ident.clone();
    swift_code += &format!("enum {} {{ ", ident);
    if !i.variants.is_empty() {
        swift_code += "case ";
        let variant_code = i
            .variants
            .iter()
            .map(generate_variant_swift_ident)
            .collect::<Vec<String>>()
            .join(", ");
        swift_code += &*variant_code;
    }
    swift_code += " }";
    CodegenOutput {
        rust_code: "".to_string(),
        swift_code,
    }
}

fn generate_variant_swift_ident(variant: &Variant) -> String {
    let result = format!("{}", variant.ident);
    let mut result = lower_case(&result);
    if variant.fields.is_empty() {
        result
    } else {
        let fields_code = variant
            .fields
            .iter()
            .map(|field| {
                let mut r = "".to_string();
                if let Some(field_name) = &field.ident {
                    r += &*format!("let {}: ", field_name);
                }
                r += &*format_type(&field.ty);
                r
            })
            .collect::<Vec<String>>()
            .join(", ");

        result += "(";
        result += &*fields_code;
        result += ")";
        result
    }
}

fn format_type(ty: &Type) -> String {
    let type_aliases: HashMap<String, String> =
        HashMap::from([("usize".to_string(), "UInt".to_string())]);
    match ty {
        Type::Path(pth) => {
            let ident = pth.path.get_ident().unwrap().to_string();
            type_aliases.get(&ident).cloned().unwrap_or(ident)
        }
        // Type::Ptr(_) => {}
        // Type::Reference(_) => {}
        // Type::Slice(_) => {}
        // Type::TraitObject(_) => {}
        _ => todo!("Oh no"),
    }
}

pub(crate) fn capitalize(result: &str) -> String {
    let mut chars: Vec<char> = result.chars().collect();
    chars[0] = chars[0].to_uppercase().next().unwrap();
    chars.into_iter().collect()
}

pub(crate) fn lower_case(result: &str) -> String {
    let mut chars: Vec<char> = result.chars().collect();
    chars[0] = chars[0].to_lowercase().next().unwrap();
    chars.into_iter().collect()
}

#[cfg(test)]
mod test {
    use syn::ItemEnum;

    use crate::generate_swift_enum::generate_swift_enum;

    #[test]
    fn test_generate_simple_enum() {
        let input: ItemEnum = syn::parse_str(
            "\
        enum Something { \
            Option1, \
            Option2, \
            Option3, \
            Option4, \
        } \
        ",
        )
        .unwrap();
        let output = generate_swift_enum(&input);
        assert_eq!(
            output.swift_code,
            "enum Something { case option1, option2, option3, option4 }"
        );
    }
}
