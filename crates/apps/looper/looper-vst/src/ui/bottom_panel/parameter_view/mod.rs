use std::fmt::Debug;

use iced::Element;
use iced::{Alignment, Column, Length, Text};

use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use parameter_view_model::ParameterViewModel;

pub mod parameter_view_model;

pub struct KnobChanged<ParameterId> {
    pub id: ParameterId,
    pub value: f32,
}

pub fn view<ParameterId: 'static + Clone + Copy + Debug>(
    parameter_view_model: &mut ParameterViewModel<ParameterId>,
) -> Element<KnobChanged<ParameterId>> {
    let range = parameter_view_model.range;
    let parameter_id = parameter_view_model.id.clone();
    let mapped_value = parameter_view_model.value;
    let int_range = parameter_view_model.int_range.clone();

    HoverContainer::new(
        Column::with_children(vec![
            Text::new(&parameter_view_model.name)
                .size(Spacing::small_font_size())
                .into(),
            Knob::new(&mut parameter_view_model.knob_state, move |value| {
                let value = if let Some(int_range) = int_range {
                    int_range.snapped(value)
                } else {
                    value
                };
                let n_value = range.0 + value.as_f32() * (range.1 - range.0);

                log::debug!(
                    "id={:?} range={:?} value={} nvalue={}",
                    parameter_id,
                    range,
                    value.as_f32(),
                    n_value
                );

                KnobChanged {
                    id: parameter_id,
                    value: n_value,
                }
            })
            .size(Length::Units(Spacing::base_control_size()))
            .style(audio_knob::style::Knob)
            .into(),
            Text::new(format!(
                "{:.2}{}",
                mapped_value, parameter_view_model.suffix
            ))
            .size(Spacing::small_font_size())
            .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(Spacing::small_spacing()),
    )
    .style(audio_style::HoverContainer)
    .into()
}
