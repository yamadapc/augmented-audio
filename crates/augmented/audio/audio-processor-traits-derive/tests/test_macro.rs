use audio_processor_traits::parameters::{AudioProcessorHandle, ParameterValue};
use audio_processor_traits::AtomicF32;

#[derive(Default, audio_processor_traits_derive::AudioProcessorHandle)]
struct GainProcessorHandle {
    #[parameter(name = "Value 1")]
    value1: AtomicF32,
    #[parameter(name = "Value 2", min = 30.0, max = 60.0)]
    value2: AtomicF32,
    value3: AtomicF32,
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
