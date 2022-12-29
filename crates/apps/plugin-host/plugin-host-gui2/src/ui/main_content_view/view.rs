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
//! Stateless views for `MainContentView` layout

use iced::{
    widget::{Button, Column, Container, Row, Rule, Text},
    Element, Length,
};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style;
use audio_processor_iced_design_system::style::{Container0, Container1};

use crate::ui::audio_io_settings;
use crate::ui::main_content_view::audio_chart::AudioChart;
use crate::ui::main_content_view::audio_file_chart::AudioFileModel;
use crate::ui::main_content_view::plugin_content::View;
use crate::ui::main_content_view::status_bar::StatusBar;
use crate::ui::main_content_view::transport_controls::TransportControlsView;
use crate::ui::main_content_view::{
    audio_chart, audio_file_chart, plugin_content, transport_controls, volume_meter, Message,
};

pub struct StartStopViewModel {
    pub is_started: bool,
}

impl Default for StartStopViewModel {
    fn default() -> Self {
        Self { is_started: true }
    }
}

#[derive(derivative::Derivative)]
#[derivative(PartialEq, Debug, Clone)]
pub enum Route {
    Development,
    Settings,
}

pub struct MainContentViewModel<'a> {
    pub route: &'a Route,
    pub navigation_header_state: &'a NavigationHeaderState,
    pub audio_io_settings: &'a audio_io_settings::Controller,
    pub plugin_content: &'a View,
    pub audio_chart: &'a Option<AudioChart>,
    pub volume_meter_state: &'a volume_meter::VolumeMeter,
    pub transport_controls: &'a TransportControlsView,
    pub status_message: &'a StatusBar,
    pub start_stop_button_state: &'a StartStopViewModel,
    pub audio_file_model: &'a AudioFileModel,
}

pub fn main_content_view(view_model: MainContentViewModel) -> Element<Message> {
    let MainContentViewModel {
        route,
        navigation_header_state,
        audio_io_settings,
        plugin_content,
        audio_chart,
        volume_meter_state,
        transport_controls,
        status_message,
        start_stop_button_state,
        audio_file_model: _,
    } = view_model;

    let content_view = match route {
        Route::Development => Row::with_children(vec![Column::with_children(vec![
            plugin_content_container(plugin_content),
            Rule::horizontal(1).style(style::Rule).into(),
            bottom_visualisation_content_container(BottomVisualisationViewModel {
                audio_chart,
                volume_meter_state,
            }),
        ])
        .into()])
        .height(Length::Fill)
        .into(),
        Route::Settings => Container::new(audio_io_settings.view().map(Message::AudioIOSettings))
            .height(Length::Fill)
            .into(),
    };

    Row::with_children(vec![Column::with_children(vec![
        navigation_header(route, navigation_header_state, start_stop_button_state),
        Rule::horizontal(1).style(style::Rule).into(),
        content_view,
        Rule::horizontal(1).style(style::Rule).into(),
        transport_controls_container(transport_controls),
        Rule::horizontal(1).style(style::Rule).into(),
        status_message_container(status_message),
    ])
    .into()])
    .height(Length::Fill)
    .into()
}

struct BottomVisualisationViewModel<'a> {
    audio_chart: &'a Option<audio_chart::AudioChart>,
    volume_meter_state: &'a volume_meter::VolumeMeter,
}

#[allow(dead_code)]
fn audio_file_visualization(audio_file_model: &AudioFileModel) -> Element<Message> {
    Container::new(
        audio_file_chart::View::new(audio_file_model)
            .view()
            .map(|_| Message::None),
    )
    .width(Length::Fill)
    .max_height(200)
    .into()
}

fn bottom_visualisation_content_container(
    view_model: BottomVisualisationViewModel,
) -> Element<Message> {
    let BottomVisualisationViewModel {
        audio_chart,
        volume_meter_state,
    } = view_model;
    Container::new(
        Row::with_children(vec![
            Container::new(
                Container::new::<Element<Message>>(match audio_chart {
                    Some(chart) => chart.view().map(|_| Message::None),
                    None => Text::new("").into(),
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .style(Container0::default().border_radius(8.0)),
            )
            .padding(Spacing::base_spacing())
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),
            Container::new(volume_meter_state.view().map(Message::VolumeMeter))
                .style(Container1::default().border())
                .width(Length::Units(Spacing::base_control_size() * 2))
                .height(Length::Fill)
                .into(),
        ])
        .width(Length::Fill),
    )
    .style(Container1::default())
    .height(Length::Units(300))
    .width(Length::Fill)
    .into()
}

fn plugin_content_container(plugin_content: &plugin_content::View) -> Element<Message> {
    Container::new(plugin_content.view().map(Message::PluginContent))
        .style(Container0::default())
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn transport_controls_container(
    transport_controls: &transport_controls::TransportControlsView,
) -> Element<Message> {
    Container::new(transport_controls.view().map(Message::TransportControls))
        .style(Container1::default())
        .height(Length::Units(80))
        .width(Length::Fill)
        .into()
}

fn status_message_container(status_message: &StatusBar) -> Element<Message> {
    Container::new(status_message.clone().view().map(|_| Message::None))
        .center_y()
        .style(Container0::default())
        .height(Length::Units(20))
        .width(Length::Fill)
        .into()
}

#[derive(Default)]
pub struct NavigationHeaderState {}

fn navigation_header<'a>(
    route: &'a Route,
    _navigation_header_state: &'a NavigationHeaderState,
    start_stop_button_state: &'a StartStopViewModel,
) -> Element<'a, Message> {
    let route_button = |route, label, is_active| {
        Button::new(Text::new(label))
            .style(
                route_button_style::RouteButtonStyle {
                    is_active,
                    button: style::button::Button::default(),
                }
                .into(),
            )
            .on_press(Message::SetRoute(route))
            .into()
    };

    let route_buttons = Row::with_children(vec![
        route_button(Route::Development, "Main", Route::Development == *route),
        route_button(Route::Settings, "Settings", Route::Settings == *route),
    ])
    .spacing(Spacing::base_spacing())
    .into();

    Row::with_children(vec![
        start_stop_view(start_stop_button_state),
        route_buttons,
    ])
    .padding(Spacing::base_spacing())
    .spacing(Spacing::base_spacing())
    .into()
}

mod route_button_style {
    use iced::widget::button;

    use audio_processor_iced_design_system::style;
    use augmented::gui::iced::widget::button::{Appearance, StyleSheet};
    use augmented::gui::iced_design_system::colors::Colors;

    pub struct RouteButtonStyle {
        pub is_active: bool,
        pub button: style::button::Button,
    }

    impl From<RouteButtonStyle> for iced::theme::Button {
        fn from(value: RouteButtonStyle) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl StyleSheet for RouteButtonStyle {
        type Style = iced::Theme;

        fn active(&self, _theme: &Self::Style) -> Appearance {
            Appearance {
                border_color: if self.is_active {
                    Colors::active_border_color()
                } else {
                    Colors::border_color()
                },
                ..button::StyleSheet::active(&self.button, _theme)
            }
        }

        fn hovered(&self, _theme: &Self::Style) -> Appearance {
            button::StyleSheet::hovered(&self.button, _theme)
        }

        fn pressed(&self, _theme: &Self::Style) -> Appearance {
            button::StyleSheet::pressed(&self.button, _theme)
        }

        fn disabled(&self, _theme: &Self::Style) -> Appearance {
            button::StyleSheet::disabled(&self.button, _theme)
        }
    }
}

/// Start/stop engine button view
fn start_stop_view(start_stop_button_state: &StartStopViewModel) -> Element<Message> {
    Button::new(if start_stop_button_state.is_started {
        Text::new("Stop audio engine")
    } else {
        Text::new("Start audio engine")
    })
    .on_press(Message::StartStopButtonClicked)
    .style(style::Button::default().into())
    .into()
}
