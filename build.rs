// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use bindgen;
use std::env::var;
use std::path::PathBuf;

fn main() {
    let vulkan_sdk_path = PathBuf::from(var("VULKAN_SDK").expect("VULKAN_SDK not set"));
    let vulkan_include_dir: PathBuf = vulkan_sdk_path.join("Include");
    let vulkan_lib_dir: PathBuf = vulkan_sdk_path.join("Lib");
    let vulkan_lib_path = vulkan_lib_dir.join("vulkan-1.lib");

    (!vulkan_lib_path.exists()).then(|| {
        panic!("Vulkan library not found at {:?}", vulkan_lib_path);
    });

    println!(
        "cargo:rustc-link-search={}",
        vulkan_lib_dir.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib={}", "vulkan-1");

    let vulkan_header_rel_path = PathBuf::from("vulkan").join("vulkan.h");
    let vulkan_header_path = vulkan_include_dir.join(vulkan_header_rel_path);

    (!vulkan_header_path.exists()).then(|| {
        panic!("Vulkan header not found at {:?}", vulkan_header_path);
    });

    let bindings = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-I", vulkan_include_dir.to_str().unwrap()])
        .clang_args(&["-L", vulkan_lib_dir.to_str().unwrap()])
        .header("./src/wrapper.h")
        .allowlist_recursively(false)
        .allowlist_file(".*vulkan.*")
        .allowlist_file(".*vk_video.*")
        .allowlist_type("Vk.*")
        .blocklist_item("PFN_vk.*")
        // .allowlist_type("PFN_vk.*")
        .allowlist_function("vk.*")
        .allowlist_var("VK.*")
        .generate_cstr(true)
        .derive_default(true)
        .derive_debug(true)
        .derive_partialeq(true)
        .impl_debug(true)
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
