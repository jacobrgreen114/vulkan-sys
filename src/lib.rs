// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
use windows::core::*;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
