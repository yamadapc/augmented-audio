use iced::Application;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    plugin_host_gui2::App::run(audio_processor_iced_design_system::default_settings())
}
