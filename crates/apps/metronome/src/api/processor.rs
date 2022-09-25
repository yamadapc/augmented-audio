use audio_garbage_collector::Shared;
use audio_processor_metronome::{
    DefaultMetronomePlayhead, MetronomeProcessor, MetronomeProcessorHandle, MetronomeSound,
    MetronomeSoundType,
};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use std::collections::HashMap;

fn load_sounds() -> Option<Vec<MetronomeSoundType>> {
    let sounds_path =
        macos_bundle_resources::get_path("com.beijaflor.metronome", "sounds", None, None)?;
    let sounds_path = sounds_path.to_str()?;
    let sounds_path = urlencoding::decode(sounds_path).ok()?.into_owned();
    let sounds_path = sounds_path.replace("file://", "");
    log::info!("Found sounds path: {:?}", sounds_path);
    let settings = audio_processor_traits::AudioProcessorSettings::default();

    let file_sounds: Vec<MetronomeSoundType> = {
        let sound_file_paths = std::fs::read_dir(sounds_path).ok()?;
        log::info!("Found sounds: {:?}", sound_file_paths);

        let sounds = sound_file_paths
            .filter_map(|p| p.ok())
            .filter_map(|file_path| {
                let path = file_path.path();
                let file_path = path.to_str()?;
                audio_processor_file::AudioFileProcessor::from_path(
                    audio_garbage_collector::handle(),
                    settings,
                    file_path,
                )
                .ok()
            })
            .map(MetronomeSoundType::file)
            .collect();

        Some(sounds)
    }
    .unwrap_or_default();

    log::info!("Read sounds {}", file_sounds.len());
    Some(file_sounds)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MetronomeSoundTypeTag {
    Sine,
    Tube,
}

pub enum AppAudioThreadMessage {
    SetMetronomeSound(MetronomeSoundTypeTag),
}

pub struct AppAudioProcessor {
    metronome: MetronomeProcessor<DefaultMetronomePlayhead>,
    sounds: HashMap<MetronomeSoundTypeTag, MetronomeSoundType>,
    messages: ringbuf::Consumer<AppAudioThreadMessage>,
}

pub fn build_app_processor() -> (ringbuf::Producer<AppAudioThreadMessage>, AppAudioProcessor) {
    // Load sounds
    let mut sounds = load_sounds().unwrap_or_default();
    let mut sounds_map = HashMap::default();
    if let Some(tube_sound) = sounds.pop() {
        sounds_map.insert(MetronomeSoundTypeTag::Tube, tube_sound);
    }

    // Set-up processor
    let (tx, rx) = ringbuf::RingBuffer::new(10).split();
    let app = AppAudioProcessor {
        metronome: {
            let processor = MetronomeProcessor::default();
            processor.handle().set_is_playing(false);
            processor
        },
        sounds: sounds_map,
        messages: rx,
    };
    (tx, app)
}

impl AppAudioProcessor {
    pub fn metronome_handle(&self) -> &Shared<MetronomeProcessorHandle> {
        self.metronome.handle()
    }

    fn drain_message_queue(&mut self) {
        while let Some(message) = self.messages.pop() {
            match message {
                AppAudioThreadMessage::SetMetronomeSound(sound) => {
                    self.set_metronome_sound(sound);
                }
            }
        }
    }

    fn set_metronome_sound(&mut self, sound: MetronomeSoundTypeTag) {
        if let Some(new_sound) = self.sounds.remove(&sound) {
            let old_sound = self.metronome.set_sound(new_sound);
            self.sounds.insert(
                match old_sound {
                    MetronomeSoundType::Sine(_) => MetronomeSoundTypeTag::Sine,
                    MetronomeSoundType::File(_) => MetronomeSoundTypeTag::Tube,
                },
                old_sound,
            );
        }
    }
}

impl AudioProcessor for AppAudioProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.metronome.prepare(settings);
        for sound in self.sounds.values_mut() {
            sound.prepare(settings);
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.drain_message_queue();

        self.metronome.process(data);
    }
}
