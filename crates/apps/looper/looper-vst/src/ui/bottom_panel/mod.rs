use iced::{Alignment, Button, Column, Container, Length, Row, Text};
use iced_baseview::{Command, Element};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use looper_processor::LoopSequencerParams;
use looper_processor::{LoopSequencerProcessorHandle, LooperProcessorHandle};
use parameter_view::parameter_view_model::{ParameterId, ParameterViewModel};

mod parameter_view;

#[derive(Debug, Clone)]
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
    sequencer_handle: Shared<LoopSequencerProcessorHandle>,
    parameter_states: Vec<ParameterViewModel>,
    buttons_view: ButtonsView,
    sequence_button_state: iced::button::State,
}

impl BottomPanelView {
    pub fn new(
        processor_handle: Shared<LooperProcessorHandle>,
        sequencer_handle: Shared<LoopSequencerProcessorHandle>,
    ) -> Self {
        BottomPanelView {
            processor_handle: processor_handle.clone(),
            sequencer_handle: sequencer_handle.clone(),
            parameter_states: vec![
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
                // ParameterViewModel::new(
                //     ParameterId::PlaybackSpeed,
                //     String::from("Speed"),
                //     String::from("x"),
                //     1.0,
                //     (0.0, 2.0),
                // ),
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
            ],
            buttons_view: ButtonsView::new(processor_handle),
            sequence_button_state: iced::button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KnobChange(id, value) => {
                if let Some(state) = self
                    .parameter_states
                    .iter_mut()
                    .find(|param| param.id == id)
                {
                    state.value = value;

                    if let Some(int_range) = &state.int_range {
                        state.knob_state.snap_visible_to(int_range);
                    }

                    match state.id {
                        ParameterId::LoopVolume => {
                            self.processor_handle.set_wet_volume(state.value);
                        }
                        ParameterId::DryVolume => {
                            self.processor_handle.set_dry_volume(state.value);
                        }
                        // ParameterId::PlaybackSpeed => {
                        //     self.processor_handle.set_playback_speed(state.value);
                        // }
                        ParameterId::SeqSlices => {
                            if let Some(params) = self.sequencer_handle.params() {
                                if params.num_slices != state.value as usize {
                                    self.flush_params();
                                }
                            } else {
                                self.flush_params();
                            }
                        }
                        ParameterId::SeqSteps => {
                            if let Some(params) = self.sequencer_handle.params() {
                                if params.sequence_length != state.value as usize {
                                    self.flush_params();
                                }
                            } else {
                                self.flush_params();
                            }
                        }
                    }
                }
            }
            Message::RecordPressed => {
                self.processor_handle.toggle_recording();
            }
            Message::ClearPressed => {
                self.processor_handle.clear();
                self.sequencer_handle.clear();
            }
            Message::StopPressed => {
                self.processor_handle.toggle_playback();
            }
            Message::SequencePressed => self.flush_params(),
            Message::QuantizeModePressed => {
                self.processor_handle.set_tempo(120);
                let quantize_options = self.processor_handle.quantize_options();
                quantize_options.set_mode(quantize_options.mode().cycle());
            }
        }
        Command::none()
    }

    fn flush_params(&self) {
        self.sequencer_handle.set_params(LoopSequencerParams {
            num_slices: self
                .parameter_states
                .iter()
                .find(|param| param.id == ParameterId::SeqSlices)
                .unwrap()
                .value as usize,
            sequence_length: self
                .parameter_states
                .iter()
                .find(|param| param.id == ParameterId::SeqSteps)
                .unwrap()
                .value as usize,
            num_samples: self.processor_handle.num_samples(),
        });
    }

    pub fn view(&mut self) -> Element<Message> {
        let knobs = self
            .parameter_states
            .iter_mut()
            .map(|parameter_view_model| {
                parameter_view::view(parameter_view_model)
                    .map(|msg| Message::KnobChange(msg.id, msg.value))
            })
            .collect();
        Container::new(Column::with_children(vec![Container::new(
            Row::with_children(vec![
                Container::new(self.buttons_view.view()).center_y().into(),
                Container::new(
                    Row::with_children(knobs)
                        .spacing(Spacing::base_spacing())
                        .width(Length::Fill),
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .into(),
                Container::new(
                    Button::new(
                        &mut self.sequence_button_state,
                        Text::new("Sequence").size(Spacing::small_font_size()),
                    )
                    .on_press(Message::SequencePressed)
                    .style(audio_style::Button::default()),
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
    record_state: iced::button::State,
    clear_state: iced::button::State,
    stop_state: iced::button::State,
    quantize_mode_state: iced::button::State,
}

impl ButtonsView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle>) -> Self {
        ButtonsView {
            processor_handle,
            record_state: iced::button::State::default(),
            clear_state: iced::button::State::default(),
            stop_state: iced::button::State::default(),
            quantize_mode_state: iced::button::State::default(),
        }
    }
}

impl ButtonsView {
    pub fn view(&mut self) -> Element<Message> {
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
            Button::new(
                &mut self.record_state,
                Text::new(record_text).size(Spacing::small_font_size()),
            )
            .on_press(Message::RecordPressed)
            .style(audio_style::Button::default())
            .into(),
            Button::new(
                &mut self.clear_state,
                Text::new("Clear").size(Spacing::small_font_size()),
            )
            .on_press(Message::ClearPressed)
            .style(audio_style::Button::default())
            .into(),
            Button::new(
                &mut self.stop_state,
                Text::new(stop_text).size(Spacing::small_font_size()),
            )
            .on_press(Message::StopPressed)
            .style(audio_style::Button::default())
            .into(),
            Button::new(
                &mut self.quantize_mode_state,
                Text::new(quantize_mode_label(
                    self.processor_handle.quantize_options().mode(),
                ))
                .size(Spacing::small_font_size()),
            )
            .on_press(Message::QuantizeModePressed)
            .style(audio_style::Button::default())
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
