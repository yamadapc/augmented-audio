use iced::{Button, Column, Container, Length, Row, Text};
use iced_baseview::{Command, Element};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use looper_processor::LoopSequencerParams;
use looper_processor::{LoopSequencerProcessorHandle, LooperProcessorHandle};

use crate::ui::common::parameter_view;
use crate::ui::common::parameter_view::parameter_view_model::ParameterViewModel;
use crate::ui::common::parameter_view::MultiParameterView;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum ParameterId {
    LoopVolume,
    DryVolume,
    SeqSlices,
    SeqSteps,
}

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
    parameters_view: MultiParameterView<ParameterId>,
    buttons_view: ButtonsView,
    sequence_button_state: iced::button::State,
}

impl BottomPanelView {
    pub fn new(
        processor_handle: Shared<LooperProcessorHandle>,
        sequencer_handle: Shared<LoopSequencerProcessorHandle>,
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
            sequencer_handle: sequencer_handle.clone(),
            parameters_view,
            buttons_view: ButtonsView::new(processor_handle),
            sequence_button_state: iced::button::State::new(),
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
                self.processor_handle.toggle_recording();
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
                self.processor_handle.set_tempo(120);
                let quantize_options = self.processor_handle.quantize_options();
                quantize_options.set_mode(quantize_options.mode().cycle());
            }
        }
        Command::none()
    }

    fn on_loop_sequencer_params_changed(&self) {
        let seq_slices = self.parameters_view.get(&ParameterId::SeqSlices).unwrap();
        let seq_steps = self.parameters_view.get(&ParameterId::SeqSteps).unwrap();

        self.sequencer_handle.set_params(LoopSequencerParams {
            num_slices: seq_slices.value as usize,
            sequence_length: seq_steps.value as usize,
            num_samples: self.processor_handle.num_samples(),
        });
    }

    pub fn view(&mut self) -> Element<Message> {
        Container::new(Column::with_children(vec![Container::new(
            Row::with_children(vec![
                Container::new(self.buttons_view.view()).center_y().into(),
                self.parameters_view
                    .view()
                    .map(|msg| Message::KnobChange(msg.id, msg.value)),
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
