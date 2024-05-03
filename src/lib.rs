// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

mod bindings;
pub use bindings::*;

/// Rustified "safe" wrappers for vulkan functions
pub mod wrapper;

pub const VK_KHR_VALIDATION_LAYER_NAME: &[u8] = b"VK_LAYER_KHRONOS_validation\0";

impl Clone for VkWin32SurfaceCreateInfoKHR {
    fn clone(&self) -> Self {
        Self {
            sType: self.sType,
            pNext: self.pNext,
            flags: self.flags,
            hinstance: self.hinstance,
            hwnd: self.hwnd,
        }
    }
}
