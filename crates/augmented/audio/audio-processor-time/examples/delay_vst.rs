use audio_processor_time::MonoDelayProcessor;
use audio_processor_traits::BufferProcessor;

audio_processor_standalone::standalone_vst!(BufferProcessor<MonoDelayProcessor<f32>>);
