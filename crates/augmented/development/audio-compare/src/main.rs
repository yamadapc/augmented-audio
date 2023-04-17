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

use std::fs::File;
use std::io::BufReader;
use std::iter::repeat;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use dasp::Signal;
use hound::{SampleFormat, WavReader, WavSpec};
use num_complex::Complex;
use rayon::prelude::*;
use rustfft::FftPlanner;

use crate::model::{Args, AudioMetadata, AudioSimilarityResult, Commands, CompareResults};
use crate::server::start_server;

mod logger;
mod model;
mod server;

#[tokio::main]
async fn main() {
    let _ = logger::try_init_from_env();

    let args = Args::parse();
    match args.command {
        Commands::Run {
            targets,
            run_server,
        } => exec_run(targets, run_server).await,
        Commands::GenerateInputFiles { output_path } => {
            exec_generate_input_files(&output_path).await
        }
        Commands::Test {
            delta_similarity_threshold,
            spectral_correlation_similarity_threshold,
            cross_correlation_similarity_threshold,
            new_test_file,
            snapshot_file,
        } => {
            let (_, result) = run_compare(vec![snapshot_file, new_test_file]);
            log::info!("Received results {:#?}", result);
            let similarity = &result.similarities[0];
            let test_cases = [
                (
                    cross_correlation_similarity_threshold,
                    similarity.cross_correlation_similarity,
                    "Cross correlation",
                ),
                (
                    spectral_correlation_similarity_threshold,
                    similarity.spectral_similarity,
                    "Spectral correlation",
                ),
                (
                    delta_similarity_threshold,
                    1.0 - similarity.delta_magnitude,
                    "Delta similarity",
                ),
            ];
            let mut has_failed = false;
            for (threshold, value, label) in test_cases {
                if value > threshold {
                    log::info!("{} - OK - value={} threshold={}", label, value, threshold);
                } else {
                    has_failed = true;
                    log::error!(
                        "{} - FAILURE - value={} threshold={}",
                        label,
                        value,
                        threshold
                    );
                }
            }
            if has_failed {
                std::process::exit(1);
            }
        }
    }
}

async fn exec_generate_input_files(output_path: &str) {
    let output_path = Path::new(output_path);
    let create_file = |name, contents: Vec<[f32; 2]>| {
        let mut writer = hound::WavWriter::create(
            output_path.join(name),
            WavSpec {
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 32,
                sample_format: SampleFormat::Float,
            },
        )
        .unwrap();
        for frame in contents {
            writer.write_sample(frame[0]).unwrap();
            writer.write_sample(frame[1]).unwrap();
        }
        writer.finalize().unwrap();
    };

    let mut pulse = vec![];
    pulse.resize(44100, [0.0; 2]);
    pulse[0][0] = 1.0;
    pulse[0][1] = 1.0;
    create_file("pulse.wav", pulse);

    let mut sine = vec![];
    sine.resize(44100, [0.0; 2]);
    for (index, frame) in sine.iter_mut().enumerate() {
        let time = index as f32 / 44100.0;
        let value = (time * 440.0 * 2.0 * std::f32::consts::PI).sin();
        frame[0] = value;
        frame[1] = value;
    }
    create_file("sine.wav", sine);

    let mut sine_pulse = vec![];
    let envelope = augmented_adsr_envelope::Envelope::new();
    envelope.set_sample_rate(44100.0);
    sine_pulse.resize(44100 * 4, [0.0; 2]);
    envelope.note_on();
    envelope.set_attack(Duration::from_millis(1));
    let note_duration = 5000;
    for (index, frame) in sine_pulse.iter_mut().enumerate() {
        let envelope_value = envelope.volume();
        let time = index as f32 / 44100.0;
        let value = (time * 440.0 * 2.0 * std::f32::consts::PI).sin();
        frame[0] = value * envelope_value;
        frame[1] = value * envelope_value;
        envelope.tick();
        if index == note_duration {
            envelope.note_off();
        }
    }
    create_file("sine_pulse.wav", sine_pulse);
}

async fn exec_run(targets: Vec<String>, run_server: bool) {
    let (image_paths, compare_results) = run_compare(targets);

    if run_server {
        start_server(image_paths, compare_results).await
    }
}

fn run_compare(targets: Vec<String>) -> (Vec<String>, Arc<CompareResults>) {
    log::debug!("Opening files {:#?}", targets);
    let readers = targets
        .iter()
        .map(|path| {
            (
                path.clone(),
                hound::WavReader::open(path).expect("Failed to open WAV file"),
            )
        })
        .collect::<Vec<_>>();

    let metadatas = readers
        .iter()
        .map(|(path, reader)| AudioMetadata {
            path: path.clone(),
            filename: Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            duration_samples: reader.duration() as u32,
            duration_seconds: Duration::from_millis(
                (reader.duration() / reader.spec().sample_rate * 1000 as u32) as u64,
            )
            .as_secs_f32(),
            spec: reader.spec().into(),
        })
        .collect::<Vec<_>>();
    log::info!("Read metadatas: {:#?}", metadatas);
    let files = readers
        .into_iter()
        .map(|(name, reader)| {
            (
                name,
                read_file(reader)
                    .until_exhausted()
                    .collect::<Vec<[f32; 2]>>(),
            )
        })
        .collect::<Vec<(String, Vec<[f32; 2]>)>>();
    log::debug!("Read files.");

    let image_paths = files
        .iter()
        .map(|(name, file)| draw_audio_file(name, file))
        .collect::<Vec<_>>();

    let mut similarities = Vec::new();

    for (i, (name1, file1)) in files.iter().enumerate() {
        for (j, (name2, file2)) in files.iter().enumerate().skip(i) {
            if i != j {
                let cross_correlation_similarity =
                    compute_cross_correlation_similarity(file1, file2);
                let spectral_similarity = compute_spectral_similarity(file1, file2);
                let delta_magnitude = compute_delta_magnitude(file1, file2);
                let deltas: Vec<f32> = file1
                    .iter()
                    .zip(file2.iter())
                    .map(|(frame1, frame2)| (frame1[0] - frame2[0]).abs())
                    .take(10000)
                    .collect();

                audio_processor_testing_helpers::charts::draw_vec_chart(
                    &format!("{}-{}-{}", name1, i, j),
                    "audio",
                    deltas,
                );

                log::info!(
                    "Similarity between file1={} file2={} cross_correlation_similarity={} spectral_similarity={} delta_magnitude={}",
                    name1,
                    name2,
                    cross_correlation_similarity,
                    spectral_similarity,
                    delta_magnitude
                );
                similarities.push(AudioSimilarityResult {
                    file1: name1.clone(),
                    file2: name2.clone(),
                    cross_correlation_similarity: cross_correlation_similarity.max(0.0),
                    spectral_similarity: spectral_similarity.max(0.0),
                    delta_magnitude: delta_magnitude.max(0.0),
                });
            }
        }
    }

    let compare_results = std::sync::Arc::new(CompareResults {
        similarities,
        metadatas,
    });
    (image_paths, compare_results)
}

fn draw_audio_file(name: &str, file: &[[f32; 2]]) -> String {
    audio_processor_testing_helpers::charts::draw_vec_chart(
        &name,
        "audio",
        file.iter()
            .map(|[l, r]| l + r)
            .take(10000)
            .collect::<Vec<_>>(),
    );
    format!("{}--{}.png", name, "audio")
}

fn read_file(reader: WavReader<BufReader<File>>) -> impl Signal<Frame = [f32; 2]> {
    let spec = reader.spec();
    let duration = reader.duration();
    log::debug!("Read spec: {:#?}", spec);
    log::debug!("Read duration: {:#?}", duration);

    dasp::signal::from_interleaved_samples_iter::<_, [f32; 2]>(
        reader
            .into_samples::<f32>()
            .map(|sample| sample.expect("Failed to read file")),
    )
}

fn compute_delta_magnitude(signal1: &[[f32; 2]], signal2: &[[f32; 2]]) -> f32 {
    let left1: Vec<f32> = signal1.iter().map(|frame| frame[0]).collect();
    let left2: Vec<f32> = signal2.iter().map(|frame| frame[0]).collect();
    let right1: Vec<f32> = signal1.iter().map(|frame| frame[1]).collect();
    let right2: Vec<f32> = signal2.iter().map(|frame| frame[1]).collect();

    let delta_magnitudes: f32 = [(left1, left2), (right1, right2)]
        .par_iter()
        .map(|(s1, s2)| -> f32 {
            // let norm: f32 =
            //     s1.iter().map(|x| x.abs()).sum::<f32>() + s2.iter().map(|x| x.abs()).sum::<f32>();
            let delta_sum = compute_delta_magnitude_mono(s1, s2);
            delta_sum / (s1.len().max(s2.len()) as f32)
        })
        .sum();

    delta_magnitudes
}

fn compute_delta_magnitude_mono(signal1: &[f32], signal2: &[f32]) -> f32 {
    let mut delta_magnitude = 0.0;
    for (s1, s2) in signal1.iter().zip(signal2.iter()) {
        delta_magnitude += (s1 - s2).abs();
    }
    delta_magnitude
}

fn compute_cross_correlation_similarity(signal1: &[[f32; 2]], signal2: &[[f32; 2]]) -> f32 {
    let left1: Vec<f32> = signal1.iter().map(|frame| frame[0]).collect();
    let left2: Vec<f32> = signal2.iter().map(|frame| frame[0]).collect();

    let right1: Vec<f32> = signal1.iter().map(|frame| frame[1]).collect();
    let right2: Vec<f32> = signal2.iter().map(|frame| frame[1]).collect();

    let similarity_sum: f32 = [(left1, left2), (right1, right2)]
        .par_iter()
        .map(|(s1, s2)| compute_cross_correlation_similarity_mono(s1, s2))
        .sum();

    println!("{}", similarity_sum);

    similarity_sum / 2.0
}

fn compute_cross_correlation_similarity_mono(signal1: &[f32], signal2: &[f32]) -> f32 {
    if signal1 == signal2 {
        return 1.0;
    }
    let len1 = signal1.len();
    let len2 = signal2.len();

    let padded_len = (len1 + len2 - 1).next_power_of_two();
    let padded_signal1 = signal1
        .iter()
        .cloned()
        .chain(repeat(0.0).take(padded_len - len1))
        .collect::<Vec<_>>();
    let padded_signal2 = signal2
        .iter()
        .cloned()
        .chain(repeat(0.0).take(padded_len - len2))
        .collect::<Vec<_>>();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(padded_len);

    let mut freq_domain1 = padded_signal1
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<_>>();
    let mut freq_domain2 = padded_signal2
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<_>>();

    fft.process(&mut freq_domain1);
    fft.process(&mut freq_domain2);

    let mut cross_spectrum = freq_domain1
        .iter()
        .zip(freq_domain2.iter())
        .map(|(a, b)| a * b.conj())
        .collect::<Vec<_>>();
    let ifft = planner.plan_fft_inverse(padded_len);
    ifft.process(&mut cross_spectrum);

    let norm1 = signal1.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm2 = signal2.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();

    let max_cross_corr = cross_spectrum
        .iter()
        .map(|x| x.re / padded_len as f32)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let norm = (norm1 * norm2).max(f32::EPSILON);
    max_cross_corr / norm
}

fn compute_spectral_similarity(signal1: &[[f32; 2]], signal2: &[[f32; 2]]) -> f32 {
    if signal1 == signal2 {
        return 1.0;
    }
    let left1: Vec<f32> = signal1.iter().map(|frame| frame[0]).collect();
    let left2: Vec<f32> = signal2.iter().map(|frame| frame[0]).collect();

    let right1: Vec<f32> = signal1.iter().map(|frame| frame[1]).collect();
    let right2: Vec<f32> = signal2.iter().map(|frame| frame[1]).collect();

    let similarity_sum: f32 = [(left1, left2), (right1, right2)]
        .par_iter()
        .map(|(s1, s2)| compute_spectral_similarity_mono(s1, s2))
        .sum();

    similarity_sum / 2.0
}

fn compute_spectral_similarity_mono(signal1: &[f32], signal2: &[f32]) -> f32 {
    if signal1 == signal2 {
        return 1.0;
    }
    let len1 = signal1.len();
    let len2 = signal2.len();

    let padded_len = len1.max(len2).next_power_of_two();
    let padded_signal1 = signal1
        .iter()
        .cloned()
        .chain(repeat(0.0).take(padded_len - len1))
        .collect::<Vec<_>>();
    let padded_signal2 = signal2
        .iter()
        .cloned()
        .chain(repeat(0.0).take(padded_len - len2))
        .collect::<Vec<_>>();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(padded_len);

    let mut freq_domain1 = padded_signal1
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<_>>();
    let mut freq_domain2 = padded_signal2
        .iter()
        .map(|x| Complex::new(*x, 0.0))
        .collect::<Vec<_>>();

    fft.process(&mut freq_domain1);
    fft.process(&mut freq_domain2);

    let magnitude1 = freq_domain1.iter().map(|x| x.norm()).collect::<Vec<f32>>();
    let magnitude2 = freq_domain2.iter().map(|x| x.norm()).collect::<Vec<f32>>();

    let dot_product = magnitude1
        .iter()
        .zip(magnitude2.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>();

    let norm1 = magnitude1.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let norm2 = magnitude2.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();

    dot_product / (norm1 * norm2).max(f32::EPSILON)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cross_correlation_similarity_of_null_signal() {
        let signal1 = vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        let signal2 = vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        let similarity = compute_cross_correlation_similarity(&signal1, &signal2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_cross_correlation_similarity_of_same_signal() {
        let signal1 = vec![[1.0, 1.0], [1.0, 1.0], [1.0, 1.0], [1.0, 1.0]];
        let signal2 = vec![[1.0, 1.0], [1.0, 1.0], [1.0, 1.0], [1.0, 1.0]];
        let similarity = compute_cross_correlation_similarity(&signal1, &signal2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_identical_signals() {
        let signal1 = vec![0.5, 0.2, 0.8, 0.3];
        let signal2 = signal1.clone();
        let similarity = compute_cross_correlation_similarity_mono(&signal1, &signal2);
        assert!(
            (similarity - 1.0).abs() < 1e-6,
            "Similarity for identical signals should be 1.0"
        );
    }

    #[test]
    fn test_noisy_and_unrelated_signals() {
        let signal1 = vec![1.0, -1.0, 1.0, -1.0];
        let signal2 = vec![2.0, 2.0, -1.0, -1.0];
        let similarity_unrelated = compute_cross_correlation_similarity_mono(&signal1, &signal2);

        let noise = vec![0.1, -0.1, 0.1, -0.1];
        let signal1_noisy: Vec<f32> = signal1
            .iter()
            .zip(noise.iter())
            .map(|(s, n)| s + n)
            .collect();
        let similarity_noisy = compute_cross_correlation_similarity_mono(&signal1, &signal1_noisy);

        assert!(
            similarity_unrelated < similarity_noisy,
            "Similarity for unrelated signals should be less than similarity for related signals with noise"
        );
    }

    #[test]
    fn test_shifted_signals() {
        let signal1 = vec![0.5, 0.2, 0.8, 0.3, 0.0, 0.0];
        let signal2 = vec![0.0, 0.0, 0.5, 0.2, 0.8, 0.3];
        let similarity = compute_cross_correlation_similarity_mono(&signal1, &signal2);
        assert!(
            (similarity - 1.0).abs() < 1e-6,
            "Similarity for shifted signals should be 1.0"
        );
    }

    #[test]
    fn test_spectral_similarity_identical_signals() {
        let signal1 = vec![0.5, 0.2, 0.8, 0.3];
        let signal2 = signal1.clone();
        let similarity = compute_spectral_similarity_mono(&signal1, &signal2);
        assert!(
            (similarity - 1.0).abs() < 1e-6,
            "Spectral similarity for identical signals should be 1.0"
        );
    }

    #[test]
    fn test_spectral_similarity_shifted_signals() {
        let signal1 = vec![0.5, 0.2, 0.8, 0.3, 0.0, 0.0];
        let signal2 = vec![0.0, 0.0, 0.5, 0.2, 0.8, 0.3];
        let similarity = compute_spectral_similarity_mono(&signal1, &signal2);
        assert!(
            (similarity - 1.0).abs() < 1e-6,
            "Spectral similarity for shifted signals should be 1.0"
        );
    }
}
