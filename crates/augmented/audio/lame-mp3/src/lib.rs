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
use std::process::Command;
use std::process::ExitStatus;

pub fn convert_wav_file_to_mp3(
    wav_file_path: &str,
    mp3_file_path: &str,
) -> std::io::Result<ExitStatus> {
    let mut result = Command::new("lame")
        .arg(wav_file_path)
        .arg(mp3_file_path)
        .spawn()?;
    result.wait()
}

#[cfg(test)]
mod tests {
    use crate::convert_wav_file_to_mp3;

    #[test]
    fn it_can_encode_mp3() {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{}/test-inputs/synth.wav", crate_dir);
        let output_path = format!("{}/test-inputs/synth.mp3", crate_dir);
        let exit_code = convert_wav_file_to_mp3(&input_path, &output_path).unwrap();
        assert!(exit_code.success());
    }
}
