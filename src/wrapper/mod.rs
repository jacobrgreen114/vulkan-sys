// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

use crate::*;
use std::ffi::CStr;
use std::fmt::Pointer;
use std::ptr::{null, null_mut};

#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("NotReady")]
    NotReady = VK_NOT_READY,
    #[error("Timeout")]
    Timeout = VK_TIMEOUT,
    #[error("EventSet")]
    EventSet = VK_EVENT_SET,
    #[error("EventReset")]
    EventReset = VK_EVENT_RESET,
    #[error("Incomplete")]
    Incomplete = VK_INCOMPLETE,

    #[error("OutOfHostMemory")]
    OutOfHostMemory = VK_ERROR_OUT_OF_HOST_MEMORY,
    #[error("OutOfDeviceMemory")]
    OutOfDeviceMemory = VK_ERROR_OUT_OF_DEVICE_MEMORY,
    #[error("InitializationFailed")]
    InitializationFailed = VK_ERROR_INITIALIZATION_FAILED,
    #[error("DeviceLost")]
    DeviceLost = VK_ERROR_DEVICE_LOST,
    #[error("MemoryMapFailed")]
    MemoryMapFailed = VK_ERROR_MEMORY_MAP_FAILED,
    #[error("LayerNotPresent")]
    LayerNotPresent = VK_ERROR_LAYER_NOT_PRESENT,
    #[error("ExtensionNotPresent")]
    ExtensionNotPresent = VK_ERROR_EXTENSION_NOT_PRESENT,
    #[error("FeatureNotPresent")]
    FeatureNotPresent = VK_ERROR_FEATURE_NOT_PRESENT,
    #[error("IncompatibleDriver")]
    IncompatibleDriver = VK_ERROR_INCOMPATIBLE_DRIVER,
    #[error("TooManyObjects")]
    TooManyObjects = VK_ERROR_TOO_MANY_OBJECTS,
    #[error("FormatNotSupported")]
    FormatNotSupported = VK_ERROR_FORMAT_NOT_SUPPORTED,
    #[error("FragmentedPool")]
    FragmentedPool = VK_ERROR_FRAGMENTED_POOL,
    #[error("Unknown")]
    Unknown = VK_ERROR_UNKNOWN,
    #[error("OutOfPoolMemory")]
    OutOfPoolMemory = VK_ERROR_OUT_OF_POOL_MEMORY,
    #[error("InvalidExternalHandle")]
    InvalidExternalHandle = VK_ERROR_INVALID_EXTERNAL_HANDLE,
    #[error("Fragmentation")]
    Fragmentation = VK_ERROR_FRAGMENTATION,
    #[error("InvalidOpaqueCaptureAddress")]
    InvalidOpaqueCaptureAddress = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,

    #[error("SurfaceLostKhr")]
    SurfaceLostKhr = VK_ERROR_SURFACE_LOST_KHR,
    #[error("NativeWindowInUseKhr")]
    NativeWindowInUseKhr = VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
    #[error("OutOfDateKhr")]
    OutOfDateKhr = VK_ERROR_OUT_OF_DATE_KHR,
    #[error("IncompatibleDisplayKhr")]
    IncompatibleDisplayKhr = VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
    #[error("ValidationFailedExt")]
    ValidationFailedExt = VK_ERROR_VALIDATION_FAILED_EXT,
    #[error("InvalidShaderNv")]
    InvalidShaderNv = VK_ERROR_INVALID_SHADER_NV,
    #[error("ImageUsageNotSupportedKhr")]
    ImageUsageNotSupportedKhr = VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,

    #[error("VideoPictureLayoutNotSupportedKhr")]
    VideoPictureLayoutNotSupportedKhr = VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
    #[error("VideoProfileOperationNotSupportedKhr")]
    VideoProfileOperationNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
    #[error("VideoProfileFormatNotSupportedKhr")]
    VideoProfileFormatNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
    #[error("VideoProfileCodecNotSupportedKhr")]
    VideoProfileCodecNotSupportedKhr = VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
    #[error("VideoStdVersionNotSupportedKhr")]
    VideoStdVersionNotSupportedKhr = VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR,
    #[error("InvalidDrmFormatModifierPlaneLayoutExt")]
    InvalidDrmFormatModifierPlaneLayoutExt = VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT,

    #[error("NotPermittedKhr")]
    NotPermittedKhr = VK_ERROR_NOT_PERMITTED_KHR,
    #[error("FullScreenExclusiveModeLostExt")]
    FullScreenExclusiveModeLostExt = VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,

    #[error("InvalidVideoStdVersionKhr")]
    InvalidVideoStdParametersKhr = VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
    #[error("CompressionExhaustedExt")]
    CompressionExhaustedExt = VK_ERROR_COMPRESSION_EXHAUSTED_EXT,
    #[error("IncompatibleShaderBinaryExt")]
    IncompatibleShaderBinaryExt = VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT,
}

pub type Result<T> = std::result::Result<T, Error>;

#[inline(always)]
fn to_result(result: VkResult) -> wrapper::Result<()> {
    (result == VK_SUCCESS)
        .then(|| ())
        .ok_or_else(|| unsafe { std::mem::transmute(result) })
}

#[inline(always)]
fn option_to_ptr<T>(allocator: Option<&T>) -> *const T {
    allocator.map(|r| r as *const _).unwrap_or(null())
}

/*
   Loading
*/

pub trait InstanceProc {
    const NAME: &'static CStr;
    type SIGNATURE: Pointer;
}

macro_rules! get_proc_addr {
    ($loader:expr, $obj:expr, $proc:ty) => {
        std::mem::transmute::<_, <$proc as crate::wrapper::InstanceProc>::SIGNATURE>($loader(
            $obj,
            <$proc as crate::wrapper::InstanceProc>::NAME.as_ptr(),
        ))
    };
}

macro_rules! define_instance_proc {
    ($name:ident, $ty:ident, $symbol:expr) => {
        pub struct $name;
        impl InstanceProc for $name {
            const NAME: &'static CStr = $symbol;
            type SIGNATURE = $ty;
        }
        impl $name {
            pub fn load(loader: PFN_vkGetInstanceProcAddr, instance: VkInstance) -> Option<$ty> {
                unsafe { std::mem::transmute(get_proc_addr!(loader, instance, Self)) }
            }
        }
    };
}

macro_rules! define_device_proc {
    ($name:ident, $ty:ident, $symbol:expr) => {
        pub struct $name;
        impl InstanceProc for $name {
            const NAME: &'static CStr = $symbol;
            type SIGNATURE = $ty;
        }
        impl $name {
            pub fn load(loader: PFN_vkGetDeviceProcAddr, device: VkDevice) -> Option<$ty> {
                unsafe { std::mem::transmute(get_proc_addr!(loader, device, Self)) }
            }
        }
    };
}

define_instance_proc!(CreateInstance, PFN_vkCreateInstance, c"vkCreateInstance");
define_instance_proc!(DestroyInstance, PFN_vkDestroyInstance, c"vkDestroyInstance");

define_instance_proc!(
    CreateDebugUtilsMessengerEXT,
    PFN_vkCreateDebugUtilsMessengerEXT,
    c"vkCreateDebugUtilsMessengerEXT"
);
define_instance_proc!(
    DestroyDebugUtilsMessengerEXT,
    PFN_vkDestroyDebugUtilsMessengerEXT,
    c"vkDestroyDebugUtilsMessengerEXT"
);

define_instance_proc!(
    EnumeratePhysicalDevices,
    PFN_vkEnumeratePhysicalDevices,
    c"vkEnumeratePhysicalDevices"
);
define_instance_proc!(
    GetPhysicalDeviceProperties,
    PFN_vkGetPhysicalDeviceProperties,
    c"vkGetPhysicalDeviceProperties"
);
define_instance_proc!(
    GetPhysicalDeviceFeatures,
    PFN_vkGetPhysicalDeviceFeatures,
    c"vkGetPhysicalDeviceFeatures"
);

#[cfg(target_os = "windows")]
define_instance_proc!(
    CreateWin32SurfaceKHR,
    PFN_vkCreateWin32SurfaceKHR,
    c"vkCreateWin32SurfaceKHR"
);

define_instance_proc!(
    DestroySurfaceKHR,
    PFN_vkDestroySurfaceKHR,
    c"vkDestroySurfaceKHR"
);
define_instance_proc!(
    GetPhysicalDeviceSurfaceSupportKHR,
    PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    c"vkGetPhysicalDeviceSurfaceSupportKHR"
);
define_instance_proc!(
    GetPhysicalDeviceSurfaceCapabilitiesKHR,
    PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR"
);
define_instance_proc!(
    GetPhysicalDeviceSurfaceFormatsKHR,
    PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    c"vkGetPhysicalDeviceSurfaceFormatsKHR"
);
define_instance_proc!(
    GetPhysicalDeviceSurfacePresentModesKHR,
    PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    c"vkGetPhysicalDeviceSurfacePresentModesKHR"
);

define_instance_proc!(CreateDevice, PFN_vkCreateDevice, c"vkCreateDevice");
define_device_proc!(DestroyDevice, PFN_vkDestroyDevice, c"vkDestroyDevice");

define_device_proc!(GetDeviceQueue, PFN_vkGetDeviceQueue, c"vkGetDeviceQueue");
define_device_proc!(QueueSubmit, PFN_vkQueueSubmit, c"vkQueueSubmit");
define_device_proc!(QueuePresentKHR, PFN_vkQueuePresentKHR, c"vkQueuePresentKHR");
define_device_proc!(QueueWaitIdle, PFN_vkQueueWaitIdle, c"vkQueueWaitIdle");

define_device_proc!(
    CreateSwapchainKHR,
    PFN_vkCreateSwapchainKHR,
    c"vkCreateSwapchainKHR"
);
define_device_proc!(
    DestroySwapchainKHR,
    PFN_vkDestroySwapchainKHR,
    c"vkDestroySwapchainKHR"
);
define_device_proc!(
    GetSwapchainImagesKHR,
    PFN_vkGetSwapchainImagesKHR,
    c"vkGetSwapchainImagesKHR"
);
define_device_proc!(
    AcquireNextImageKHR,
    PFN_vkAcquireNextImageKHR,
    c"vkAcquireNextImageKHR"
);

define_device_proc!(CreateImageView, PFN_vkCreateImageView, c"vkCreateImageView");
define_device_proc!(
    DestroyImageView,
    PFN_vkDestroyImageView,
    c"vkDestroyImageView"
);

define_device_proc!(
    CreateShaderModule,
    PFN_vkCreateShaderModule,
    c"vkCreateShaderModule"
);
define_device_proc!(
    DestroyShaderModule,
    PFN_vkDestroyShaderModule,
    c"vkDestroyShaderModule"
);

define_device_proc!(
    CreatePipelineLayout,
    PFN_vkCreatePipelineLayout,
    c"vkCreatePipelineLayout"
);
define_device_proc!(
    DestroyPipelineLayout,
    PFN_vkDestroyPipelineLayout,
    c"vkDestroyPipelineLayout"
);

define_device_proc!(
    CreateRenderPass,
    PFN_vkCreateRenderPass,
    c"vkCreateRenderPass"
);
define_device_proc!(
    DestroyRenderPass,
    PFN_vkDestroyRenderPass,
    c"vkDestroyRenderPass"
);

define_device_proc!(
    CreateFramebuffer,
    PFN_vkCreateFramebuffer,
    c"vkCreateFramebuffer"
);
define_device_proc!(
    DestroyFramebuffer,
    PFN_vkDestroyFramebuffer,
    c"vkDestroyFramebuffer"
);

define_device_proc!(
    CreateGraphicsPipelines,
    PFN_vkCreateGraphicsPipelines,
    c"vkCreateGraphicsPipelines"
);

define_device_proc!(DestroyPipeline, PFN_vkDestroyPipeline, c"vkDestroyPipeline");

define_device_proc!(CreateFence, PFN_vkCreateFence, c"vkCreateFence");
define_device_proc!(DestroyFence, PFN_vkDestroyFence, c"vkDestroyFence");
define_device_proc!(WaitForFences, PFN_vkWaitForFences, c"vkWaitForFences");
define_device_proc!(ResetFences, PFN_vkResetFences, c"vkResetFences");

define_device_proc!(CreateSemaphore, PFN_vkCreateSemaphore, c"vkCreateSemaphore");
define_device_proc!(
    DestroySemaphore,
    PFN_vkDestroySemaphore,
    c"vkDestroySemaphore"
);

define_device_proc!(
    CreateCommandPool,
    PFN_vkCreateCommandPool,
    c"vkCreateCommandPool"
);
define_device_proc!(
    DestroyCommandPool,
    PFN_vkDestroyCommandPool,
    c"vkDestroyCommandPool"
);

define_device_proc!(
    AllocateCommandBuffers,
    PFN_vkAllocateCommandBuffers,
    c"vkAllocateCommandBuffers"
);
define_device_proc!(
    FreeCommandBuffers,
    PFN_vkFreeCommandBuffers,
    c"vkFreeCommandBuffers"
);
define_device_proc!(
    BeginCommandBuffer,
    PFN_vkBeginCommandBuffer,
    c"vkBeginCommandBuffer"
);
define_device_proc!(
    EndCommandBuffer,
    PFN_vkEndCommandBuffer,
    c"vkEndCommandBuffer"
);
define_device_proc!(
    ResetCommandBuffer,
    PFN_vkResetCommandBuffer,
    c"vkResetCommandBuffer"
);
define_device_proc!(
    CmdBeginRenderPass,
    PFN_vkCmdBeginRenderPass,
    c"vkCmdBeginRenderPass"
);
define_device_proc!(
    CmdEndRenderPass,
    PFN_vkCmdEndRenderPass,
    c"vkCmdEndRenderPass"
);
define_device_proc!(CmdBindPipeline, PFN_vkCmdBindPipeline, c"vkCmdBindPipeline");
define_device_proc!(CmdSetViewport, PFN_vkCmdSetViewport, c"vkCmdSetViewport");
define_device_proc!(CmdSetScissor, PFN_vkCmdSetScissor, c"vkCmdSetScissor");
define_device_proc!(
    CmdBindVertexBuffers,
    PFN_vkCmdBindVertexBuffers,
    c"vkCmdBindVertexBuffers"
);
define_device_proc!(
    CmdBindIndexBuffer,
    PFN_vkCmdBindIndexBuffer,
    c"vkCmdBindIndexBuffer"
);
define_device_proc!(CmdDraw, PFN_vkCmdDraw, c"vkCmdDraw");
define_device_proc!(CmdDrawIndexed, PFN_vkCmdDrawIndexed, c"vkCmdDrawIndexed");
define_device_proc!(CmdDrawIndirect, PFN_vkCmdDrawIndirect, c"vkCmdDrawIndirect");
define_device_proc!(
    CmdDrawIndexedIndirect,
    PFN_vkCmdDrawIndexedIndirect,
    c"vkCmdDrawIndexedIndirect"
);
define_device_proc!(CmdDispatch, PFN_vkCmdDispatch, c"vkCmdDispatch");

define_device_proc!(
    CmdPipelineBarrier,
    PFN_vkCmdPipelineBarrier,
    c"vkCmdPipelineBarrier"
);
define_device_proc!(
    CmdClearColorImage,
    PFN_vkCmdClearColorImage,
    c"vkCmdClearColorImage"
);
define_device_proc!(
    CmdClearDepthStencilImage,
    PFN_vkCmdClearDepthStencilImage,
    c"vkCmdClearDepthStencilImage"
);
define_device_proc!(CmdCopyBuffer, PFN_vkCmdCopyBuffer, c"vkCmdCopyBuffer");
define_device_proc!(CmdCopyImage, PFN_vkCmdCopyImage, c"vkCmdCopyImage");
define_device_proc!(
    CmdCopyBufferToImage,
    PFN_vkCmdCopyBufferToImage,
    c"vkCmdCopyBufferToImage"
);
define_device_proc!(
    CmdCopyImageToBuffer,
    PFN_vkCmdCopyImageToBuffer,
    c"vkCmdCopyImageToBuffer"
);
define_device_proc!(CmdBlitImage, PFN_vkCmdBlitImage, c"vkCmdBlitImage");
define_device_proc!(CmdResolveImage, PFN_vkCmdResolveImage, c"vkCmdResolveImage");
define_device_proc!(CmdBeginQuery, PFN_vkCmdBeginQuery, c"vkCmdBeginQuery");
define_device_proc!(CmdEndQuery, PFN_vkCmdEndQuery, c"vkCmdEndQuery");
define_device_proc!(
    CmdResetQueryPool,
    PFN_vkCmdResetQueryPool,
    c"vkCmdResetQueryPool"
);
define_device_proc!(
    CmdWriteTimestamp,
    PFN_vkCmdWriteTimestamp,
    c"vkCmdWriteTimestamp"
);

/*
   Instance
*/

#[inline]
pub fn create_instance(
    proc: PFN_vkCreateInstance,
    create_info: &VkInstanceCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkInstance> {
    let mut instance = null_mut();
    to_result(unsafe { proc(create_info, option_to_ptr(allocator), &mut instance) })
        .map(|_| instance)
}

#[inline]
pub fn destroy_instance(
    proc: PFN_vkDestroyInstance,
    instance: VkInstance,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(instance, option_to_ptr(allocator)) }
}

/*
   Physical Device
*/

#[inline]
pub fn enumerate_physical_devices(
    proc: PFN_vkEnumeratePhysicalDevices,
    handle: VkInstance,
) -> wrapper::Result<Vec<VkPhysicalDevice>> {
    let mut count = 0;
    to_result(unsafe { proc(handle, &mut count, null_mut()) })?;

    let mut physical_devices = Vec::<VkPhysicalDevice>::new();
    physical_devices.resize(count as usize, null_mut());

    to_result(unsafe { proc(handle, &mut count, physical_devices.as_mut_ptr()) })
        .map(|_| physical_devices)
}

#[inline]
pub fn get_physical_device_properties(
    proc: PFN_vkGetPhysicalDeviceProperties,
    handle: VkPhysicalDevice,
) -> VkPhysicalDeviceProperties {
    let mut p: VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
    unsafe { proc(handle, &mut p) };
    p
}

#[inline]
pub fn get_physical_device_features(
    proc: PFN_vkGetPhysicalDeviceFeatures,
    handle: VkPhysicalDevice,
) -> VkPhysicalDeviceFeatures {
    let mut p: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };
    unsafe { proc(handle, &mut p) };
    p
}

#[inline]
pub fn get_physical_device_queue_family_properties(
    proc: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    handle: VkPhysicalDevice,
) -> Vec<VkQueueFamilyProperties> {
    unsafe {
        let mut count = 0;
        proc(handle, &mut count, null_mut());

        let mut properties = Vec::<VkQueueFamilyProperties>::with_capacity(count as usize);

        proc(handle, &mut count, properties.as_mut_ptr());
        properties.set_len(count as usize);

        properties
    }
}

#[inline]
pub fn get_physical_device_memory_properties(
    proc: PFN_vkGetPhysicalDeviceMemoryProperties,
    handle: VkPhysicalDevice,
) -> VkPhysicalDeviceMemoryProperties {
    let mut p: VkPhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
    unsafe { proc(handle, &mut p) };
    p
}

/*
   Surface
*/

#[inline]
#[cfg(target_os = "windows")]
pub fn create_win32_surface_khr(
    proc: PFN_vkCreateWin32SurfaceKHR,
    root: VkInstance,
    create_info: &VkWin32SurfaceCreateInfoKHR,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkSurfaceKHR> {
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
    proc: PFN_vkDestroySurfaceKHR,
    instance: VkInstance,
    surface: VkSurfaceKHR,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(instance, surface, option_to_ptr(allocator)) }
}

/*
   Surface Capabilities
*/

#[inline]
pub fn get_physical_device_surface_support_khr(
    proc: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
) -> wrapper::Result<bool> {
    let mut supported = VK_FALSE;
    to_result(unsafe { proc(physical_device, queue_family_index, surface, &mut supported) })
        .map(|_| supported != VK_FALSE)
}

#[inline]
pub fn get_physical_device_surface_capabilities_khr(
    proc: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<VkSurfaceCapabilitiesKHR> {
    let mut capabilities = unsafe { std::mem::zeroed() };
    to_result(unsafe { proc(physical_device, surface, &mut capabilities) }).map(|_| capabilities)
}

#[inline]
pub fn get_physical_device_surface_formats_khr(
    proc: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
) -> Result<Vec<VkSurfaceFormatKHR>> {
    unsafe {
        let mut count = 0;
        to_result(proc(physical_device, surface, &mut count, null_mut()))?;

        let mut surface_formats: Vec<VkSurfaceFormatKHR> = Vec::with_capacity(count as usize);
        // surface_formats.resize(count as usize, unsafe { std::mem::zeroed() });

        to_result(proc(
            physical_device,
            surface,
            &mut count,
            surface_formats.as_mut_ptr(),
        ))
        .map(|_| {
            surface_formats.set_len(count as usize);
            surface_formats
        })
    }
}

#[inline]
pub fn get_physical_device_surface_present_modes_khr(
    proc: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
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

/*
   Device
*/

#[inline]
pub fn create_device(
    proc: PFN_vkCreateDevice,
    physical_device: VkPhysicalDevice,
    create_info: &VkDeviceCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkDevice> {
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
    proc: PFN_vkDestroyDevice,
    device: VkDevice,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, option_to_ptr(allocator)) };
}

/*
   Queue
*/

#[inline]
pub fn get_device_queue(
    proc: PFN_vkGetDeviceQueue,
    device: VkDevice,
    family_index: u32,
    queue_index: u32,
) -> VkQueue {
    let mut queue = null_mut();
    unsafe { proc(device, family_index, queue_index, &mut queue) };
    queue
}

pub fn queue_submit(
    proc: PFN_vkQueueSubmit,
    queue: VkQueue,
    submits: &[VkSubmitInfo],
    fence: Option<VkFence>,
) -> wrapper::Result<()> {
    to_result(unsafe {
        proc(
            queue,
            submits.len() as u32,
            submits.as_ptr(),
            fence.unwrap_or(std::ptr::null_mut()),
        )
    })
}

pub fn queue_present_khr(
    proc: PFN_vkQueuePresentKHR,
    queue: VkQueue,
    present_info: &VkPresentInfoKHR,
) -> wrapper::Result<()> {
    to_result(unsafe { proc(queue, present_info) })
}

pub fn queue_wait_idle(proc: PFN_vkQueueWaitIdle, queue: VkQueue) -> wrapper::Result<()> {
    to_result(unsafe { proc(queue) })
}

/*
   Swapchain
*/

#[inline]
pub fn create_swapchain_khr(
    proc: PFN_vkCreateSwapchainKHR,
    device: VkDevice,
    create_info: &VkSwapchainCreateInfoKHR,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkSwapchainKHR> {
    let mut surface = null_mut();
    to_result(unsafe { proc(device, create_info, option_to_ptr(allocator), &mut surface) })
        .map(|_| surface)
}

#[inline]
pub fn destroy_swapchain_khr(
    proc: PFN_vkDestroySwapchainKHR,
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, swapchain, option_to_ptr(allocator)) }
}

#[inline]
pub fn get_swapchain_images_khr(
    proc: PFN_vkGetSwapchainImagesKHR,
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> wrapper::Result<Vec<VkImage>> {
    let mut count = 0;
    to_result(unsafe { proc(device, swapchain, &mut count, null_mut()) })?;

    let mut images = Vec::<VkImage>::new();
    images.resize(count as usize, null_mut());

    to_result(unsafe { proc(device, swapchain, &mut count, images.as_mut_ptr()) }).map(|_| images)
}

#[inline]
pub fn acquire_next_image_khr(
    proc: PFN_vkAcquireNextImageKHR,
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: Option<VkSemaphore>,
    fence: Option<VkFence>,
) -> wrapper::Result<u32> {
    let mut image_index = 0;
    to_result(unsafe {
        proc(
            device,
            swapchain,
            timeout,
            semaphore.unwrap_or(null_mut()),
            fence.unwrap_or(null_mut()),
            &mut image_index,
        )
    })
    .map(|_| image_index)
}

/*
   Image
*/

/*
   Image View
*/

#[inline]
pub fn create_image_view(
    proc: PFN_vkCreateImageView,
    device: VkDevice,
    create_info: &VkImageViewCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkImageView> {
    let mut image_view = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut image_view,
        )
    })
    .map(|_| image_view)
}

#[inline]
pub fn destroy_image_view(
    proc: PFN_vkDestroyImageView,
    device: VkDevice,
    image_view: VkImageView,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, image_view, option_to_ptr(allocator)) }
}

/*
    Render Pass
*/

#[inline]
pub fn create_render_pass(
    proc: PFN_vkCreateRenderPass,
    device: VkDevice,
    create_info: &VkRenderPassCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkRenderPass> {
    let mut render_pass = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut render_pass,
        )
    })
    .map(|_| render_pass)
}

#[inline]
pub fn destroy_render_pass(
    proc: PFN_vkDestroyRenderPass,
    device: VkDevice,
    render_pass: VkRenderPass,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, render_pass, option_to_ptr(allocator)) }
}

/*
   Framebuffer
*/

#[inline]
pub fn create_framebuffer(
    proc: PFN_vkCreateFramebuffer,
    device: VkDevice,
    create_info: &VkFramebufferCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkFramebuffer> {
    let mut framebuffer = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut framebuffer,
        )
    })
    .map(|_| framebuffer)
}

#[inline]
pub fn destroy_framebuffer(
    proc: PFN_vkDestroyFramebuffer,
    device: VkDevice,
    framebuffer: VkFramebuffer,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, framebuffer, option_to_ptr(allocator)) }
}

/*
   Descriptor Set Layout
*/

#[inline]
pub fn create_descriptor_set_layout(
    proc: PFN_vkCreateDescriptorSetLayout,
    device: VkDevice,
    create_info: &VkDescriptorSetLayoutCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkDescriptorSetLayout> {
    let mut descriptor_set_layout = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut descriptor_set_layout,
        )
    })
    .map(|_| descriptor_set_layout)
}

#[inline]
pub fn destroy_descriptor_set_layout(
    proc: PFN_vkDestroyDescriptorSetLayout,
    device: VkDevice,
    descriptor_set_layout: VkDescriptorSetLayout,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, descriptor_set_layout, option_to_ptr(allocator)) }
}

/*
    Pipeline Layout
*/

#[inline]
pub fn create_pipeline_layout(
    proc: PFN_vkCreatePipelineLayout,
    device: VkDevice,
    create_info: &VkPipelineLayoutCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkPipelineLayout> {
    let mut pipeline_layout = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut pipeline_layout,
        )
    })
    .map(|_| pipeline_layout)
}

#[inline]
pub fn destroy_pipeline_layout(
    proc: PFN_vkDestroyPipelineLayout,
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, pipeline_layout, option_to_ptr(allocator)) }
}

/*
   Shader Module
*/

#[inline]
pub fn create_shader_module(
    proc: PFN_vkCreateShaderModule,
    device: VkDevice,
    create_info: &VkShaderModuleCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkShaderModule> {
    let mut shader_module = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut shader_module,
        )
    })
    .map(|_| shader_module)
}

#[inline]
pub fn destroy_shader_module(
    proc: PFN_vkDestroyShaderModule,
    device: VkDevice,
    shader_module: VkShaderModule,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, shader_module, option_to_ptr(allocator)) }
}

/*
   Graphics Pipeline
*/

#[inline]
pub fn create_graphics_pipelines(
    proc: PFN_vkCreateGraphicsPipelines,
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_infos: &[VkGraphicsPipelineCreateInfo],
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<Vec<VkPipeline>> {
    let mut pipelines = Vec::new();
    pipelines.resize(create_infos.len(), null_mut());
    to_result(unsafe {
        proc(
            device,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            option_to_ptr(allocator),
            pipelines.as_mut_ptr(),
        )
    })
    .map(|_| pipelines)
}

#[inline]
pub fn destroy_pipeline(
    proc: PFN_vkDestroyPipeline,
    device: VkDevice,
    pipeline: VkPipeline,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, pipeline, option_to_ptr(allocator)) }
}

/*
   Fence
*/

#[inline]
pub fn create_fence(
    proc: PFN_vkCreateFence,
    device: VkDevice,
    create_info: &VkFenceCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkFence> {
    let mut fence = null_mut();
    to_result(unsafe { proc(device, create_info, option_to_ptr(allocator), &mut fence) })
        .map(|_| fence)
}

#[inline]
pub fn destroy_fence(
    proc: PFN_vkDestroyFence,
    device: VkDevice,
    fence: VkFence,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, fence, option_to_ptr(allocator)) }
}

#[inline]
pub fn wait_for_fences(
    proc: PFN_vkWaitForFences,
    device: VkDevice,
    fences: &[VkFence],
    wait_all: bool,
    timeout: u64,
) -> wrapper::Result<()> {
    to_result(unsafe {
        proc(
            device,
            fences.len() as u32,
            fences.as_ptr(),
            wait_all.into(),
            timeout,
        )
    })
}

#[inline]
pub fn reset_fences(
    proc: PFN_vkResetFences,
    device: VkDevice,
    fences: &[VkFence],
) -> wrapper::Result<()> {
    to_result(unsafe { proc(device, fences.len() as u32, fences.as_ptr()) })
}

/*
   Semaphore
*/

#[inline]
pub fn create_semaphore(
    proc: PFN_vkCreateSemaphore,
    device: VkDevice,
    create_info: &VkSemaphoreCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkSemaphore> {
    let mut semaphore = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut semaphore,
        )
    })
    .map(|_| semaphore)
}

#[inline]
pub fn destroy_semaphore(
    proc: PFN_vkDestroySemaphore,
    device: VkDevice,
    semaphore: VkSemaphore,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, semaphore, option_to_ptr(allocator)) }
}

/*
   Command Pool
*/

#[inline]
pub fn create_command_pool(
    proc: PFN_vkCreateCommandPool,
    device: VkDevice,
    create_info: &VkCommandPoolCreateInfo,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkCommandPool> {
    let mut command_pool = null_mut();
    to_result(unsafe {
        proc(
            device,
            create_info,
            option_to_ptr(allocator),
            &mut command_pool,
        )
    })
    .map(|_| command_pool)
}

#[inline]
pub fn destroy_command_pool(
    proc: PFN_vkDestroyCommandPool,
    device: VkDevice,
    command_pool: VkCommandPool,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(device, command_pool, option_to_ptr(allocator)) }
}

/*
   Command Buffer
*/

#[inline]
pub fn allocate_command_buffers(
    proc: PFN_vkAllocateCommandBuffers,
    device: VkDevice,
    allocate_info: &VkCommandBufferAllocateInfo,
) -> wrapper::Result<Vec<VkCommandBuffer>> {
    let mut command_buffers = Vec::new();
    command_buffers.resize(allocate_info.commandBufferCount as usize, null_mut());
    to_result(unsafe { proc(device, allocate_info, command_buffers.as_mut_ptr()) })
        .map(|_| command_buffers)
}

/*
   Command Buffer Record
*/

#[inline]
pub fn reset_command_buffer(
    proc: PFN_vkResetCommandBuffer,
    command_buffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> wrapper::Result<()> {
    to_result(unsafe { proc(command_buffer, flags) })
}

#[inline]
pub fn begin_command_buffer(
    proc: PFN_vkBeginCommandBuffer,
    command_buffer: VkCommandBuffer,
    begin_info: &VkCommandBufferBeginInfo,
) -> wrapper::Result<()> {
    to_result(unsafe { proc(command_buffer, begin_info) })
}

#[inline]
pub fn end_command_buffer(
    proc: PFN_vkEndCommandBuffer,
    command_buffer: VkCommandBuffer,
) -> wrapper::Result<()> {
    to_result(unsafe { proc(command_buffer) })
}

#[inline]
pub fn cmd_begin_render_pass(
    proc: PFN_vkCmdBeginRenderPass,
    command_buffer: VkCommandBuffer,
    render_pass_begin: &VkRenderPassBeginInfo,
    contents: VkSubpassContents,
) {
    unsafe { proc(command_buffer, render_pass_begin, contents) }
}

#[inline]
pub fn cmd_end_render_pass(proc: PFN_vkCmdEndRenderPass, command_buffer: VkCommandBuffer) {
    unsafe { proc(command_buffer) }
}

/*
   Debug Utils
*/

#[inline]
pub fn create_debug_utils_messenger_ext(
    proc: PFN_vkCreateDebugUtilsMessengerEXT,
    instance: VkInstance,
    create_info: &VkDebugUtilsMessengerCreateInfoEXT,
    allocator: Option<&VkAllocationCallbacks>,
) -> wrapper::Result<VkDebugUtilsMessengerEXT> {
    let mut messenger = null_mut();
    to_result(unsafe {
        proc(
            instance,
            create_info,
            option_to_ptr(allocator),
            &mut messenger,
        )
    })
    .map(|_| messenger)
}

#[inline]
pub fn destroy_debug_utils_messenger_ext(
    proc: PFN_vkDestroyDebugUtilsMessengerEXT,
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    allocator: Option<&VkAllocationCallbacks>,
) {
    unsafe { proc(instance, messenger, option_to_ptr(allocator)) }
}
