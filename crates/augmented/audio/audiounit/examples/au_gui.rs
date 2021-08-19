use cacao::foundation::id;
use cacao::macos::window::Window;
use cacao::macos::{App, AppDelegate};
use cacao::objc::runtime::Object;
use cacao::utils::Controller;
use objc_id::ShareId;

use audiounit::{AUAudioUnit, AVAudioUnitComponentManager};

struct AuViewController(id);

impl Controller for AuViewController {
    fn get_backing_node(&self) -> ShareId<Object> {
        unsafe { ShareId::from_ptr(self.0) }
    }
}

#[derive(Default)]
struct AuGuiApp {
    window: Window,
}

impl AppDelegate for AuGuiApp {
    fn did_finish_launching(&self) {
        App::activate();

        let audio_unit_manager = AVAudioUnitComponentManager::shared();
        let components = audio_unit_manager.all_components();
        let component_with_view = components
            .iter()
            .find(|component| component.has_custom_view())
            .unwrap();

        let audio_unit_descr = component_with_view.audio_component_description();
        let audio_unit = AUAudioUnit::instantiate(audio_unit_descr);
        let view_controller = AuViewController(audio_unit.request_view_controller());

        self.window.set_minimum_content_size(500., 500.);
        self.window.set_title("Content");
        self.window.set_content_view_controller(&view_controller);
        self.window.show();
    }
}

fn main() {
    App::new("com.augmented.augui", AuGuiApp::default()).run()
}
