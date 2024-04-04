// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const VK_KHR_VALIDATION_LAYER_NAME: &[u8] = b"VK_LAYER_KHRONOS_validation\0";

pub mod vk {
    use crate::*;
    use std::ptr::{null, null_mut};

    #[repr(i32)]
    #[non_exhaustive]
    #[derive(Debug)]
    pub enum Error {
        OutOfHostMemory = VK_ERROR_OUT_OF_HOST_MEMORY,
        OutOfDeviceMemory = VK_ERROR_OUT_OF_DEVICE_MEMORY,
        InitializationFailed = VK_ERROR_INITIALIZATION_FAILED,
        DeviceLost = VK_ERROR_DEVICE_LOST,
        MemoryMapFailed = VK_ERROR_MEMORY_MAP_FAILED,
        LayerNotPresent = VK_ERROR_LAYER_NOT_PRESENT,
        ExtensionNotPresent = VK_ERROR_EXTENSION_NOT_PRESENT,
        FeatureNotPresent = VK_ERROR_FEATURE_NOT_PRESENT,
        IncompatibleDriver = VK_ERROR_INCOMPATIBLE_DRIVER,
        TooManyObjects = VK_ERROR_TOO_MANY_OBJECTS,
        FormatNotSupported = VK_ERROR_FORMAT_NOT_SUPPORTED,
        FragmentedPool = VK_ERROR_FRAGMENTED_POOL,
        Unknown = VK_ERROR_UNKNOWN,
        OutOfPoolMemory = VK_ERROR_OUT_OF_POOL_MEMORY,
        InvalidExternalHandle = VK_ERROR_INVALID_EXTERNAL_HANDLE,
        Fragmentation = VK_ERROR_FRAGMENTATION,
        InvalidOpaqueCaptureAddress = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,

        SurfaceLostKhr = VK_ERROR_SURFACE_LOST_KHR,
        NativeWindowInUseKhr = VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
        OutOfDateKhr = VK_ERROR_OUT_OF_DATE_KHR,
        IncompatibleDisplayKhr = VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
        ValidationFailedExt = VK_ERROR_VALIDATION_FAILED_EXT,
        InvalidShaderNv = VK_ERROR_INVALID_SHADER_NV,
        ImageUsageNotSupportedKhr = VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,

        VideoPictureLayoutNotSupportedKhr = VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
        VideoProfileOperationNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
        VideoProfileFormatNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
        VideoProfileCodecNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
        VideoStdVersionNotSupportedKhr = VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR,
        InvalidDrmFormatModifierPlaneLayoutExt =
            VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT,

        NotPermittedKhr = VK_ERROR_NOT_PERMITTED_KHR,
        FullScreenExclusiveModeLostExt = VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,

        InvalidVideoStdParametersKhr = VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
        CompressionExhaustedExt = VK_ERROR_COMPRESSION_EXHAUSTED_EXT,
        IncompatibleShaderBinaryExt = VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT,
    }

    pub type Result<T> = std::result::Result<T, Error>;

    #[inline(always)]
    fn to_result(result: VkResult) -> vk::Result<()> {
        if result >= 0 {
            Ok(())
        } else {
            unsafe { Err(std::mem::transmute(result)) }
        }
    }

    #[inline(always)]
    fn option_to_ptr<T>(allocator: Option<&T>) -> *const T {
        allocator.map(|r| r as *const _).unwrap_or(null())
    }

    #[inline]
    pub fn create_instance(
        proc: unsafe extern "C" fn(
            pCreateInfo: *const VkInstanceCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pInstance: *mut VkInstance,
        ) -> VkResult,
        create_info: &VkInstanceCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkInstance> {
        let mut instance = null_mut();
        to_result(unsafe { proc(create_info, option_to_ptr(allocator), &mut instance) })
            .map(|_| instance)
    }

    #[inline]
    pub fn destroy_instance(
        proc: unsafe extern "C" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks),
        instance: VkInstance,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(instance, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn create_device(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            pCreateInfo: *const VkDeviceCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pDevice: *mut VkDevice,
        ) -> VkResult,
        physical_device: VkPhysicalDevice,
        create_info: &VkDeviceCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkDevice> {
        let mut device = null_mut();
        to_result(unsafe {
            proc(
                physical_device,
                create_info,
                option_to_ptr(allocator),
                &mut device,
            )
        })
        .map(|_| device)
    }

    #[inline]
    pub fn destroy_device(
        proc: unsafe extern "C" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks),
        device: VkDevice,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, option_to_ptr(allocator)) };
    }

    #[inline]
    pub fn get_device_queue(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            queueFamilyIndex: u32,
            queueIndex: u32,
            pQueue: *mut VkQueue,
        ),
        device: VkDevice,
        family_index: u32,
        queue_index: u32,
    ) -> VkQueue {
        let mut queue = null_mut();
        unsafe { proc(device, family_index, queue_index, &mut queue) };
        queue
    }

    #[inline]
    pub fn enumerate_physical_device(
        proc: unsafe extern "C" fn(
            instance: VkInstance,
            pPhysicalDeviceCount: *mut u32,
            pPhysicalDevices: *mut VkPhysicalDevice,
        ) -> VkResult,
        handle: VkInstance,
    ) -> vk::Result<Vec<VkPhysicalDevice>> {
        let mut count = 0;
        to_result(unsafe { proc(handle, &mut count, null_mut()) })?;

        let mut physical_devices = Vec::<VkPhysicalDevice>::new();
        physical_devices.resize(count as usize, null_mut());

        to_result(unsafe { proc(handle, &mut count, physical_devices.as_mut_ptr()) })
            .map(|_| physical_devices)
    }

    #[inline]
    pub fn get_physical_device_properties(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            pProperties: *mut VkPhysicalDeviceProperties,
        ),
        handle: VkPhysicalDevice,
    ) -> VkPhysicalDeviceProperties {
        let mut p: VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
        unsafe { proc(handle, &mut p) };
        p
    }

    #[inline]
    #[cfg(target_os = "windows")]
    pub fn create_win32_surface_khr(
        proc: unsafe extern "C" fn(
            instance: VkInstance,
            pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
            pAllocator: *const VkAllocationCallbacks,
            pSurface: *mut VkSurfaceKHR,
        ) -> VkResult,
        root: VkInstance,
        create_info: &VkWin32SurfaceCreateInfoKHR,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkSurfaceKHR> {
        let mut surface = null_mut();
        to_result(unsafe {
            proc(
                root,
                create_info,
                allocator.map(|r| r as *const _).unwrap_or(null()),
                &mut surface,
            )
        })
        .map(|_| surface)
    }

    #[inline]
    pub fn destroy_surface_khr(
        proc: unsafe extern "C" fn(
            instance: VkInstance,
            surface: VkSurfaceKHR,
            pAllocator: *const VkAllocationCallbacks,
        ),
        instance: VkInstance,
        surface: VkSurfaceKHR,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(instance, surface, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn create_swapchain_khr(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pCreateInfo: *const VkSwapchainCreateInfoKHR,
            pAllocator: *const VkAllocationCallbacks,
            pSwapchain: *mut VkSwapchainKHR,
        ) -> VkResult,
        device: VkDevice,
        create_info: &VkSwapchainCreateInfoKHR,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkSwapchainKHR> {
        let mut surface = null_mut();
        to_result(unsafe { proc(device, create_info, option_to_ptr(allocator), &mut surface) })
            .map(|_| surface)
    }

    #[inline]
    pub fn destroy_swapchain_khr(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            swapchain: VkSwapchainKHR,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, swapchain, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn get_physical_device_surface_support_khr(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            queueFamilyIndex: u32,
            surface: VkSurfaceKHR,
            pSupported: *mut VkBool32,
        ) -> VkResult,
        physical_device: VkPhysicalDevice,
        queue_family_index: u32,
        surface: VkSurfaceKHR,
    ) -> vk::Result<bool> {
        let mut supported = VK_FALSE;
        to_result(unsafe { proc(physical_device, queue_family_index, surface, &mut supported) })
            .map(|_| supported != VK_FALSE)
    }

    #[inline]
    pub fn get_physical_device_surface_capabilities_khr(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            surface: VkSurfaceKHR,
            pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
        ) -> VkResult,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> Result<VkSurfaceCapabilitiesKHR> {
        let mut capabilities = unsafe { std::mem::zeroed() };
        to_result(unsafe { proc(physical_device, surface, &mut capabilities) })
            .map(|_| capabilities)
    }

    #[inline]
    pub fn get_physical_device_surface_formats_khr(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            surface: VkSurfaceKHR,
            pPresentModeCount: *mut u32,
            pPresentModes: *mut VkSurfaceFormatKHR,
        ) -> VkResult,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> Result<Vec<VkSurfaceFormatKHR>> {
        let mut count = 0;
        to_result(unsafe { proc(physical_device, surface, &mut count, null_mut()) })?;

        let mut surface_formats: Vec<VkSurfaceFormatKHR> = Vec::new();
        surface_formats.resize(count as usize, unsafe { std::mem::zeroed() });

        to_result(unsafe {
            proc(
                physical_device,
                surface,
                &mut count,
                surface_formats.as_mut_ptr(),
            )
        })
        .map(|_| surface_formats)
    }

    #[inline]
    pub fn get_physical_device_surface_present_modes_khr(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            surface: VkSurfaceKHR,
            pPresentModeCount: *mut u32,
            pPresentModes: *mut VkPresentModeKHR,
        ) -> VkResult,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> Result<Vec<VkPresentModeKHR>> {
        let mut count = 0;
        to_result(unsafe { proc(physical_device, surface, &mut count, null_mut()) })?;

        let mut present_modes = Vec::new();
        present_modes.resize(count as usize, 0);

        to_result(unsafe {
            proc(
                physical_device,
                surface,
                &mut count,
                present_modes.as_mut_ptr(),
            )
        })
        .map(|_| present_modes)
    }
}
