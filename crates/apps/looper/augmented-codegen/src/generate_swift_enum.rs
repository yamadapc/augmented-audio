use syn::Variant;

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
    lower_case(&result)
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
