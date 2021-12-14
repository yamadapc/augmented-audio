use std::process::Command;
use std::process::ExitStatus;

fn convert_wav_file_to_mp3(
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
