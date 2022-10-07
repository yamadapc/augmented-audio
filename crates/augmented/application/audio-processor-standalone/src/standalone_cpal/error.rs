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

use crate::standalone_cpal::options::ConfigureDeviceError;

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AudioThreadError {
    #[error("Failed to configure device: {0}")]
    ConfigureDeviceError(#[from] ConfigureDeviceError),
    #[error("Failed to configure input stream: {0}")]
    BuildInputStreamError(cpal::BuildStreamError),
    #[error("Failed to configure output stream: {0}")]
    BuildOutputStreamError(cpal::BuildStreamError),
    #[error("Failed to start input stream: {0}")]
    InputStreamError(cpal::PlayStreamError),
    #[error("Failed to start output stream: {0}")]
    OutputStreamError(cpal::PlayStreamError),
}
