// = copyright ====================================================================
// Simple Metronome: macOS Metronome app
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================

#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use audio_processor_metronome::*;
pub use bridge_generated::*;

mod api;
mod bridge_generated;

#[cfg(target_os = "android")]
pub use android_init::*;

#[cfg(target_os = "android")]
mod android_init {
    use std::ffi::c_void;

    #[no_mangle]
    pub extern "C" fn JNI_OnLoad(
        vm: jni::JavaVM,
        res: *mut std::os::raw::c_void,
    ) -> jni::sys::jint {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Info)
                .with_tag("metronome")
                .with_tag("native"),
        );
        log::info!("JNI_OnLoad called");
        let _env = vm.get_env().unwrap();
        let vm = vm.get_java_vm_pointer() as *mut c_void;
        unsafe {
            ndk_context::initialize_android_context(vm, res);
        }
        jni::JNIVersion::V6.into()
    }
}
