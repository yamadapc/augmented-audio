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

    use audio_processor_testing_helpers::sine_buffer;
    use audio_processor_testing_helpers::test_level_equivalence;

    use super::*;

    #[test]
    fn test_converting_sample_rate_will_not_change_the_level() {
        let input_output_rates = vec![
            [44100.0, 22100.0],
            [44100.0, 15000.0],
            [44100.0, 10000.0],
            [22100.0, 10000.0],
            [22100.0, 20000.0],
            [22100.0, 92000.0],
        ];

        let duration = Duration::from_secs(1);
        let input_window_size = 512;

        for [input_rate, output_rate] in input_output_rates {
            println!(
                "==> Input rate: {} Output rate: {}",
                input_rate, output_rate
            );
            let input_buffer = sine_buffer(input_rate, 440.0, duration);
            let mut output_buffer = Vec::new();
            output_buffer.resize((duration.as_secs_f32() * output_rate).ceil() as usize, 0.0);
            convert_sample_rate(input_rate, &input_buffer, output_rate, &mut output_buffer);

            let output_window_size =
                ((output_rate / input_rate) * input_window_size as f32) as usize;
            test_level_equivalence(
                &input_buffer,
                &output_buffer,
                input_window_size,
                output_window_size,
                0.05, // <- Threshold is super high because it does change the level
            )
        }
    }
}
