/// Perform sample rate conversion of a buffer using linear interpolation
pub fn convert_sample_rate(input_rate: f32, input: &[f32], output_rate: f32, output: &mut [f32]) {
    if (output_rate - input_rate).abs() < f32::EPSILON {
        for (idx, sample) in input.iter().enumerate() {
            output[idx] = *sample;
        }
        return;
    }

    // Up-sample -> Output has higher sample rate
    if output_rate > input_rate {
        // Number of input samples per output sample
        // ex. output:88kHz - input:44kHz = 0.5 input samples per output sample
        let input_samples_per_output = input_rate / output_rate;

        for (sample_index, output_sample) in output.iter_mut().enumerate() {
            let input_index: f32 = input_samples_per_output * (sample_index as f32);
            let input_index_floor = input_index.floor() as usize;

            let base_sample = input[input_index_floor];
            let next_sample = if input_index_floor + 1 < input.len() {
                input[input_index_floor + 1]
            } else {
                0.0
            };

            let delta = input_index - (input_index_floor as f32);
            let result = (1.0 - delta) * base_sample + delta * next_sample;

            *output_sample = result;
        }

        return;
    }

    // Down-sample -> Output has lower sample rate
    let output_samples_per_input = output_rate / input_rate;
    for sample_index in 0..input.len() {
        let output_index = output_samples_per_input * (sample_index as f32);
        let output_index_floor = output_index.floor() as usize;

        let base_sample = input[sample_index];
        let next_sample = if sample_index + 1 < input.len() {
            input[sample_index + 1]
        } else {
            0.0
        };

        let delta = output_index - (output_index_floor as f32);
        let result = (1.0 - delta) * base_sample + delta * next_sample;

        output[output_index_floor] = result;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use oscillator::generators::sine_generator;
    use oscillator::Oscillator;

    use super::*;

    fn sine_buffer(sample_rate: f32, frequency: f32, length: Duration) -> Vec<f32> {
        let mut source = Oscillator::new(sine_generator);
        source.set_sample_rate(sample_rate);
        source.set_frequency(frequency);
        let mut output = Vec::new();
        let length_samples = (length.as_secs_f32() * sample_rate).ceil();
        output.resize(length_samples as usize, 0.0);
        for sample in &mut output {
            *sample = source.get();
        }
        output
    }

    fn rms_level(input: &[f32]) -> f32 {
        if input.is_empty() {
            return 0.0;
        }
        let mut s = 0.0;
        for i in input {
            s += i.abs();
        }
        s / (input.len() as f32)
    }

    #[test]
    fn test_converting_sample_rate_will_not_change_the_level() {
        let mut input_output_rates = vec![
            [44100.0, 22100.0],
            [44100.0, 15000.0],
            [44100.0, 10000.0],
            [22100.0, 10000.0],
            [22100.0, 20000.0],
            [22100.0, 92000.0],
        ];

        // generate random test cases
        for _ in 0..100 {
            let input_rate = 22000.0 + rand::random::<f32>() * 172000.0;
            let output_rate = 22000.0 + rand::random::<f32>() * 172000.0;
            input_output_rates.push([input_rate, output_rate])
        }

        let duration = Duration::from_secs(1);
        let input_window_size = 512;

        for [input_rate, output_rate] in input_output_rates {
            println!("Input rate: {} Output rate: {}", input_rate, output_rate);
            let input_buffer = sine_buffer(input_rate, 440.0, duration);
            let mut output_buffer = Vec::new();
            output_buffer.resize((duration.as_secs_f32() * output_rate).ceil() as usize, 0.0);
            convert_sample_rate(input_rate, &input_buffer, output_rate, &mut output_buffer);

            let output_window_size =
                ((output_rate / input_rate) * input_window_size as f32) as usize;
            test_level_equivalence(
                input_buffer,
                &mut output_buffer,
                input_window_size,
                output_window_size,
            )
        }
    }

    fn test_level_equivalence(
        input_buffer: Vec<f32>,
        output_buffer: &mut Vec<f32>,
        input_window_size: usize,
        output_window_size: usize,
    ) {
        let input_chunks = input_buffer.chunks(input_window_size);
        let output_chunks = output_buffer.chunks(output_window_size);
        assert!(input_buffer.len() > 0);
        assert!(output_buffer.len() > 0);
        // assert!((input_chunks.len() as i32 - output_chunks.len() as i32).abs() < 2);
        for (input_chunk, output_chunk) in input_chunks.zip(output_chunks) {
            let input_level = rms_level(input_chunk);
            let output_level = rms_level(output_chunk);
            assert!(input_level - output_level < f32::EPSILON);
        }
    }
}
