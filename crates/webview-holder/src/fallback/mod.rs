use serde::Serialize;
use std::error::Error;
use std::ffi::c_void;
use tokio::sync::broadcast::Sender;

pub struct WebviewHolder;

impl WebviewHolder {
    pub unsafe fn new(_size: (i32, i32)) -> WebviewHolder {
        WebviewHolder {}
    }

    pub unsafe fn initialize(&mut self, _parent: *mut c_void, _url: &str) {}

    pub unsafe fn attach_to_parent(&mut self, _parent: *mut c_void) {}
}

impl WebviewHolder {
    pub fn set_on_message_callback(&mut self, _on_message_callback: Sender<String>) {}
    pub fn clear_on_message_callback(&mut self) {}
    pub fn send_message<Msg>(&self, _message: &Msg) -> Result<(), Box<dyn Error>>
    where
        Msg: Serialize,
    {
        todo!()
    }
}
