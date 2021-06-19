pub mod audio_io;
pub mod commands;
mod processors;
mod timer;
pub mod vst_host;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
