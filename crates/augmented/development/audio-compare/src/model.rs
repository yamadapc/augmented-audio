use clap::Parser;
use hound::WavSpec;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub targets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    Float,
    Int,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spec {
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
pub struct AudioMetadata {
    pub path: String,
    pub filename: String,
    pub duration_samples: u32,
    pub duration_seconds: f32,
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AudioSimilarityResult {
    pub file1: String,
    pub file2: String,
    pub cross_correlation_similarity: f32,
    pub spectral_similarity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CompareResults {
    pub similarities: Vec<AudioSimilarityResult>,
    pub metadatas: Vec<AudioMetadata>,
}
