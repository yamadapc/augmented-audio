#![allow(unknown_lints)]
#![allow(clippy)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// These lints ignore unsafe undefined behaviour
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(unaligned_references)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
