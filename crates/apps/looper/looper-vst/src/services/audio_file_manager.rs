//! This might be better extract off of here

use std::collections::HashMap;
use std::path::PathBuf;

use audio_processor_file::file_io::AudioFileError;
use audio_processor_file::InMemoryAudioFile;

type FileId = usize;

/// Handles input messages to read files, dispatches outputs messages when files have been read
pub struct AudioFileManager {
    audio_files: HashMap<FileId, InMemoryAudioFile>,
    current_id: FileId,
}

impl Default for AudioFileManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioFileManager {
    pub fn new() -> Self {
        Self {
            audio_files: Default::default(),
            current_id: 0,
        }
    }

    pub fn read_file(&mut self, file_path: PathBuf) -> Result<FileId, AudioFileError> {
        self.current_id += 1;
        let file_id = self.current_id;
        let file = InMemoryAudioFile::from_path(file_path.to_str().unwrap())?;
        self.audio_files.insert(file_id, file);
        Ok(file_id)
    }
}
