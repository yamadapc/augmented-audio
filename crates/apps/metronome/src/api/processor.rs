// = copyright ====================================================================
// Simple Metronome: macOS Metronome app
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
use audio_garbage_collector::Shared;
use audio_processor_metronome::{
    DefaultMetronomePlayhead, MetronomeProcessor, MetronomeProcessorHandle, MetronomeSound,
    MetronomeSoundType,
};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use std::collections::HashMap;
use std::hash::Hash;

struct LoadedSound {
    sound: MetronomeSoundType,
    file_path: String,
}

fn load_sounds() -> Option<Vec<LoadedSound>> {
    let sounds_path =
        macos_bundle_resources::get_path("com.beijaflor.metronome", "sounds", None, None)?;
    let sounds_path = sounds_path.to_str()?;
    let sounds_path = urlencoding::decode(sounds_path).ok()?.into_owned();
    let sounds_path = sounds_path.replace("file://", "");
    log::info!("Found sounds path: {:?}", sounds_path);
    let settings = audio_processor_traits::AudioProcessorSettings::default();

    let file_sounds: Vec<LoadedSound> = {
        let sound_file_paths = std::fs::read_dir(sounds_path).ok()?;
        log::info!("Found sounds: {:?}", sound_file_paths);

        let sounds = sound_file_paths
            .filter_map(|p| p.ok())
            .filter_map(|file_path| {
                let path = file_path.path();
                let file_path = path.to_str()?;
                let processor = audio_processor_file::AudioFileProcessor::from_path(
                    audio_garbage_collector::handle(),
                    settings,
                    file_path,
                )
                .ok()?;

                let file_name = path.file_name()?.to_str()?.to_string();
                Some((processor, file_name))
            })
            .map(|(processor, path)| {
                let sound = MetronomeSoundType::file(processor);
                LoadedSound {
                    sound,
                    file_path: path,
                }
            })
            .collect();

        Some(sounds)
    }
    .unwrap_or_default();

    log::info!("Read sounds {}", file_sounds.len());
    Some(file_sounds)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MetronomeSoundTypeTag {
    Sine,
    Tube,
    Glass,
    Snap,
}

pub enum AppAudioThreadMessage {
    SetMetronomeSound(MetronomeSoundTypeTag),
}

pub struct AppAudioProcessor {
    metronome: MetronomeProcessor<DefaultMetronomePlayhead>,
    sounds: HashMap<MetronomeSoundTypeTag, MetronomeSoundType>,
    messages: ringbuf::Consumer<AppAudioThreadMessage>,
    tag: MetronomeSoundTypeTag,
}

pub fn build_app_processor() -> (ringbuf::Producer<AppAudioThreadMessage>, AppAudioProcessor) {
    // Load sounds
    let sounds = load_sounds().unwrap_or_default();
    let mut sounds_map: HashMap<MetronomeSoundTypeTag, MetronomeSoundType> = HashMap::default();
    let mut known_sounds: HashMap<String, MetronomeSoundTypeTag> = [
        ("tube-click.wav", MetronomeSoundTypeTag::Tube),
        ("snap.wav", MetronomeSoundTypeTag::Snap),
        ("glass.wav", MetronomeSoundTypeTag::Glass),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();

    for loaded_sound in sounds {
        if let Some(tag) = known_sounds.remove(&loaded_sound.file_path) {
            sounds_map.insert(tag, loaded_sound.sound);
        }
    }

    // Set-up processor
    let (tx, rx) = ringbuf::RingBuffer::new(10).split();
    let app = AppAudioProcessor {
        metronome: {
            let processor = MetronomeProcessor::default();
            processor.handle().set_is_playing(false);
            processor
        },
        tag: MetronomeSoundTypeTag::Sine,
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
            let old_tag = self.tag;
            self.tag = sound;
            let old_sound = self.metronome.set_sound(new_sound);
            self.sounds.insert(old_tag, old_sound);
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
