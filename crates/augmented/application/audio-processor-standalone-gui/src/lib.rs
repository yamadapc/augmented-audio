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

//! Provides generic [`iced`] GUI for implementations of [`audio_processor_traits::parameters::AudioProcessorHandle`]
//!
//! GUI can be ran as a standalone app or as a `vst::editor::Editor`
//!
//! * [`open`] runs the app standalone
//! * [`editor`] runs a boxed `Editor` instance

use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use iced::Theme;
use iced_audio::{Normal, NormalParam};
use iced_baseview::{
    widget::Column, widget::Container, widget::Row, widget::Text, Alignment, Application, Command,
    Element, Length, Settings,
};

use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::Container1;
use audio_processor_traits::parameters::{
    AudioProcessorHandleRef, FloatType, ParameterSpec, ParameterType, ParameterValue,
};

#[derive(Clone)]
struct Flags {
    handle: AudioProcessorHandleRef,
}

struct ParameterModel {
    spec: ParameterSpec,
    knob_state: iced_audio::NormalParam,
}

struct GenericAudioProcessorApplication {
    handle: AudioProcessorHandleRef,
    parameter_models: Vec<ParameterModel>,
}

#[derive(Debug, Clone)]
enum Message {
    KnobChange(usize, f32),
}

impl Application for GenericAudioProcessorApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut parameter_models = vec![];
        for i in 0..flags.handle.parameter_count() {
            let spec = flags.handle.get_parameter_spec(i);
            let value = flags.handle.get_parameter(i).unwrap();
            let value = match (value, spec.ty()) {
                (
                    ParameterValue::Float { value },
                    ParameterType::Float(FloatType { range, .. }),
                ) => (value - range.0) / (range.1 - range.0),
            };
            parameter_models.push(ParameterModel {
                spec,
                knob_state: NormalParam {
                    value: Normal::from_clipped(value),
                    default: Normal::from_clipped(value),
                },
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

    fn title(&self) -> String {
        "Audio processor standalone".to_string()
    }

    fn update(
        &mut self,
        _window_queue: &mut iced_baseview::window::WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::KnobChange(index, value) => self
                .handle
                .set_parameter(index, ParameterValue::Float { value }),
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme> {
        let handle = &self.handle;

        Container::new(Row::with_children(
            self.parameter_models
                .iter()
                .enumerate()
                .map(|(parameter_index, model)| {
                    let value = handle.get_parameter(parameter_index).unwrap();
                    parameter_view(parameter_index, model, value)
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

fn parameter_view(
    parameter_index: usize,
    model: &ParameterModel,
    value: ParameterValue,
) -> Element<Message, Theme> {
    match (value, model.spec.ty()) {
        (ParameterValue::Float { value }, ParameterType::Float(FloatType { range, .. })) => {
            let range = *range;

            Column::with_children(vec![
                Text::new(model.spec.name())
                    .size(Spacing::small_font_size())
                    .into(),
                Knob::new(model.knob_state, move |value| {
                    let n_value = range.0 + value.as_f32() * (range.1 - range.0);
                    Message::KnobChange(parameter_index, n_value)
                })
                .size(Length::Fixed(Spacing::base_control_size() as f32))
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
}

/// Create a VST editor for this handle
pub fn editor(handle: AudioProcessorHandleRef) -> Box<dyn vst::editor::Editor> {
    let editor =
        augmented_iced_editor::IcedEditor::<GenericAudioProcessorApplication>::new(Flags {
            handle,
        });
    Box::new(editor)
}

/// Open this generic editor as a standalone app. Will block, needs to be run on the main-thread.
pub fn open(handle: AudioProcessorHandleRef) {
    iced_baseview::open_blocking::<GenericAudioProcessorApplication>(Settings {
        window: WindowOpenOptions {
            title: "audio-processor-standalone".to_string(),
            size: Size {
                width: 300.0,
                height: 300.0,
            },
            scale: WindowScalePolicy::SystemScaleFactor,
            #[cfg(feature = "glow")]
            gl_config: None,
        },
        iced_baseview: iced_baseview::settings::IcedBaseviewSettings {
            ignore_non_modifier_keys: false,
            always_redraw: true,
        },
        flags: Flags { handle },
    });
}
