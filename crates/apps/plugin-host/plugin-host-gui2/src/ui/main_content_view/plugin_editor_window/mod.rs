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
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use actix::Addr;
use iced::Rectangle;
use vst::editor::Editor;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use plugin_host_lib::actor_system::ActorSystemThread;
use plugin_host_lib::audio_io::GetPluginInstanceMessage;
use plugin_host_lib::processors::shared_processor::SharedProcessor;
use plugin_host_lib::TestPluginHost;

mod view;

pub trait IEditorPluginInstanceProvider {
    fn plugin_instance(&self) -> Option<SharedProcessor<PluginInstance>>;
}

impl IEditorPluginInstanceProvider for Arc<Mutex<TestPluginHost>> {
    fn plugin_instance(&self) -> Option<SharedProcessor<PluginInstance>> {
        self.lock().unwrap().plugin_instance()
    }
}

impl IEditorPluginInstanceProvider for Addr<TestPluginHost> {
    fn plugin_instance(&self) -> Option<SharedProcessor<PluginInstance>> {
        let addr = self.clone();
        ActorSystemThread::current()
            .spawn_result(async move { addr.send(GetPluginInstanceMessage).await.unwrap() })
    }
}

pub enum ClosePluginWindowResult {
    NoWindow,
    ClosedPlugin { window_frame: Rectangle },
}

pub struct EditorController<EditorPluginInstanceProvider: IEditorPluginInstanceProvider> {
    plugin_instance_provider: EditorPluginInstanceProvider,
    editor: Option<Box<dyn Editor>>,
    window_handle: Option<view::PluginWindowHandle>,
    /// Whether the editor window is above all others
    editor_is_floating: bool,
    /// Cached window frame from a previous editor open
    previous_plugin_window_frame: Option<Rectangle>,
}

impl<EditorPluginInstanceProvider: IEditorPluginInstanceProvider>
    EditorController<EditorPluginInstanceProvider>
{
    pub fn new(plugin_instance_provider: EditorPluginInstanceProvider) -> Self {
        EditorController {
            plugin_instance_provider,
            editor: None,
            window_handle: None,
            editor_is_floating: false,
            previous_plugin_window_frame: None,
        }
    }

    pub fn open_window(&mut self) {
        if self.window_handle.is_some() {
            log::warn!("Refusing to open 2 plugin editors");
            return;
        }

        log::info!("Opening plugin editor");
        let frame = self
            .previous_plugin_window_frame
            .map(|frame| frame.position());
        let editor = self.editor();

        if let Some(editor) = editor {
            log::info!("Found plugin editor");
            let size = editor.size();
            let window = view::open_plugin_window(editor, size, frame);
            log::info!("Opened editor window");
            self.window_handle = Some(window);
        } else {
            log::error!("No editor returned");
        }

        if self.editor_is_floating {
            let _ = self.float_window();
        }
    }

    pub fn close_window(&mut self) -> ClosePluginWindowResult {
        if let Some(window_handle) = self.window_handle.take() {
            log::info!("Closing plugin editor");
            if let Some(editor) = &mut self.editor {
                editor.close();
            }
            log::info!("Closing plugin window");
            let frame = view::close_window(window_handle.raw_window_handle);
            match frame {
                Some(frame) => {
                    self.previous_plugin_window_frame = Some(frame);
                    ClosePluginWindowResult::ClosedPlugin {
                        window_frame: frame,
                    }
                }
                None => ClosePluginWindowResult::NoWindow,
            }
        } else {
            log::warn!("Close requested, but there's no plugin window handle");
            ClosePluginWindowResult::NoWindow
        }
    }

    pub fn float_window(&mut self) {
        self.editor_is_floating = true;
        if let Some(handle) = &mut self.window_handle {
            handle.float();
        }
    }

    pub fn on_reload(&mut self) -> bool {
        self.editor = None;
        matches!(
            self.close_window(),
            ClosePluginWindowResult::ClosedPlugin { .. }
        )
    }

    fn editor(&mut self) -> Option<&mut Box<dyn Editor>> {
        if self.editor.is_some() {
            return self.editor.as_mut();
        }

        let instance = self.plugin_instance_provider.plugin_instance();

        log::info!("Found plugin instance");
        let instance_ptr = instance?.deref() as *const PluginInstance as *mut PluginInstance;
        if let Some(editor) = unsafe { instance_ptr.as_mut() }.unwrap().get_editor() {
            self.editor = Some(editor);
            self.editor.as_mut()
        } else {
            None
        }
    }
}
