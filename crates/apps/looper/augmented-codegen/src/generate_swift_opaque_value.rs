use std::fmt::format;
use std::ops::Add;

use super::CodegenOutput;

pub struct OpaqueValueInput {
    pub identifier: String,
}

pub struct OpaqueValueMethod {
    pub parent: String,
    pub identifier: String,
    pub arguments: Vec<(String, String)>,
    pub return_value: Option<String>,
}

pub fn generate_opaque_value(input: OpaqueValueInput) -> CodegenOutput {
    let mut rust_code = "".to_string();
    rust_code += &*format!(
        r#"
#[no_mangle]
pub extern "C" fn boxed__{}__delete(ptr: *mut {}) {{
    let _ = unsafe {{ Box::from_raw(ptr) }};
}}
"#,
        input.identifier, input.identifier
    );
    let destructor_c_name = format!("{}__delete", input.identifier);

    let ident = "Boxed$".to_string() + &*input.identifier;
    let mut swift_code = "".to_string();
    swift_code += &*format!("class {} {{\n", ident);
    swift_code += "    let __innerPtr: OpaquePointer\n";
    swift_code += "    init(rawPtr: OpaquePointer) { self.__innerPtr = rawPtr }\n";
    swift_code += &*format!("    deinit {{ {}(self.__innerPtr) }}\n", destructor_c_name);
    swift_code += &*format!("}}\n");

    CodegenOutput {
        rust_code,
        swift_code,
    }
}

pub fn generate_opaque_method(value: OpaqueValueMethod) -> CodegenOutput {
    let mut rust_code = "".to_string();
    let arguments = value
        .arguments
        .iter()
        .map(|(name, ty)| format!("{}: {}", name, ty))
        .collect::<Vec<String>>();
    let arguments = {
        if !arguments.is_empty() {
            let s = arguments.join(", ");
            ", ".to_string() + &*s
        } else {
            "".to_string()
        }
    };
    let argument_names = value
        .arguments
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();
    let argument_names = if argument_names.is_empty() {
        "".to_string()
    } else {
        ", ".to_string() + &*argument_names.join(", ")
    };
    let return_value = value
        .return_value
        .as_ref()
        .map(|v| format!(" -> *mut {}", v))
        .unwrap_or("".to_string());

    let body = if value.return_value.is_some() {
        format!(
            r#"let result = {}::{}(value{});
    Box::into_raw(Box::new(result))"#,
            value.parent, value.identifier, argument_names
        )
    } else {
        format!(
            "{}::{}(value{})",
            value.parent, value.identifier, argument_names
        )
    };
    rust_code += &*format!(
        r#"
pub extern "C" fn boxed__{}__{}(ptr: *mut {}{}){} {{
    let value: &{} = unsafe {{ &(*ptr) }};
    {}
}}
"#,
        value.parent, value.identifier, value.parent, arguments, return_value, value.parent, body
    );
    CodegenOutput {
        rust_code,
        swift_code: "".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_opaque_method_rust() {
        let opaque_value = generate_opaque_method(OpaqueValueMethod {
            parent: "LooperEngine".to_string(),
            identifier: "trigger".to_string(),
            arguments: vec![],
            return_value: None,
        });
        assert_eq!(
            opaque_value.rust_code,
            r#"
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *mut LooperEngine) {
    let value: &LooperEngine = unsafe { &(*ptr) };
    LooperEngine::trigger(value);
}
"#
        );
    }

    #[test]
    fn test_generate_opaque_method_rust_with_arguments() {
        let opaque_value = generate_opaque_method(OpaqueValueMethod {
            parent: "LooperEngine".to_string(),
            identifier: "trigger".to_string(),
            arguments: vec![
                ("something".to_string(), "f32".to_string()),
                ("something_else".to_string(), "i32".to_string()),
            ],
            return_value: None,
        });
        assert_eq!(
            opaque_value.rust_code,
            r#"
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *mut LooperEngine, something: f32, something_else: i32) {
    let value: &LooperEngine = unsafe { &(*ptr) };
    LooperEngine::trigger(value, something, something_else);
}
"#
        );
    }

    #[test]
    fn test_generate_opaque_method_rust_with_return_value() {
        let opaque_value = generate_opaque_method(OpaqueValueMethod {
            parent: "LooperEngine".to_string(),
            identifier: "trigger".to_string(),
            arguments: vec![],
            return_value: Some("SomeOtherValue".to_string()),
        });
        assert_eq!(
            opaque_value.rust_code,
            r#"
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *mut LooperEngine) -> *mut SomeOtherValue {
    let value: &LooperEngine = unsafe { &(*ptr) };
    let result = LooperEngine::trigger(value);
    Box::into_raw(Box::new(result))
}
"#
        );
    }

    #[test]
    fn test_generate_opaque_rust() {
        let opaque_value = generate_opaque_value(OpaqueValueInput {
            identifier: "LooperEngine".to_string(),
        });
        assert_eq!(
            opaque_value.rust_code,
            r#"
#[no_mangle]
pub extern "C" fn boxed__LooperEngine__delete(ptr: *mut LooperEngine) {
    let _ = unsafe { Box::from_raw(ptr) };
}
"#
        )
    }

    #[test]
    fn test_generate_opaque_value_swift() {
        let opaque_value = generate_opaque_value(OpaqueValueInput {
            identifier: "LooperEngine".to_string(),
        });
        assert_eq!(
            opaque_value.swift_code,
            r"class Boxed$LooperEngine {
    let __innerPtr: OpaquePointer
    init(rawPtr: OpaquePointer) { self.__innerPtr = rawPtr }
    deinit { LooperEngine__delete(self.__innerPtr) }
}
"
        )
    }
}
