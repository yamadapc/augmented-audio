//! Stateless views for `MainContentView` layout

use iced::{Button, Column, Container, Element, Length, Row, Rule, Text};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;

use crate::ui::audio_io_settings;
use crate::ui::main_content_view::audio_chart::AudioChart;
use crate::ui::main_content_view::audio_file_chart::AudioFileModel;
use crate::ui::main_content_view::plugin_content::View;
use crate::ui::main_content_view::status_bar::StatusBar;
use crate::ui::main_content_view::transport_controls::TransportControlsView;
use crate::ui::main_content_view::volume_meter::VolumeMeter;
use crate::ui::main_content_view::{
    audio_chart, audio_file_chart, plugin_content, transport_controls, volume_meter, Message,
};

pub struct StartStopViewModel {
    pub is_started: bool,
    pub button_state: iced::button::State,
}

pub struct MainContentViewModel<'a> {
    pub audio_io_settings: &'a mut audio_io_settings::Controller,
    pub plugin_content: &'a mut View,
    pub audio_chart: &'a Option<AudioChart>,
    pub volume_handle: &'a Option<Shared<VolumeMeterProcessorHandle>>,
    pub volume_meter_state: &'a mut volume_meter::State,
    pub transport_controls: &'a mut TransportControlsView,
    pub status_message: &'a StatusBar,
    pub start_stop_button_state: &'a mut StartStopViewModel,
    pub audio_file_model: &'a AudioFileModel,
}

pub fn main_content_view(view_model: MainContentViewModel) -> Element<Message> {
    let MainContentViewModel {
        audio_io_settings,
        plugin_content,
        audio_chart,
        volume_handle,
        volume_meter_state,
        transport_controls,
        status_message,
        start_stop_button_state,
        audio_file_model: _,
    } = view_model;

    Row::with_children(vec![Column::with_children(vec![
        audio_io_settings.view().map(Message::AudioIOSettings),
        Container::new(
            Button::new(
                &mut start_stop_button_state.button_state,
                if start_stop_button_state.is_started {
                    Text::new("Stop audio engine")
                } else {
                    Text::new("Start audio engine")
                },
            )
            .on_press(Message::StartStopButtonClicked)
            .style(audio_processor_iced_design_system::style::Button),
        )
        .padding(Spacing::base_spacing())
        .center_x()
        .into(),
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
        plugin_content_container(plugin_content),
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
        bottom_visualisation_content_container(BottomVisualisationViewModel {
            audio_chart,
            volume_meter_state,
            volume_handle,
        }),
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
    .into()
}

struct BottomVisualisationViewModel<'a> {
    audio_chart: &'a Option<audio_chart::AudioChart>,
    volume_meter_state: &'a mut volume_meter::State,
    volume_handle: &'a Option<Shared<VolumeMeterProcessorHandle>>,
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
        volume_handle,
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
            Container::new(
                VolumeMeter::new(volume_handle.into())
                    .view(volume_meter_state)
                    .map(Message::VolumeMeter),
            )
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
