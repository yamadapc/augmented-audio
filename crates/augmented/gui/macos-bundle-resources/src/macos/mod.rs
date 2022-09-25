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
use std::ffi::CString;
use std::path::PathBuf;

use core_foundation::base::CFAllocatorGetDefault;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetMainBundle};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFStringCreateWithCString, CFStringGetCString, CFStringGetLength,
};
use core_foundation::url::CFURLGetString;
use core_foundation_sys::bundle::CFBundleRef;
use core_foundation_sys::string::CFStringRef;
use core_foundation_sys::url::CFURLRef;

extern "C" {
    pub fn CFBundleCopyResourceURL(
        bundle: CFBundleRef,
        resource_name: CFStringRef,
        resource_type: CFStringRef,
        sub_dir_name: CFStringRef,
    ) -> CFURLRef;
}

/// Build a CFStringRef out of a &str ref.
fn make_cfstring(s: &str) -> Option<CFStringRef> {
    unsafe {
        let allocator = CFAllocatorGetDefault();
        let c_str = CString::new(s).ok()?;
        let cfstring_ref =
            CFStringCreateWithCString(allocator, c_str.as_ptr(), kCFStringEncodingUTF8);

        if cfstring_ref.is_null() {
            return None;
        }

        Some(cfstring_ref)
    }
}

/// Check if there's a non-null main CFBundle.
pub fn has_main_bundle() -> bool {
    unsafe {
        let main_bundle = CFBundleGetMainBundle();
        !main_bundle.is_null()
    }
}

/// Check if there's a non-null CFBundle with this identifier.
pub fn has_bundle(bundle_identifier: &str) -> bool {
    unsafe {
        let bundle_identifier = make_cfstring(bundle_identifier);
        if let Some(bundle_identifier) = bundle_identifier {
            let bundle = CFBundleGetBundleWithIdentifier(bundle_identifier);
            !bundle.is_null()
        } else {
            false
        }
    }
}

/// Build a `String` from a `CFStringRef`.
fn string_from_cfstring(url_cfstring: CFStringRef) -> Option<String> {
    unsafe {
        let length = CFStringGetLength(url_cfstring) + 1;
        let mut output_str = String::with_capacity(length as usize);
        for _ in 0..length {
            output_str.push(' ');
        }
        let output_str = CString::new(output_str).ok()?;
        let output_str = output_str.into_raw();
        let result = CFStringGetCString(url_cfstring, output_str, length, kCFStringEncodingUTF8);
        if result == 0 {
            return None;
        }
        let output_str = CString::from_raw(output_str);
        let output_str = output_str.to_str().ok()?;
        Some(output_str.to_string())
    }
}

/// Get the path to a resource
pub fn get_path(
    bundle_identifier: &str,
    resource_name: &str,
    resource_type: Option<&str>,
    sub_dir_name: Option<&str>,
) -> Option<PathBuf> {
    let resource_name = make_cfstring(resource_name)?;
    let resource_type = resource_type
        .and_then(make_cfstring)
        .unwrap_or(std::ptr::null());
    let sub_dir_name = sub_dir_name
        .and_then(make_cfstring)
        .unwrap_or(std::ptr::null());

    unsafe {
        log::debug!("Getting bundle {}", bundle_identifier);
        let bundle_identifier = make_cfstring(bundle_identifier)?;
        let main_bundle = CFBundleGetBundleWithIdentifier(bundle_identifier);
        if main_bundle.is_null() {
            return None;
        }

        log::debug!("Getting resource URL");
        let url_ref =
            CFBundleCopyResourceURL(main_bundle, resource_name, resource_type, sub_dir_name);
        if url_ref.is_null() {
            return None;
        }

        log::debug!("Converting URL to string");
        let url_cfstring = CFURLGetString(url_ref);
        if url_cfstring.is_null() {
            return None;
        }

        let output_str = string_from_cfstring(url_cfstring)?;
        Some(PathBuf::from(output_str))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_cfstring() {
        let str = "something here yada yada";
        let cf_str = make_cfstring(str);
        assert!(cf_str.is_some());
        let cf_str = cf_str.unwrap();
        assert_ne!(cf_str, std::ptr::null());
        assert_eq!(string_from_cfstring(cf_str).unwrap(), String::from(str));
    }

    #[test]
    fn test_make_cfstring_twice_is_safe() {
        let str = "something here";
        {
            let cf_str = make_cfstring(str);
            assert!(cf_str.is_some());
            let cf_str = cf_str.unwrap();
            assert_ne!(cf_str, std::ptr::null());
            assert_eq!(string_from_cfstring(cf_str).unwrap(), String::from(str));
        }
        {
            let cf_str = make_cfstring(str);
            assert!(cf_str.is_some());
            let cf_str = cf_str.unwrap();
            assert_ne!(cf_str, std::ptr::null());
            assert_eq!(string_from_cfstring(cf_str).unwrap(), String::from(str));
        }
    }
}
