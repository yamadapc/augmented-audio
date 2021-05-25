mod plugin_parameter;

#[macro_use]
extern crate vst;
extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate darwin_webkit;
extern crate oscillator;

use cocoa::appkit::{NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::WKScriptMessage;
use objc::runtime::BOOL;
use oscillator::Oscillator;
use plugin_parameter::{ParameterStore, PluginParameterImpl};
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};

static RATE_PARAMETER_ID: &str = "rate";
static DEPTH_PARAMETER_ID: &str = "depth";

struct TremoloParameters {}

impl PluginParameters for TremoloParameters {}

struct TremoloPlugin {
    parameters: Arc<ParameterStore>,
    oscillator_left: Oscillator<f32>,
    oscillator_right: Oscillator<f32>,
}

impl TremoloPlugin {
    fn build_parameters() -> ParameterStore {
        let mut store = ParameterStore::new();
        store.add_parameter(
            String::from(RATE_PARAMETER_ID),
            Arc::new(Mutex::new(PluginParameterImpl::new_with(
                String::from("Rate"),
                String::from("Hz"),
                0.1,
                true,
            ))),
        );
        store.add_parameter(
            String::from(DEPTH_PARAMETER_ID),
            Arc::new(Mutex::new(PluginParameterImpl::new_with(
                String::from("Depth"),
                String::from(""),
                1.0,
                true,
            ))),
        );
        store
    }
}

impl Plugin for TremoloPlugin {
    fn new(_host: HostCallback) -> Self {
        TremoloPlugin {
            parameters: Arc::new(TremoloPlugin::build_parameters()),
            oscillator_left: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::sine_generator,
            ),
            oscillator_right: Oscillator::new_with_sample_rate(
                44100.,
                oscillator::generators::sine_generator,
            ),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "TasV2".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2501, // Used by hosts to differentiate between plugins.
            parameters: self.parameters.get_num_parameters(),
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        println!("TremoloPlugin - set_sample_rate");
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    // TODO - why isn't this called?
    fn start_process(&mut self) {
        println!("TremoloPlugin - start_process");
        self.oscillator_left.set_frequency(0.1);
        self.oscillator_right.set_frequency(0.1);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if buffer.input_count() != buffer.output_count() {
            panic!("Unsupported input/output mismatch");
        }

        let num_channels = buffer.input_count();
        let num_samples = buffer.samples();
        let (input, mut output) = buffer.split();

        for channel in 0..num_channels {
            if channel > 2 {
                break;
            }

            let osc = if channel == 0 {
                &mut self.oscillator_left
            } else {
                &mut self.oscillator_right
            };
            let input_samples = input.get(channel);
            let output_samples = output.get_mut(channel);

            for sample_index in 0..num_samples {
                let volume = osc.next();
                output_samples[sample_index] = volume * input_samples[sample_index];
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(TremoloEditor::new(self.parameters.clone())))
    }
}

struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    webview: Option<DarwinWKWebView>,
}

impl TremoloEditor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        TremoloEditor {
            parameters,
            webview: None,
        }
    }

    unsafe fn initialize_webview(&mut self, parent: *mut c_void) -> Option<bool> {
        let origin = NSPoint::new(0.0, 0.0);
        let size = NSSize::new(500.0, 500.0);
        let frame = NSRect::new(origin, size);
        let webview = darwin_webkit::helpers::dwk_webview::DarwinWKWebView::new(frame);
        let parent_id = parent as id;

        // TODO - this is only for development
        webview.load_url("http://localhost:3000");
        parent_id.addSubview_(webview.get_native_handle());

        let window_id: id = msg_send![parent_id, window];
        window_id.setStyleMask_(
            NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSClosableWindowMask,
        );
        window_id.setMinSize_(size);
        self.webview = Some(webview);

        let mut self_ptr: *mut Self = self;
        let on_message_ptr = Box::into_raw(Box::new(|_, wk_script_message| {
            (*self_ptr).on_message(wk_script_message);
        }));
        let webview = self.webview.as_mut().unwrap();
        webview.add_message_handler("editor", on_message_ptr);

        Some(true)
    }

    unsafe fn on_message(&mut self, wk_script_message: id) {
        // https://developer.apple.com/documentation/webkit/wkscriptmessage/1417901-body?language=objc
        // Allowed types are NSNumber, NSString, NSDate, NSArray, NSDictionary, and NSNull.
        let body = wk_script_message.body();

        // only support string for simplicity
        let string_class = class!(NSString);
        let is_string: BOOL = msg_send![body, isKindOfClass: string_class];
        if is_string == YES {
            let str = string_from_nsstring(body);
            println!("Got message from JavaScript {}", str.as_ref().unwrap());
            self.webview
                .as_ref()
                .unwrap()
                .evaluate_javascript("alert('got message')");
        }
    }
}

impl Editor for TremoloEditor {
    fn size(&self) -> (i32, i32) {
        (500, 500)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        unsafe { self.initialize_webview(parent).unwrap_or(false) }
    }

    fn close(&mut self) {
        self.webview = None
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}

plugin_main!(TremoloPlugin); // Important!
