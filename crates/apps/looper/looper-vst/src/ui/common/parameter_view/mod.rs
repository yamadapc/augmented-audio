use std::collections::HashMap;
use std::fmt::Debug;

use iced::Element;
use iced::{Alignment, Column, Length, Text};
use iced_audio::{IntRange, Normal};

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

pub struct MultiParameterView<ParameterId> {
    states: HashMap<ParameterId, ParameterViewModel<ParameterId>>,
}

impl<ParameterId> MultiParameterView<ParameterId>
where
    ParameterId: std::hash::Hash + Clone + std::cmp::Eq + Copy + Debug + 'static,
{
    pub fn new(parameters: Vec<ParameterViewModel<ParameterId>>) -> Self {
        let mut states = HashMap::new();
        for parameter in parameters {
            states.insert(parameter.id.clone(), parameter);
        }

        Self { states }
    }

    pub fn get(&self, parameter_id: &ParameterId) -> Option<&ParameterViewModel<ParameterId>> {
        self.states.get(parameter_id)
    }

    pub fn get_mut(
        &mut self,
        parameter_id: &ParameterId,
    ) -> Option<&mut ParameterViewModel<ParameterId>> {
        self.states.get_mut(parameter_id)
    }

    pub fn update(
        &mut self,
        parameter_id: &ParameterId,
        value: f32,
    ) -> Option<&mut ParameterViewModel<ParameterId>> {
        if let Some(state) = self.get_mut(parameter_id) {
            state.value = value;
            if let Some(int_range) = &state.int_range {
                state.knob_state.snap_visible_to(int_range);
            }
            Some(state)
        } else {
            None
        }
    }

    pub fn view(&mut self) -> Element<KnobChanged<ParameterId>> {
        parameters_row(self.states.values_mut())
    }
}

pub fn parameters_row<'a, ParameterId: 'static + Clone + Copy + Debug>(
    parameter_states: impl IntoIterator<Item = &'a mut ParameterViewModel<ParameterId>>,
) -> Element<'a, KnobChanged<ParameterId>> {
    use iced::{Container, Row};

    let knobs = parameter_states
        .into_iter()
        .map(|parameter_view_model| view(parameter_view_model))
        .collect();

    Container::new(
        Row::with_children(knobs)
            .spacing(Spacing::base_spacing())
            .width(Length::Fill),
    )
    .center_x()
    .center_y()
    .width(Length::Fill)
    .into()
}

pub fn view<ParameterId: 'static + Clone + Copy + Debug>(
    parameter_view_model: &mut ParameterViewModel<ParameterId>,
) -> Element<KnobChanged<ParameterId>> {
    let range = parameter_view_model.range;
    let parameter_id = parameter_view_model.id.clone();
    let mapped_value = parameter_view_model.value;
    let int_range = parameter_view_model.int_range.clone();
    let label = &parameter_view_model.name;
    let value_label = format!("{:.2}{}", mapped_value, parameter_view_model.suffix);

    HoverContainer::new(
        Column::with_children(vec![
            Text::new(label).size(Spacing::small_font_size()).into(),
            Knob::new(&mut parameter_view_model.knob_state, move |value| {
                map_normal_to_message(parameter_id, range, int_range, value)
            })
            .size(Length::Units(Spacing::base_control_size()))
            .style(audio_knob::style::Knob)
            .into(),
            Text::new(value_label)
                .size(Spacing::small_font_size())
                .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(Spacing::small_spacing()),
    )
    .style(audio_style::HoverContainer)
    .into()
}

fn map_normal_to_message<ParameterId: 'static + Clone + Copy + Debug>(
    parameter_id: ParameterId,
    range: (f32, f32),
    int_range: Option<IntRange>,
    value: Normal,
) -> KnobChanged<ParameterId> {
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
}
