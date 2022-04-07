use samplerate::ConverterType;

pub fn convert_sample_rate(input_rate: f32, input: &[f32], output_rate: f32, output: &mut [f32]) {
    if (input_rate - output_rate).abs() < f32::EPSILON {
        for (s, d) in input.iter().zip(output.iter_mut()) {
            *d = *s;
        }
        return;
    }

    let result = samplerate::convert(
        input_rate as u32,
        output_rate as u32,
        1,
        ConverterType::SincBestQuality,
        input,
    )
    .unwrap();
    for (sample, out) in result.iter().zip(output.iter_mut()) {
        *out = *sample;
    }
}
