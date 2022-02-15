use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_traits::parameters::AudioProcessorHandleRef;

mod generic_gui {
    use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
    use iced::{Alignment, Column, Command, Container, Length, Row, Text};
    use iced_audio::{Normal, NormalParam};
    use iced_baseview::{Element, IcedWindow, Settings};

    use audio_processor_iced_design_system::knob as audio_knob;
    use audio_processor_iced_design_system::knob::Knob;
    use audio_processor_iced_design_system::spacing::Spacing;
    use audio_processor_iced_design_system::style::Container1;
    use audio_processor_traits::parameters::{
        AudioProcessorHandleRef, ParameterSpec, ParameterType, ParameterValue,
    };

    struct Flags {
        handle: AudioProcessorHandleRef,
    }

    struct ParameterModel {
        spec: ParameterSpec,
        knob_state: iced_audio::knob::State,
    }

    struct GenericAudioProcessorApplication {
        handle: AudioProcessorHandleRef,
        parameter_models: Vec<ParameterModel>,
    }

    #[derive(Debug, Clone)]
    enum Message {
        KnobChange(usize, f32),
    }

    impl iced_baseview::Application for GenericAudioProcessorApplication {
        type Executor = iced::executor::Default;
        type Message = Message;
        type Flags = Flags;

        fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
            let mut parameter_models = vec![];
            for i in 0..flags.handle.parameter_count() {
                let spec = flags.handle.get_parameter_spec(i);
                let value = flags.handle.get_parameter(i).unwrap();
                let value = match (value, spec.ty()) {
                    (ParameterValue::Float { value }, ParameterType::Float { range, .. }) => {
                        (value - range.0) / (range.1 - range.0)
                    }
                };
                parameter_models.push(ParameterModel {
                    spec,
                    knob_state: iced_audio::knob::State::new(NormalParam::new(Normal::from(value))),
                });
            }

            (
                Self {
                    handle: flags.handle,
                    parameter_models,
                },
                Command::none(),
            )
        }

        fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
            match message {
                Message::KnobChange(index, value) => self
                    .handle
                    .set_parameter(index, ParameterValue::Float { value }),
            }
            Command::none()
        }

        fn view(&mut self) -> Element<'_, Self::Message> {
            Container::new(Row::with_children(
                self.parameter_models
                    .iter_mut()
                    .enumerate()
                    .map(|(parameter_index, model)| {
                        let value = self.handle.get_parameter(parameter_index).unwrap();

                        match (value, model.spec.ty()) {
                            (
                                ParameterValue::Float { value },
                                ParameterType::Float { range, .. },
                            ) => {
                                let range = *range;

                                Column::with_children(vec![
                                    Text::new(model.spec.name())
                                        .size(Spacing::small_font_size())
                                        .into(),
                                    Knob::new(&mut model.knob_state, move |value| {
                                        let n_value =
                                            range.0 + value.as_f32() * (range.1 - range.0);
                                        Message::KnobChange(parameter_index, n_value)
                                    })
                                    .size(Length::Units(Spacing::base_control_size()))
                                    .style(audio_knob::style::Knob)
                                    .into(),
                                    Text::new(format!("{:.2}", value))
                                        .size(Spacing::small_font_size())
                                        .into(),
                                ])
                                .align_items(Alignment::Center)
                                .spacing(Spacing::small_spacing())
                                .into()
                            }
                        }
                    })
                    .collect(),
            ))
            .width(Length::Fill)
            .padding(Spacing::base_spacing())
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(Container1::default())
            .into()
        }
    }

    pub fn open(handle: AudioProcessorHandleRef) {
        IcedWindow::<GenericAudioProcessorApplication>::open_blocking(Settings {
            window: WindowOpenOptions {
                title: "bitcrusher".to_string(),
                size: Size {
                    width: 300.0,
                    height: 300.0,
                },
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            flags: Flags { handle },
        });
    }
}

fn main() {
    wisual_logger::init_from_env();

    let processor = BitCrusherProcessor::default();
    processor.handle().set_bit_rate(44100.0 / 4.0);

    let handle: AudioProcessorHandleRef = Box::new(processor.generic_handle());
    let _audio_handles = audio_processor_standalone::audio_processor_start(processor);

    generic_gui::open(handle);
}
