mod plugin_parameter;

#[macro_use]
extern crate vst;
extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate darwin_webkit;
extern crate oscillator;

use cocoa::appkit::{NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::{make_new_handler, WKScriptMessage};
use darwin_webkit::webkit::WKUserContentController;
use objc::runtime::BOOL;
use oscillator::Oscillator;
use plugin_parameter::{ParameterStore, PluginParameterImpl};
use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Instant;
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
        // If there's already a webview just re-attach
        if let Some(webview) = &self.webview {
            TremoloEditor::attach_to_parent(parent, ns_size, &webview);
            return Some(true);
        }

        let (ns_size, mut webview) = TremoloEditor::create_webview();
        TremoloEditor::attach_to_parent(parent, ns_size, &webview);

        self.attach_message_handler(&mut webview);
        webview.load_url("http://127.0.0.1:3000");
        self.webview = Some(webview);

        Some(true)
    }

    unsafe fn attach_message_handler(&mut self, webview: &mut DarwinWKWebView) {
        let mut self_ptr: *mut Self = self;
        let on_message_ptr = Box::into_raw(Box::new(move |_, wk_script_message| {
            (*self_ptr).on_message(wk_script_message);
        }));
        let webview = self.webview.as_mut().unwrap();
        let name = "editor";

        // This creates a new objective-c class for the message handler
        let handler = make_new_handler(
            format!(
                "DWKHandler_{}__{}",
                name,
                Instant::now().elapsed().as_micros()
            )
            .as_str(),
            on_message_ptr,
        );

        let name = NSString::alloc(nil).init_str(name);
        webview
            .get_user_content_controller_handle()
            .addScriptMessageHandler(handler, name);
    }

    unsafe fn create_webview(&self) -> (NSSize, DarwinWKWebView) {
        let origin = NSPoint::new(0.0, 0.0);
        let (width, height) = self.size();
        let size = NSSize::new(width as f64, height as f64);
        let frame = NSRect::new(origin, size);
        let webview = darwin_webkit::helpers::dwk_webview::DarwinWKWebView::new(frame);
        (size, webview)
    }

    unsafe fn attach_to_parent(parent: *mut c_void, ns_size: NSSize, webview: &DarwinWKWebView) {
        let parent_id = parent as id;
        parent_id.addSubview_(webview.get_native_handle());
        let window_id: id = msg_send![parent_id, window];
        window_id.setStyleMask_(
            NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSClosableWindowMask,
        );
        window_id.setMinSize_(ns_size);
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
            let webview = self.webview.as_ref().unwrap().get_native_handle();

            let msg = NSString::alloc(nil).init_str("window.audioRuntime.ok()");
            let _: () = msg_send![webview, evaluateJavaScript: msg];
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
        // self.webview = None
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}

plugin_main!(TremoloPlugin); // Important!
