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
            .map(|variant| generate_variant_swift_ident(variant))
            .collect::<Vec<String>>()
            .join(", ");
        swift_code += &*variant_code;
    }
    swift_code += &format!(" }}");
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
            type_aliases.get(&ident).map(|s| s.clone()).unwrap_or(ident)
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
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    chars.into_iter().collect()
}

pub(crate) fn lower_case(result: &str) -> String {
    let mut chars: Vec<char> = result.chars().collect();
    chars[0] = chars[0].to_lowercase().nth(0).unwrap();
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
