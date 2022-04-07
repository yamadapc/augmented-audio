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
use audio_processor_traits::parameters::{AudioProcessorHandle, ParameterValue};
use audio_processor_traits::AtomicF32;

#[derive(Default, audio_processor_traits_derive::AudioProcessorHandle)]
struct GainProcessorHandle {
    #[parameter(name = "Value 1")]
    value1: AtomicF32,
    #[parameter(name = "Value 2", min = 30.0, max = 60.0)]
    value2: AtomicF32,
    #[allow(unused)]
    value3: AtomicF32,
    #[allow(unused)]
    value4: AtomicF32,
}

#[test]
fn test_values_count_is_correct() {
    let handle = GainProcessorHandle::default();
    assert_eq!(handle.parameter_count(), 2);
    let result = handle.get_parameter_spec(0);
    assert_eq!(result.name(), "Value 1".to_string());
    let result = handle.get_parameter_spec(1);
    assert_eq!(result.name(), "Value 2".to_string());
    assert_eq!(result.ty().float().unwrap().range, (30.0, 60.0));
}

#[test]
fn test_we_can_get_set_values() {
    let handle = GainProcessorHandle::default();
    handle.value1.set(100.0);
    let value = handle.get_parameter(0).unwrap();
    assert_eq!(value, ParameterValue::Float { value: 100.0 });
    handle.set_parameter(0, ParameterValue::Float { value: 200.0 });
    let value = handle.get_parameter(0).unwrap();
    assert_eq!(value, ParameterValue::Float { value: 200.0 });
    let value = handle.value1.get();
    assert_eq!(value, 200.0);
}
