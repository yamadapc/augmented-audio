use std::ops::Add;

use super::CodegenOutput;

pub struct OpaqueValueInput {
    identifier: String,
}

pub struct OpaqueValueMethod {
    parent: String,
    identifier: String,
    arguments: Vec<String>,
    return_value: String,
}

pub fn generate_opaque_value(input: OpaqueValueInput) -> CodegenOutput {
    let mut rust_code = "".to_string();
    rust_code += &*format!(
        r#"
#[no_mangle]
pub extern "C" fn {}__delete(ptr: *mut {}) {{
    let _ = unsafe {{ Box::from_raw(ptr) }};
}}
"#,
        input.identifier, input.identifier
    );
    let destructor_c_name = format!("{}__delete", input.identifier);

    let ident = "Native$".to_string() + &*input.identifier;
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

fn generate_opaque_method(value: OpaqueValueMethod) -> CodegenOutput {
    let mut rust_code = "".to_string();
    rust_code += &*format!(
        r#"
pub extern "C" fn {}__{}(ptr: *mut {}) {{
    let value: &{} = unsafe {{ &(*ptr) }};
    {}::{}(value);
}}
"#,
        value.parent, value.identifier, value.parent, value.parent, value.parent, value.identifier
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
            return_value: "".to_string(),
        });
        assert_eq!(
            opaque_value.rust_code,
            r#"
pub extern "C" fn LooperEngine__trigger(ptr: *mut LooperEngine) {
    let value: &LooperEngine = unsafe { &(*ptr) };
    LooperEngine::trigger(value);
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
pub extern "C" fn LooperEngine__delete(ptr: *mut LooperEngine) {
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
            r"class Native$LooperEngine {
    let __innerPtr: OpaquePointer
    init(rawPtr: OpaquePointer) { self.__innerPtr = rawPtr }
    deinit { LooperEngine__delete(self.__innerPtr) }
}
"
        )
    }
}
