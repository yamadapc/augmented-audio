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

pub fn has_main_bundle() -> bool {
    unsafe {
        let main_bundle = CFBundleGetMainBundle();
        !main_bundle.is_null()
    }
}

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

fn str_from_cfstring(url_cfstring: CFStringRef) -> Option<String> {
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

/// Get the path to a resource on the main bundle
pub fn get_path(
    bundle_identifier: &str,
    resource_name: &str,
    resource_type: Option<&str>,
    sub_dir_name: Option<&str>,
) -> Option<PathBuf> {
    let resource_name = make_cfstring(resource_name)?;
    let resource_type = resource_type
        .map(|resource_type| make_cfstring(resource_type))
        .flatten()
        .unwrap_or(std::ptr::null());
    let sub_dir_name = sub_dir_name
        .map(|sub_dir_name| make_cfstring(sub_dir_name))
        .flatten()
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

        let output_str = str_from_cfstring(url_cfstring)?;

        Some(PathBuf::from(output_str))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_cfstring() {
        let str = "seomthing here yada yada";
        let cf_str = make_cfstring(str);
        assert!(cf_str.is_some());
        let cf_str = cf_str.unwrap();
        assert_ne!(cf_str, std::ptr::null());
        assert_eq!(str_from_cfstring(cf_str).unwrap(), String::from(str));
    }
}
