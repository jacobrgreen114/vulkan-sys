// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
use {
    core::ffi::c_uint as DWORD, windows::core::PCWSTR as LPCWSTR, windows::Win32::Foundation::*,
    windows::Win32::Graphics::Gdi::*, windows::Win32::Security::*,
};

// include bindgens generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Handwritten functions definitions because bindgen wraps then in ```Option```.
// todo - remove this when generating from bindgen
mod funcs;
pub use funcs::*;
