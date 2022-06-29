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
use std::collections::HashSet;

use crate::generate_swift_enum::capitalize;

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
pub extern "C" fn boxed__{}__delete(ptr: *const {}) {{
    let _ = unsafe {{ Box::from_raw(ptr as *mut {}) }};
}}
"#,
        input.identifier, input.identifier, input.identifier
    );
    let destructor_c_name = format!("{}__delete", input.identifier);

    let ident = "Boxed$".to_string() + &*input.identifier;
    let mut swift_code = "".to_string();
    swift_code += &*format!("class {} {{\n", ident);
    swift_code += "    let __innerPtr: OpaquePointer\n";
    swift_code += "    init(rawPtr: OpaquePointer) { self.__innerPtr = rawPtr }\n";
    swift_code += &*format!("    deinit {{ {}(self.__innerPtr) }}\n", destructor_c_name);
    swift_code += "}\n";

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
        .map(|v| format!(" -> *const {}", v))
        .unwrap_or_else(|| "".to_string());

    let mut primitives = HashSet::new();
    primitives.insert("f32");
    primitives.insert("f64");
    primitives.insert("bool");
    primitives.insert("i32");
    primitives.insert("i64");
    primitives.insert("usize");

    let should_box = value.return_value.is_some()
        && !primitives.contains(&*value.return_value.as_ref().cloned().unwrap());
    let body = if should_box {
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
pub extern "C" fn boxed__{}__{}(ptr: *const {}{}){} {{
    let value: &{} = unsafe {{ &(*ptr) }};
    {}
}}
"#,
        value.parent, value.identifier, value.parent, arguments, return_value, value.parent, body
    );

    let argument_names = value
        .arguments
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();
    let mut swift_code = "".to_string();
    let swift_method_name = to_camel_case(&*value.identifier);
    let parent_name = format!("Boxed${}", value.parent);
    swift_code += &*format!(
        r#"
extension {} {{
    func {}({}){} {{
        boxed__{}__{}({})
    }}
}}
    "#,
        parent_name,
        swift_method_name,
        value
            .arguments
            .iter()
            .map(|(identifier, ty)| { format!("{}: {}", identifier, ty) })
            .collect::<Vec<String>>()
            .join(", "),
        value
            .return_value
            .map(|ret| format!(" -> {}", ret))
            .unwrap_or_else(|| "".to_string()),
        value.parent,
        value.identifier,
        argument_names.join(", ")
    );

    CodegenOutput {
        rust_code,
        swift_code,
    }
}

fn to_camel_case(identifier: &str) -> String {
    let parts = identifier.split('_');
    let mut result = "".to_string();
    let mut first = true;
    for part in parts {
        if first {
            result += part;
            first = false;
        } else {
            result += &capitalize(part)
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        let inp = "something_here";
        assert_eq!(to_camel_case(inp), "somethingHere");
    }

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
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *const LooperEngine) {
    let value: &LooperEngine = unsafe { &(*ptr) };
    LooperEngine::trigger(value)
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
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *const LooperEngine, something: f32, something_else: i32) {
    let value: &LooperEngine = unsafe { &(*ptr) };
    LooperEngine::trigger(value, something, something_else)
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
pub extern "C" fn boxed__LooperEngine__trigger(ptr: *const LooperEngine) -> *const SomeOtherValue {
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
pub extern "C" fn boxed__LooperEngine__delete(ptr: *const LooperEngine) {
    let _ = unsafe { Box::from_raw(ptr as *mut LooperEngine) };
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
