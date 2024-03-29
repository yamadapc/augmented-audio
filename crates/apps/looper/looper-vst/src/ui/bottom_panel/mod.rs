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
use iced::{widget::Button, widget::Column, widget::Container, widget::Row, widget::Text, Length};
use iced_baseview::{Command, Element};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use looper_processor::{LoopShufflerParams, LooperHandleThread};
use looper_processor::{LoopShufflerProcessorHandle, LooperProcessorHandle};

use crate::ui::common::parameter_view::parameter_view_model::ParameterViewModel;
use crate::ui::common::parameter_view::MultiParameterView;

// TODO - this shouldn't be on this file
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum ParameterId {
    LoopVolume,
    DryVolume,
    SeqSlices,
    SeqSteps,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    KnobChange(ParameterId, f32),
    RecordPressed,
    ClearPressed,
    StopPressed,
    SequencePressed,
    QuantizeModePressed,
}

pub struct BottomPanelView {
    processor_handle: Shared<LooperProcessorHandle>,
    sequencer_handle: Shared<LoopShufflerProcessorHandle>,
    parameters_view: MultiParameterView<ParameterId>,
    buttons_view: ButtonsView,
}

impl BottomPanelView {
    pub fn new(
        processor_handle: Shared<LooperProcessorHandle>,
        sequencer_handle: Shared<LoopShufflerProcessorHandle>,
    ) -> Self {
        let parameters = vec![
            ParameterViewModel::new(
                ParameterId::LoopVolume,
                String::from("Loop"),
                String::from(""),
                processor_handle.wet_volume(),
                (0.0, 1.0),
            ),
            ParameterViewModel::new(
                ParameterId::DryVolume,
                String::from("Dry"),
                String::from(""),
                processor_handle.dry_volume(),
                (0.0, 1.0),
            ),
            ParameterViewModel::new(
                ParameterId::SeqSlices,
                String::from("Seq. Slices"),
                String::from(""),
                4.0,
                (1.0, 32.0),
            )
            .snap_int(),
            ParameterViewModel::new(
                ParameterId::SeqSteps,
                String::from("Seq. Steps"),
                String::from(""),
                4.0,
                (1.0, 32.0),
            )
            .snap_int(),
        ];
        let parameters_view = MultiParameterView::new(parameters);

        BottomPanelView {
            processor_handle: processor_handle.clone(),
            sequencer_handle,
            parameters_view,
            buttons_view: ButtonsView::new(processor_handle),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KnobChange(id, value) => {
                if let Some(state) = self.parameters_view.update(&id, value) {
                    match state.id {
                        ParameterId::LoopVolume => {
                            self.processor_handle.set_wet_volume(state.value);
                        }
                        ParameterId::DryVolume => {
                            self.processor_handle.set_dry_volume(state.value);
                        }
                        ParameterId::SeqSlices => {
                            if let Some(params) = self.sequencer_handle.params() {
                                if params.num_slices != state.value as usize {
                                    self.on_loop_sequencer_params_changed();
                                }
                            } else {
                                self.on_loop_sequencer_params_changed();
                            }
                        }
                        ParameterId::SeqSteps => {
                            if let Some(params) = self.sequencer_handle.params() {
                                if params.sequence_length != state.value as usize {
                                    self.on_loop_sequencer_params_changed();
                                }
                            } else {
                                self.on_loop_sequencer_params_changed();
                            }
                        }
                    }
                }
            }
            Message::RecordPressed => {
                self.processor_handle
                    .toggle_recording(LooperHandleThread::OtherThread);
            }
            Message::ClearPressed => {
                self.processor_handle.clear();
                self.sequencer_handle.clear();
            }
            Message::StopPressed => {
                self.processor_handle.toggle_playback();
            }
            Message::SequencePressed => self.on_loop_sequencer_params_changed(),
            Message::QuantizeModePressed => {
                self.processor_handle.set_tempo(120.0);
                let quantize_options = self.processor_handle.quantize_options();
                quantize_options.set_mode(quantize_options.mode().cycle());
            }
        }
        Command::none()
    }

    fn on_loop_sequencer_params_changed(&self) {
        let seq_slices = self.parameters_view.get(&ParameterId::SeqSlices).unwrap();
        let seq_steps = self.parameters_view.get(&ParameterId::SeqSteps).unwrap();

        self.sequencer_handle.set_params(LoopShufflerParams {
            num_slices: seq_slices.value as usize,
            sequence_length: seq_steps.value as usize,
            num_samples: self.processor_handle.num_samples(),
        });
    }

    pub fn view(&self) -> Element<Message, iced::Theme> {
        Container::new(Column::with_children(vec![Container::new(
            Row::with_children(vec![
                Container::new(self.buttons_view.view()).center_y().into(),
                self.parameters_view
                    .view()
                    .map(|msg| Message::KnobChange(msg.id, msg.value)),
                Container::new(
                    Button::new(Text::new("Sequence").size(Spacing::small_font_size()))
                        .on_press(Message::SequencePressed)
                        .style(audio_style::Button::default().into()),
                )
                .center_x()
                .center_y()
                .into(),
            ])
            .spacing(Spacing::base_spacing()),
        )
        .center_y()
        .width(Length::Fill)
        .padding(Spacing::base_spacing())
        .into()]))
        .center_y()
        .style(Container1::default())
        .into()
    }
}

struct ButtonsView {
    processor_handle: Shared<LooperProcessorHandle>,
}

impl ButtonsView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle>) -> Self {
        ButtonsView { processor_handle }
    }
}

impl ButtonsView {
    pub fn view(&self) -> Element<Message, iced::Theme> {
        let record_text = if self.processor_handle.is_recording() {
            "Stop recording"
        } else {
            "Record"
        };
        let stop_text = if self.processor_handle.is_playing_back() {
            "Stop"
        } else {
            "Play"
        };

        Row::with_children(vec![
            Button::new(Text::new(record_text).size(Spacing::small_font_size()))
                .on_press(Message::RecordPressed)
                .style(audio_style::Button::default().into())
                .into(),
            Button::new(Text::new("Clear").size(Spacing::small_font_size()))
                .on_press(Message::ClearPressed)
                .style(audio_style::Button::default().into())
                .into(),
            Button::new(Text::new(stop_text).size(Spacing::small_font_size()))
                .on_press(Message::StopPressed)
                .style(audio_style::Button::default().into())
                .into(),
            Button::new(
                Text::new(quantize_mode_label(
                    self.processor_handle.quantize_options().mode(),
                ))
                .size(Spacing::small_font_size()),
            )
            .on_press(Message::QuantizeModePressed)
            .style(audio_style::Button::default().into())
            .into(),
        ])
        .into()
    }
}

fn quantize_mode_label(mode: looper_processor::QuantizeMode) -> String {
    use looper_processor::QuantizeMode::*;

    match mode {
        None => "Free tempo",
        SnapNext => "Snap next",
        SnapClosest => "Snap closest",
    }
    .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use audio_garbage_collector::make_shared;

    #[test]
    fn test_buttons_view() {
        let processor_handle = make_shared(LooperProcessorHandle::default());
        let buttons_view = ButtonsView::new(processor_handle);
        let _view = buttons_view.view();
    }

    #[test]
    fn test_quantize_mode_label_none() {
        assert_eq!(
            quantize_mode_label(looper_processor::QuantizeMode::None),
            "Free tempo"
        );
    }

    #[test]
    fn test_quantize_mode_label_snap_next() {
        assert_eq!(
            quantize_mode_label(looper_processor::QuantizeMode::SnapNext),
            "Snap next"
        );
    }

    #[test]
    fn test_quantize_mode_label_snap_closest() {
        assert_eq!(
            quantize_mode_label(looper_processor::QuantizeMode::SnapClosest),
            "Snap closest"
        );
    }
}
