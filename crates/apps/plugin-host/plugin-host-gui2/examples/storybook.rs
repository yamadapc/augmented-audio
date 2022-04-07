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
use derive_more::{From, TryInto};

use plugin_host_gui2::ui;
use ui::audio_io_settings;
use ui::audio_io_settings::dropdown_with_label;
use ui::main_content_view::{plugin_content, status_bar, transport_controls, volume_meter};

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    AudioIOSettings(audio_io_settings::view::Message),
    TransportControls(transport_controls::Message),
    Dropdown(String),
    PluginContent(plugin_content::Message),
    VolumeMeter(volume_meter::Message),
    None(()),
}

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<Message>()
        .story("Dropdown with label", dropdown_with_label::story::default())
        .story("AudioIOSettings", audio_io_settings::view::story::default())
        .story("Transport controls", transport_controls::story::default())
        .story("Plugin content", plugin_content::story::default())
        .story("Volume meter", volume_meter::story::default())
        .story("Status bar", status_bar::story::default())
        .run()
}
