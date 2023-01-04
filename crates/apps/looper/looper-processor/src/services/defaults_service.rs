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

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64),
    Data(Vec<u8>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = &self {
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl From<cacao::defaults::Value> for Value {
    fn from(v: cacao::defaults::Value) -> Self {
        match v {
            cacao::defaults::Value::Bool(v) => Value::Bool(v),
            cacao::defaults::Value::String(v) => Value::String(v),
            cacao::defaults::Value::Float(v) => Value::Float(v),
            cacao::defaults::Value::Integer(v) => Value::Integer(v),
            cacao::defaults::Value::Data(v) => Value::Data(v),
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl From<Value> for cacao::defaults::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::Bool(v) => cacao::defaults::Value::Bool(v),
            Value::String(v) => cacao::defaults::Value::String(v),
            Value::Float(v) => cacao::defaults::Value::Float(v),
            Value::Integer(v) => cacao::defaults::Value::Integer(v),
            Value::Data(v) => cacao::defaults::Value::Data(v),
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn get(key: &str) -> Option<Value> {
    None
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn set(key: &str, value: Value) -> Option<Value> {
    None
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn get(key: &str) -> Option<Value> {
    cacao::defaults::UserDefaults::default()
        .get(key)
        .map(|v| v.into())
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn set(key: &str, value: Value) {
    cacao::defaults::UserDefaults::default()
        .insert(key, value.into())
}
