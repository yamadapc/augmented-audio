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

use audio_processor_testing_helpers::relative_path;
use iced::{widget::Column, widget::Text, Command, Length};

use audio_processor_analysis::transient_detection::stft::{
    markers::build_markers, markers::AudioFileMarker, IterativeTransientDetectionParams,
};
use audio_processor_file::AudioFileProcessor;
use audio_processor_iced_design_system::{spacing::Spacing, style::Container1};
use audio_processor_iced_storybook::StoryView;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};

use crate::ui::common::parameter_view::{
    parameter_view_model::ParameterViewModel, KnobChanged, MultiParameterView,
};

use super::*;

#[derive(Clone, Debug)]
pub enum StoryMessage {
    Inner(Message),
    Knobs(KnobChanged<ParameterId>),
    ProcessMarkers(usize),
    SetMarkers(Vec<AudioFileMarker>),
}

#[allow(dead_code)]
pub fn default() -> Story {
    Story::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterId {
    FFTSize,
    ThresholdTimeSpreadFactor,
    IterationMagnitudeFactor,
    IterationCount,
    Gate,
}

fn build_parameters() -> MultiParameterView<ParameterId> {
    use ParameterId::*;
    let parameters = vec![
        ParameterViewModel::new(FFTSize, "FFT Size", "", 2048.0, (1024.0, 8192.0)).snap_int(),
        ParameterViewModel::new(
            IterationMagnitudeFactor,
            "Iteration magnitude factor",
            "",
            0.1,
            (0.0, 1.0),
        ),
        ParameterViewModel::new(
            ThresholdTimeSpreadFactor,
            "Threshold time spread factor",
            "",
            2.0,
            (0.0, 10.0),
        ),
        ParameterViewModel::new(Gate, "Gate", "", 0.4, (0.0, 1.0)),
        ParameterViewModel::new(IterationCount, "Iteration count", "", 20.0, (1.0, 40.0))
            .snap_int(),
    ];

    MultiParameterView::new(parameters)
}

pub struct Story {
    editor: AudioEditorView,
    parameters_view: MultiParameterView<ParameterId>,
    params: IterativeTransientDetectionParams,
    is_loading: bool,
    audio_file_buffer: Vec<f32>,
    cursor: usize,
}

impl Default for Story {
    fn default() -> Self {
        let mut editor = AudioEditorView::default();
        let settings = AudioProcessorSettings::default();
        log::info!("Reading audio file");
        let mut audio_file_buffer = get_example_file_buffer(settings);
        let markers = build_markers(&settings, &mut audio_file_buffer, Default::default(), 0.4);

        log::info!("Building editor model");
        editor.markers = markers;
        editor.audio_file_model = Some(AudioFileModel::from_buffer(
            settings,
            audio_file_buffer.clone(),
        ));

        let parameters_view = build_parameters();
        log::info!("Starting");
        Self {
            editor,
            parameters_view,
            params: Default::default(),
            audio_file_buffer,
            is_loading: false,
            cursor: 0,
        }
    }
}

fn get_example_file_buffer(settings: AudioProcessorSettings) -> Vec<f32> {
    let mut processor = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &relative_path!("../../../augmented/audio/audio-processor-analysis/hiphop-drum-loop.mp3"),
        // &relative_path!("../../../../input-files/synthetizer-loop.mp3"),
    )
    .unwrap();
    processor.prepare(settings);
    let channels = processor.buffer().clone();
    let mut output = vec![];
    for (s1, s2) in channels[0].iter().zip(channels[1].clone()) {
        output.push(s1 + s2);
    }
    output
}

impl StoryView<StoryMessage> for Story {
    fn update(&mut self, message: StoryMessage) -> Command<StoryMessage> {
        match message {
            StoryMessage::Inner(message) => {
                AudioEditorView::update(&mut self.editor, message);
                Command::none()
            }
            StoryMessage::Knobs(message) => {
                if let Some(state) = self.parameters_view.update(&message.id, message.value) {
                    match state.id {
                        ParameterId::FFTSize => {
                            self.params.fft_size = ((state.value / 1024.0).floor() * 1024.0)
                                .clamp(state.range.0, state.range.1)
                                as usize;
                            self.parameters_view
                                .update(&ParameterId::FFTSize, self.params.fft_size as f32);
                        }
                        ParameterId::IterationMagnitudeFactor => {
                            self.params.iteration_magnitude_factor = state.value;
                        }
                        ParameterId::IterationCount => {
                            self.params.iteration_count = state.value as usize;
                        }
                        ParameterId::ThresholdTimeSpreadFactor => {
                            self.params.threshold_time_spread_factor = state.value;
                        }
                        _ => {}
                    }

                    self.cursor += 1;
                    let cursor: usize = self.cursor;
                    Command::perform(tokio::time::sleep(Duration::from_secs(1)), move |_| {
                        StoryMessage::ProcessMarkers(cursor)
                    })
                } else {
                    Command::none()
                }
            }
            StoryMessage::ProcessMarkers(cursor) => {
                if self.cursor != cursor {
                    return Command::none();
                }

                let mut audio_file = self.audio_file_buffer.clone();
                self.is_loading = true;
                let params = self.params.clone();

                let gate_value = self.parameters_view.get(&ParameterId::Gate).unwrap().value;
                Command::perform(
                    tokio::task::spawn_blocking(move || {
                        build_markers(
                            &AudioProcessorSettings::default(),
                            &mut audio_file,
                            params,
                            gate_value,
                        )
                    }),
                    |result| StoryMessage::SetMarkers(result.unwrap()),
                )
            }
            StoryMessage::SetMarkers(markers) => {
                self.editor.markers = markers;
                self.is_loading = false;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<StoryMessage> {
        let view = self.editor.view();

        Column::with_children(vec![
            view.map(StoryMessage::Inner),
            Text::new(if self.is_loading {
                "Loading..."
            } else {
                "Ready"
            })
            .into(),
            Container::new(self.parameters_view.view().map(StoryMessage::Knobs))
                .padding(Spacing::base_spacing())
                .style(Container1::default())
                .width(Length::Fill)
                .into(),
        ])
        .into()
    }
}
