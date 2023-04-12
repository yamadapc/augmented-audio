use std::fs::File;
use std::io::BufReader;
use std::iter::repeat;
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use dasp::Signal;
use hound::{WavReader, WavSpec};
use num_complex::Complex;
use rayon::prelude::*;
use rustfft::FftPlanner;
use serde::{Deserialize, Serialize};
use warp::Filter;

mod logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    Float,
    Int,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
struct Spec {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub sample_format: SampleFormat,
}

impl From<WavSpec> for Spec {
    fn from(value: WavSpec) -> Self {
        Spec {
            channels: value.channels,
            sample_rate: value.sample_rate,
            bits_per_sample: value.bits_per_sample,
            sample_format: match value.sample_format {
                hound::SampleFormat::Float => SampleFormat::Float,
                hound::SampleFormat::Int => SampleFormat::Int,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AudioMetadata {
    path: String,
    filename: String,
    duration_samples: u32,
    duration_seconds: f32,
    spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AudioSimilarityResult {
    file1: String,
    file2: String,
    similarity: f32,
}

#[derive(Serialize, Deserialize)]
struct CompareResults {
    similarities: Vec<AudioSimilarityResult>,
    metadatas: Vec<AudioMetadata>,
}

#[tokio::main]
async fn main() {
    let _ = logger::try_init_from_env();

    let args = Args::parse();
    log::debug!("Opening files {:#?}", args.targets);
    let readers = args
        .targets
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
                let similarity = compute_cross_correlation_similarity(file1, file2);
                log::info!("Similarity between {} and {}: {}", name1, name2, similarity);
                similarities.push(AudioSimilarityResult {
                    file1: name1.clone(),
                    file2: name2.clone(),
                    similarity,
                });
            }
        }
    }

    let compare_results = std::sync::Arc::new(CompareResults {
        similarities,
        metadatas,
    });

    let images_dir = tempdir::TempDir::new("audio-compare").expect("Failed to create temp dir");
    std::fs::create_dir_all(format!("{}/images", images_dir.path().display()))
        .expect("Failed to create images dir");
    image_paths.iter().for_each(|image| {
        let image_name = Path::new(image).file_name().unwrap().to_str().unwrap();
        let target = format!("{}/images/{}", images_dir.path().display(), image_name);
        log::info!("Moving image {} to {}", image, target);
        std::fs::rename(image, target).expect("Failed to move image file")
    });

    let images_dir_route = warp::path("images")
        .and(warp::fs::dir(images_dir.path().join("images")))
        .with(warp::log("images"));
    let home_route = warp::get()
        .and(warp::path::end())
        .map(move || handle_get_home(&compare_results))
        .with(warp::log("home"));
    warp::serve(home_route.or(images_dir_route))
        .run(([127, 0, 0, 1], 3030))
        .await
}

fn draw_audio_file(name: &str, file: &[[f32; 2]]) -> String {
    audio_processor_testing_helpers::charts::draw_vec_chart(
        &name,
        "audio",
        file.iter().map(|[l, r]| l + r).collect::<Vec<_>>(),
    );
    format!("{}--{}.png", name, "audio")
}

fn handle_get_home(results: &CompareResults) -> warp::reply::Html<String> {
    let mut context = tera::Context::new();
    context.insert("metadatas", &results.metadatas);
    context.insert("similarities", &results.similarities);
    let result = tera::Tera::one_off(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Audio Compare</title>
        <style>

body {
  font-family: sans-serif;
}        

table {
  border-spacing: 0;
}

table th,
table td {
  text-align: left;
  border: 1px solid rgba(0,0,0,0.3);
  padding: 2px 4px;
  box-sizing: border-box;
}

table td img {
  max-width: 200px;
  max-height: 200px;
}
        </style>
    </head>
    <body>
        <h1>Audio Compare</h1>
        <h2>Metadata</h2>
        <table>
             <thead>
                 <tr>
                      <th>File</th>
                      <th>Duration seconds</th>
                      <th>Sample rate</th>
                      <th>Channels</th>
                      <th>Bits per sample</th>
                      <th>Image</th>
                 </tr>
                </thead>
                <tbody>
                 {% for metadata in metadatas %}
                      <tr>
                            <td>{{ metadata.path }}</td>
                            <td>{{ metadata.duration_seconds }}</td>
                            <td>{{ metadata.spec.sample_rate }}Hz</td>
                            <td>{{ metadata.spec.channels }}</td>
                            <td>{{ metadata.spec.bits_per_sample }}</td>
                            <td>
                                <img src="/images/{{ metadata.filename }}--audio.png" />
                            </td>
                      </tr>
                 {% endfor %}
             </tbody>
        </table>

        <h2>Cross-correlation similarity</h2>
        <table>
            <thead>
                <tr>
                    <th>File 1</th>
                    <th>File 2</th>
                    <th>Similarity</th>
                </tr>
            </thead>
            <tbody>
                {% for similarity in similarities %}
                    <tr>
                        <td>{{ similarity.file1 }}</td>
                        <td>{{ similarity.file2 }}</td>
                        <td>{{ similarity.similarity }}</td>
                    </tr>
                {% endfor %}
            </tbody>
        </table>
    </body>
</html>        
        "#,
        &mut context,
        false,
    );

    warp::reply::html(result.unwrap_or_else(|err| {
        log::error!("Failed to compile template: {}", err);
        format!("Failed to compile template: {}", err)
    }))
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

fn compute_cross_correlation_similarity(signal1: &[[f32; 2]], signal2: &[[f32; 2]]) -> f32 {
    let left1: Vec<f32> = signal1.iter().map(|frame| frame[0]).collect();
    let left2: Vec<f32> = signal2.iter().map(|frame| frame[0]).collect();

    let right1: Vec<f32> = signal1.iter().map(|frame| frame[1]).collect();
    let right2: Vec<f32> = signal2.iter().map(|frame| frame[1]).collect();

    let similarity_sum: f32 = [(left1, left2), (right1, right2)]
        .par_iter()
        .map(|(s1, s2)| compute_cross_correlation_similarity_mono(s1, s2))
        .sum();

    similarity_sum / 2.0
}

fn compute_cross_correlation_similarity_mono(signal1: &[f32], signal2: &[f32]) -> f32 {
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

    log::debug!("Max cross correlation: {}", max_cross_corr);
    log::debug!("Norms: {} {}", norm1, norm2);
    max_cross_corr / (norm1 * norm2)
}
