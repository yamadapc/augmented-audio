//! Forked from https://github.com/jonas-k/macos-multitouch/blob/master/src/lib.rs
use std::ffi::{c_double, c_int, c_void};

#[derive(Debug)]
#[repr(C)]
pub struct MtPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MtReadout {
    pub pos: MtPoint,
    pub vel: MtPoint,
}

#[derive(Debug)]
#[repr(C)]
pub struct Finger {
    pub frame: i32,
    pub timestamp: f64,
    pub identifier: i32,
    pub state: i32,
    pub finger_number: i32,
    pub unknown0: i32,
    pub normalized: MtReadout,
    pub size: f32,
    pub unknown1: i32,
    pub angle: f32,      // \
    pub major_axis: f32, //  |- ellipsoid
    pub minor_axis: f32, // /
    pub mm: MtReadout,
    pub unknown2: [i32; 2],
    pub unknown3: f32,
}

type MTDeviceRef = *const c_void;

#[link(name = "MultitouchSupport", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn MTRegisterContactFrameCallbackWithRefcon(
        device: MTDeviceRef,
        callback: extern "C" fn(MTDeviceRef, &Finger, c_int, c_double, c_int, *mut c_void) -> c_int,
        user_data: *mut c_void,
    ) -> c_void;
    fn MTDeviceStart(device: MTDeviceRef, number: i32) -> c_void;
    fn MTDeviceStop(device: MTDeviceRef, number: i32) -> c_void;
    fn MTDeviceCreateList() -> core_foundation_sys::array::CFArrayRef;
}

pub struct MultitouchDevice {
    _device: MTDeviceRef,
    is_started: bool,
}

extern "C" fn callback_handler(
    device: MTDeviceRef,
    data: &Finger,
    length: c_int,
    timestamp: c_double,
    frame: c_int,
    user_data: *mut c_void,
) -> c_int {
    let closure: &mut &mut dyn FnMut(MTDeviceRef, &[Finger], f64, i32) =
        unsafe { std::mem::transmute(user_data) };
    let fingers = unsafe { std::slice::from_raw_parts(data, length as usize) };
    closure(device, fingers, timestamp, frame);

    return 0 as c_int;
}

impl MultitouchDevice {
    fn new(device: MTDeviceRef) -> MultitouchDevice {
        MultitouchDevice {
            _device: device,
            is_started: false,
        }
    }

    pub fn register_contact_frame_callback<F>(&mut self, callback: F) -> Result<(), &'static str>
    where
        F: FnMut(MTDeviceRef, &[Finger], f64, i32),
    {
        if !self.is_started {
            let cb: Box<Box<dyn FnMut(MTDeviceRef, &[Finger], f64, i32)>> =
                Box::new(Box::new(callback));
            unsafe {
                MTRegisterContactFrameCallbackWithRefcon(
                    self._device,
                    callback_handler,
                    Box::into_raw(cb) as *mut _,
                );
            }
            self.is_started = true;
            unsafe { MTDeviceStart(self._device, 0) };
            return Ok(());
        }

        Err("There is already a callback registered to this device.")
    }

    pub fn stop(&mut self) {
        unsafe { MTDeviceStop(self._device, 0) };
    }
}

pub fn get_multitouch_devices() -> Vec<MultitouchDevice> {
    let device_list = unsafe { MTDeviceCreateList() };
    let count = unsafe { core_foundation_sys::array::CFArrayGetCount(device_list) };

    let mut ret_val: Vec<MultitouchDevice> = Vec::new();
    for i in 0..count {
        ret_val.push(MultitouchDevice::new(unsafe {
            core_foundation_sys::array::CFArrayGetValueAtIndex(device_list, i)
        }));
    }

    ret_val
}
