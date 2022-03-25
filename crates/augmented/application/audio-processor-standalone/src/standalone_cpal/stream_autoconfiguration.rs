use audio_processor_traits::AudioProcessorSettings;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{
    BufferSize, StreamConfig, SupportedBufferSize, SupportedInputConfigs, SupportedOutputConfigs,
    SupportedStreamConfigRange,
};
use itertools::Itertools;

enum BlockSize {
    Fixed(usize),
    Variable,
}

pub struct AudioConfiguration {
    settings: AudioProcessorSettings,
    input: Option<StreamConfig>,
    output: Option<StreamConfig>,
}

#[derive(thiserror::Error, Debug)]
enum AudioConfigurationError {
    #[error("Failed to get supported device configuration")]
    SupportedStreamConfigsError {
        cause: cpal::SupportedStreamConfigsError,
        device_name: String,
        is_input: bool,
    },
}

#[derive(Debug, PartialEq)]
struct SupportedConfigs {
    sample_rate: (f32, f32),
    channel_count: (usize, usize),
    buffer_size: SupportedBufferSize,
}

#[derive(Default)]
pub struct AudioConfigurationManager {}

impl AudioConfigurationManager {
    pub fn negotiate_default_configuration(
        &self,
        preferred_settings: AudioProcessorSettings,
    ) -> Result<AudioConfiguration, AudioConfigurationError> {
        let host = cpal::default_host();
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();
        log::info!(
            "Found input/output devices\n  host={:?}\n  input={:?}\n  output={:?}",
            host.id(),
            input_device.as_ref().map(|d| d.name().ok()).flatten(),
            output_device.as_ref().map(|d| d.name().ok()).flatten()
        );

        let input_configs = if let Some(device) = &input_device {
            Some(device.supported_input_configs().map_err(|err| {
                AudioConfigurationError::SupportedStreamConfigsError {
                    cause: err,
                    device_name: device.name().ok().unwrap_or(String::from("<unknown>")),
                    is_input: true,
                }
            })?)
        } else {
            None
        };

        let output_configs = if let Some(device) = &output_device {
            Some(device.supported_output_configs().map_err(|err| {
                AudioConfigurationError::SupportedStreamConfigsError {
                    cause: err,
                    device_name: device.name().ok().unwrap_or(String::from("<unknown>")),
                    is_input: true,
                }
            })?)
        } else {
            None
        };

        let input_configs = input_configs
            .map(|input_configs| {
                input_configs
                    .map(|config| SupportedConfigs {
                        sample_rate: (
                            config.min_sample_rate().0 as f32,
                            config.max_sample_rate().0 as f32,
                        ),
                        channel_count: (config.channels().into(), config.channels().into()),
                        buffer_size: config.buffer_size().clone(),
                    })
                    .collect()
            })
            .unwrap_or(vec![]);
        let output_configs = output_configs
            .map(|output_configs| {
                output_configs
                    .map(|config| SupportedConfigs {
                        sample_rate: (
                            config.min_sample_rate().0 as f32,
                            config.max_sample_rate().0 as f32,
                        ),
                        channel_count: (config.channels().into(), config.channels().into()),
                        buffer_size: config.buffer_size().clone(),
                    })
                    .collect()
            })
            .unwrap_or(vec![]);
        let settings = self.build_settings(preferred_settings, &input_configs, &output_configs)?;

        Ok(AudioConfiguration {
            settings,
            input: None,
            output: None,
        })
    }

    fn best_config<'a>(
        &self,
        suggested_channels: usize,
        suggested_sample_rate: f32,
        suggested_block_size: usize,
        input_configs: &'a [SupportedConfigs],
    ) -> Option<&'a SupportedConfigs> {
        input_configs
            .iter()
            .sorted_by(|config1, config2| {
                let mut value1 = 0;
                let mut value2 = 0;

                if matches_channels(suggested_channels, config1) {
                    value1 += 1
                }
                if matches_sample_rate(suggested_sample_rate, config1) {
                    value1 += 1;
                }
                if matches_block_size(suggested_block_size, config1) {
                    value1 += 1;
                }

                if matches_channels(suggested_channels, config2) {
                    value2 += 1
                }
                if matches_sample_rate(suggested_sample_rate, config2) {
                    value2 += 1;
                }
                if matches_block_size(suggested_block_size, config2) {
                    value2 += 1;
                }

                if value1 > value2 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .find(|_| true)
    }

    fn build_settings(
        &self,
        mut suggested_config: AudioProcessorSettings,
        input_configs: &[SupportedConfigs],
        output_configs: &[SupportedConfigs],
    ) -> Result<AudioProcessorSettings, AudioConfigurationError> {
        let best_input = self.best_config(
            suggested_config.input_channels(),
            suggested_config.sample_rate(),
            suggested_config.block_size(),
            input_configs,
        );
        let best_output = self.best_config(
            suggested_config.output_channels(),
            suggested_config.sample_rate(),
            suggested_config.block_size(),
            output_configs,
        );
        suggested_config.input_channels = best_input
            .map(|config| config.channel_count.0.max(suggested_config.input_channels))
            .unwrap_or(0);
        suggested_config.output_channels = best_output
            .map(|config| config.channel_count.0.max(suggested_config.input_channels))
            .unwrap_or(0);
        suggested_config.sample_rate = best_output
            .map(|config| config.sample_rate.0.max(suggested_config.sample_rate))
            .unwrap_or(suggested_config.sample_rate);
        suggested_config.block_size = best_output
            .map(|config| {
                if let SupportedBufferSize::Range { min, .. } = config.buffer_size {
                    (min as usize).max(suggested_config.block_size)
                } else {
                    suggested_config.block_size
                }
            })
            .unwrap_or(0);
        Ok(suggested_config)
    }
}

fn matches_block_size(suggested_block_size: usize, config: &SupportedConfigs) -> bool {
    match config.buffer_size {
        SupportedBufferSize::Range { min, max } => {
            min as usize <= suggested_block_size && max as usize >= suggested_block_size
        }
        SupportedBufferSize::Unknown => false,
    }
}

fn matches_sample_rate(suggested_sample_rate: f32, config: &SupportedConfigs) -> bool {
    config.sample_rate.0 as f32 <= suggested_sample_rate
        && config.sample_rate.1 as f32 >= suggested_sample_rate
}

fn matches_channels(suggested_channels: usize, config: &SupportedConfigs) -> bool {
    config.channel_count.0 as usize <= suggested_channels
        && config.channel_count.1 as usize >= suggested_channels
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pick_best_config_picks_a_matching_configuration() {
        let _ = wisual_logger::try_init_from_env();
        let settings = AudioProcessorSettings {
            sample_rate: 100.0,
            input_channels: 2,
            output_channels: 2,
            block_size: 512,
        };
        let input_configs = vec![SupportedConfigs {
            sample_rate: (100.0, 100.0),
            channel_count: (2, 2),
            buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
        }];

        let suggested_channels = settings.input_channels();
        let suggested_sample_rate = settings.sample_rate();
        let suggested_block_size = settings.block_size();
        let result = AudioConfigurationManager::default()
            .best_config(
                suggested_channels,
                suggested_sample_rate,
                suggested_block_size,
                &input_configs,
            )
            .unwrap();
        assert_eq!(
            result,
            &SupportedConfigs {
                sample_rate: (100.0, 100.0),
                channel_count: (2, 2),
                buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
            }
        );
    }

    #[test]
    fn test_pick_best_config_picks_the_best_matching_configuration() {
        let _ = wisual_logger::try_init_from_env();
        let settings = AudioProcessorSettings {
            sample_rate: 100.0,
            input_channels: 2,
            output_channels: 2,
            block_size: 512,
        };
        let input_configs = vec![
            SupportedConfigs {
                sample_rate: (200.0, 200.0),
                channel_count: (2, 2),
                buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
            },
            SupportedConfigs {
                sample_rate: (100.0, 100.0),
                channel_count: (4, 4),
                buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
            },
            SupportedConfigs {
                sample_rate: (100.0, 100.0),
                channel_count: (2, 2),
                buffer_size: SupportedBufferSize::Range { min: 100, max: 100 },
            },
            SupportedConfigs {
                sample_rate: (100.0, 100.0),
                channel_count: (2, 2),
                buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
            },
        ];

        let suggested_channels = settings.input_channels();
        let suggested_sample_rate = settings.sample_rate();
        let suggested_block_size = settings.block_size();
        let result = AudioConfigurationManager::default()
            .best_config(
                suggested_channels,
                suggested_sample_rate,
                suggested_block_size,
                &input_configs,
            )
            .unwrap();
        assert_eq!(
            result,
            &SupportedConfigs {
                sample_rate: (100.0, 100.0),
                channel_count: (2, 2),
                buffer_size: SupportedBufferSize::Range { min: 512, max: 512 },
            }
        );
    }

    #[test]
    fn test_negotiate_default_configuration() {
        let _ = wisual_logger::try_init_from_env();
        let manager = AudioConfigurationManager::default();
        let settings = AudioProcessorSettings::default();
        let result = manager.negotiate_default_configuration(settings);
    }
}
