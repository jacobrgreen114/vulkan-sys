// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use bindgen;
use std::env::var;
use std::path::PathBuf;

macro_rules! cargo_warning {
    ($($arg:tt)*) => {
        println!("cargo:warning={}", format!($($arg)*));
    };
}

macro_rules! cargo_panic {
    ($($arg:tt)*) => {
        cargo_warning!($($arg)*);
        panic!($($arg)*);
    };
}

const RUSTIFIED_ENUMS: &[&str] = &["VkObjectType"];
const BITFIELD_ENUMS: &[&str] = &[];

fn main() {
    let vulkan_sdk_path = PathBuf::from(var("VULKAN_SDK").expect("VULKAN_SDK not set"));
    let vulkan_include_dir: PathBuf = vulkan_sdk_path.join("Include");
    let vulkan_lib_dir: PathBuf = vulkan_sdk_path.join("Lib");
    let vulkan_lib_path = vulkan_lib_dir.join("vulkan-1.lib");

    (!vulkan_lib_path.exists()).then(|| {
        cargo_panic!(
            "cargo:warning=Vulkan library not found at {:?}",
            vulkan_lib_path
        );
    });

    println!(
        "cargo:rustc-link-search={}",
        vulkan_lib_dir.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib={}", "vulkan-1");

    let vulkan_header_rel_path = PathBuf::from("vulkan").join("vulkan.h");
    let vulkan_header_path = vulkan_include_dir.join(vulkan_header_rel_path);

    (!vulkan_header_path.exists()).then(|| {
        cargo_panic!(
            "cargo:warning=Vulkan header not found at {:?}",
            vulkan_header_path
        );
    });

    let platform_define = if cfg!(target_os = "windows") {
        "VK_USE_PLATFORM_WIN32_KHR"
    } else if cfg!(target_os = "linux") {
        "VK_USE_PLATFORM_WAYLAND_KHR"
    } else if cfg!(target_os = "macos") {
        "VK_USE_PLATFORM_MACOS_MVK"
    } else {
        cargo_panic!("Platform not currently supported");
    };

    let mut builder = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-I", vulkan_include_dir.to_str().unwrap()])
        .clang_args(&["-L", vulkan_lib_dir.to_str().unwrap()])
        .clang_args(&["-D", platform_define])
        .header(vulkan_header_path.to_str().unwrap())
        .allowlist_recursively(false)
        .allowlist_file(".*vulkan.*")
        .allowlist_file(".*vk_video.*")
        .allowlist_type("Vk.*")
        .allowlist_type("PFN_vk.*")
        .allowlist_function("vk.*")
        .allowlist_var("VK.*")
        .prepend_enum_name(false);

    for rustified in RUSTIFIED_ENUMS {
        builder = builder.rustified_non_exhaustive_enum(rustified)
    }

    for bitfield in BITFIELD_ENUMS {
        builder = builder.bitfield_enum(bitfield)
    }

    let bindings = builder
        .generate()
        .inspect_err(|e| {
            cargo_panic!("{}", e);
        })
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .inspect_err(|e| {
            cargo_panic!("{}", e);
        })
        .unwrap();
}
