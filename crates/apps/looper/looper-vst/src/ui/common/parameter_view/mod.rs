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
use std::collections::HashMap;
use std::fmt::Debug;

use iced::Element;
use iced::{Alignment, Column, Length, Text};
use iced_audio::{IntRange, Normal};
use itertools::Itertools;

use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use parameter_view_model::ParameterViewModel;

pub mod parameter_view_model;

#[derive(Clone, Debug)]
pub struct KnobChanged<ParameterId> {
    pub id: ParameterId,
    pub value: f32,
}

pub struct MultiParameterView<ParameterId> {
    states: HashMap<ParameterId, ParameterViewModel<ParameterId>>,
    order: Vec<ParameterId>,
}

impl<ParameterId> MultiParameterView<ParameterId>
where
    ParameterId: std::hash::Hash + Clone + std::cmp::Eq + Copy + Debug + 'static,
{
    pub fn new(parameters: Vec<ParameterViewModel<ParameterId>>) -> Self {
        let parameter_order = parameters.iter().map(|parameter| parameter.id).collect();
        let mut states = HashMap::new();
        for parameter in parameters {
            states.insert(parameter.id, parameter);
        }

        Self {
            states,
            order: parameter_order,
        }
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
            state.knob_state.set_normal(Normal::from(
                (value - state.range.0) / (state.range.1 - state.range.0),
            ));
            if let Some(int_range) = &state.int_range {
                state.knob_state.snap_visible_to(int_range);
            }
            Some(state)
        } else {
            None
        }
    }

    pub fn view(&mut self) -> Element<KnobChanged<ParameterId>> {
        let order: HashMap<ParameterId, usize> = self
            .order
            .iter()
            .enumerate()
            .map(|(order, id)| (*id, order))
            .collect();
        let values_mut = self
            .states
            .values_mut()
            .sorted_by(|v1, v2| order[&v1.id].cmp(&order[&v2.id]));
        parameters_row(values_mut)
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
    let parameter_id = parameter_view_model.id;
    let mapped_value = parameter_view_model.value;
    let int_range = parameter_view_model.int_range;
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
