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

use clap::{Parser, Subcommand};
use hound::WavSpec;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test {
        #[arg(long, short)]
        new_test_file: String,
        #[arg(long, short)]
        snapshot_file: String,
        #[arg(long, short)]
        cross_correlation_similarity_threshold: f32,
        #[arg(long, short)]
        spectral_correlation_similarity_threshold: f32,
        #[arg(long, short)]
        delta_similarity_threshold: f32,
    },
    Run {
        targets: Vec<String>,
        #[arg(long, short)]
        run_server: bool,
    },
    GenerateInputFiles {
        #[arg(long, short)]
        output_path: String,
    },
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
    pub delta_magnitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareResults {
    pub similarities: Vec<AudioSimilarityResult>,
    pub metadatas: Vec<AudioMetadata>,
}
