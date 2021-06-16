pub mod audio_io;
mod audio_settings;
pub mod commands;
pub mod host;
mod timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
