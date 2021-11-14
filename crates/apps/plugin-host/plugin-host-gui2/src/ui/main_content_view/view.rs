//! Stateless views for `MainContentView` layout

use iced::{button, Button, Column, Container, Element, Length, Row, Rule, Text};

use audio_processor_iced_design_system::spacing::Spacing;
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
    pub button_state: iced::button::State,
}

impl Default for StartStopViewModel {
    fn default() -> Self {
        Self {
            is_started: true,
            button_state: Default::default(),
        }
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
    pub navigation_header_state: &'a mut NavigationHeaderState,
    pub audio_io_settings: &'a mut audio_io_settings::Controller,
    pub plugin_content: &'a mut View,
    pub audio_chart: &'a mut Option<AudioChart>,
    pub volume_meter_state: &'a mut volume_meter::VolumeMeter,
    pub transport_controls: &'a mut TransportControlsView,
    pub status_message: &'a StatusBar,
    pub start_stop_button_state: &'a mut StartStopViewModel,
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
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
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
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
        content_view,
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
        transport_controls_container(transport_controls),
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
        status_message_container(status_message),
    ])
    .into()])
    .height(Length::Fill)
    .into()
}

struct BottomVisualisationViewModel<'a> {
    audio_chart: &'a mut Option<audio_chart::AudioChart>,
    volume_meter_state: &'a mut volume_meter::VolumeMeter,
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

fn plugin_content_container(plugin_content: &mut plugin_content::View) -> Element<Message> {
    Container::new(plugin_content.view().map(Message::PluginContent))
        .style(Container0::default())
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn transport_controls_container(
    transport_controls: &mut transport_controls::TransportControlsView,
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
pub struct NavigationHeaderState {
    main_button_state: button::State,
    settings_button_state: button::State,
}

fn navigation_header<'a>(
    route: &'a Route,
    navigation_header_state: &'a mut NavigationHeaderState,
    start_stop_button_state: &'a mut StartStopViewModel,
) -> Element<'a, Message> {
    let route_button = |button_state, route, label, is_active| {
        Button::new(button_state, Text::new(label))
            .style(route_button_style::RouteButtonStyle {
                is_active,
                button: audio_processor_iced_design_system::style::button::Button,
            })
            .on_press(Message::SetRoute(route))
            .into()
    };

    let route_buttons = Row::with_children(vec![
        route_button(
            &mut navigation_header_state.main_button_state,
            Route::Development,
            "Main",
            Route::Development == *route,
        ),
        route_button(
            &mut navigation_header_state.settings_button_state,
            Route::Settings,
            "Settings",
            Route::Settings == *route,
        ),
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
    use augmented::gui::design::colors::Colors;
    use augmented::gui::iced::button::Style;

    pub struct RouteButtonStyle {
        pub is_active: bool,
        pub button: audio_processor_iced_design_system::style::button::Button,
    }

    impl iced::button::StyleSheet for RouteButtonStyle {
        fn active(&self) -> Style {
            Style {
                border_color: if self.is_active {
                    Colors::active_border_color()
                } else {
                    Colors::border_color()
                },
                ..self.button.active()
            }
        }

        fn hovered(&self) -> Style {
            self.button.hovered()
        }

        fn pressed(&self) -> Style {
            self.button.pressed()
        }

        fn disabled(&self) -> Style {
            self.button.disabled()
        }
    }
}

/// Start/stop engine button view
fn start_stop_view(start_stop_button_state: &mut StartStopViewModel) -> Element<Message> {
    Button::new(
        &mut start_stop_button_state.button_state,
        if start_stop_button_state.is_started {
            Text::new("Stop audio engine")
        } else {
            Text::new("Start audio engine")
        },
    )
    .on_press(Message::StartStopButtonClicked)
    .style(audio_processor_iced_design_system::style::Button)
    .into()
}
