// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fmt::{Display, Formatter};

pub const VK_KHR_VALIDATION_LAYER_NAME: &[u8] = b"VK_LAYER_KHRONOS_validation\0";

pub mod vk {
    use crate::*;
    use std::ffi::CStr;
    use std::ptr::{null, null_mut};

    #[repr(i32)]
    #[non_exhaustive]
    #[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
    pub enum Error {
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
        InvalidDrmFormatModifierPlaneLayoutExt =
            VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT,

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

    /*
       Loading
    */

    trait Proc {
        const NAME: &'static CStr;
        type SIGNATURE: From<fn()>;
    }

    // struct CreateInstanceProc;
    // impl Proc for CreateInstanceProc {
    //     const NAME: &'static CStr = c"vkCreateInstance";
    //     type SIGNATURE = unsafe extern "C" fn(
    //         pCreateInfo: *const VkInstanceCreateInfo,
    //         pAllocator: *const VkAllocationCallbacks,
    //         pInstance: *mut VkInstance,
    //     ) -> VkResult;
    // }

    #[inline]
    pub fn get_instance_proc_addr(
        proc: unsafe extern "C" fn(VkInstance, *const std::os::raw::c_char) -> PFN_vkVoidFunction,
        instance: VkInstance,
        name: &CStr,
    ) -> PFN_vkVoidFunction {
        unsafe { proc(instance, name.as_ptr()) }
    }

    #[inline]
    pub fn get_device_proc_addr(
        proc: unsafe extern "C" fn(VkDevice, *const std::os::raw::c_char) -> PFN_vkVoidFunction,
        device: VkDevice,
        name: &CStr,
    ) -> PFN_vkVoidFunction {
        unsafe { proc(device, name.as_ptr()) }
    }

    /*
       Instance
    */

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

    /*
       Physical Device
    */

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
    pub fn get_physical_device_features(
        proc: unsafe extern "C" fn(
            physicalDevice: VkPhysicalDevice,
            pFeatures: *mut VkPhysicalDeviceFeatures,
        ),
        handle: VkPhysicalDevice,
    ) -> VkPhysicalDeviceFeatures {
        let mut p: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };
        unsafe { proc(handle, &mut p) };
        p
    }

    /*
       Surface
    */

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

    /*
       Surface Capabilities
    */

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

    /*
       Device
    */

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

    /*
       Command Pool
    */

    #[inline]
    pub fn create_command_pool(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pCreateInfo: *const VkCommandPoolCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pCommandPool: *mut VkCommandPool,
        ) -> VkResult,
        device: VkDevice,
        create_info: &VkCommandPoolCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkCommandPool> {
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
        proc: unsafe extern "C" fn(
            device: VkDevice,
            command_pool: VkCommandPool,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        command_pool: VkCommandPool,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, command_pool, option_to_ptr(allocator)) }
    }

    /*
       Swapchain
    */

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
    pub fn get_swapchain_images_khr(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            swapchain: VkSwapchainKHR,
            pSwapchainImageCount: *mut u32,
            pSwapchainImages: *mut VkImage,
        ) -> VkResult,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
    ) -> vk::Result<Vec<VkImage>> {
        let mut count = 0;
        to_result(unsafe { proc(device, swapchain, &mut count, null_mut()) })?;

        let mut images = Vec::<VkImage>::new();
        images.resize(count as usize, null_mut());

        to_result(unsafe { proc(device, swapchain, &mut count, images.as_mut_ptr()) })
            .map(|_| images)
    }

    #[inline]
    pub fn create_shader_module(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pCreateInfo: *const VkShaderModuleCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pShaderModule: *mut VkShaderModule,
        ) -> VkResult,
        device: VkDevice,
        create_info: &VkShaderModuleCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkShaderModule> {
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
        proc: unsafe extern "C" fn(
            device: VkDevice,
            shader_module: VkShaderModule,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        shader_module: VkShaderModule,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, shader_module, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn create_pipeline_layout(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pCreateInfo: *const VkPipelineLayoutCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pPipelineLayout: *mut VkPipelineLayout,
        ) -> VkResult,
        device: VkDevice,
        create_info: &VkPipelineLayoutCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkPipelineLayout> {
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
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pipeline_layout: VkPipelineLayout,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        pipeline_layout: VkPipelineLayout,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, pipeline_layout, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn create_render_pass(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pCreateInfo: *const VkRenderPassCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pRenderPass: *mut VkRenderPass,
        ) -> VkResult,
        device: VkDevice,
        create_info: &VkRenderPassCreateInfo,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkRenderPass> {
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
        proc: unsafe extern "C" fn(
            device: VkDevice,
            render_pass: VkRenderPass,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        render_pass: VkRenderPass,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, render_pass, option_to_ptr(allocator)) }
    }

    #[inline]
    pub fn create_graphics_pipelines(
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pipelineCache: VkPipelineCache,
            createInfoCount: u32,
            pCreateInfos: *const VkGraphicsPipelineCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pPipelines: *mut VkPipeline,
        ) -> VkResult,
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_infos: &[VkGraphicsPipelineCreateInfo],
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<Vec<VkPipeline>> {
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
        proc: unsafe extern "C" fn(
            device: VkDevice,
            pipeline: VkPipeline,
            pAllocator: *const VkAllocationCallbacks,
        ),
        device: VkDevice,
        pipeline: VkPipeline,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(device, pipeline, option_to_ptr(allocator)) }
    }

    /*
       Debug Utils
    */

    #[inline]
    pub fn create_debug_utils_messenger_ext(
        proc: unsafe extern "C" fn(
            instance: VkInstance,
            pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
            pAllocator: *const VkAllocationCallbacks,
            pMessenger: *mut VkDebugUtilsMessengerEXT,
        ) -> VkResult,
        instance: VkInstance,
        create_info: &VkDebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&VkAllocationCallbacks>,
    ) -> vk::Result<VkDebugUtilsMessengerEXT> {
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

    pub fn destroy_debug_utils_messenger_ext(
        proc: unsafe extern "C" fn(
            instance: VkInstance,
            messenger: VkDebugUtilsMessengerEXT,
            pAllocator: *const VkAllocationCallbacks,
        ),
        instance: VkInstance,
        messenger: VkDebugUtilsMessengerEXT,
        allocator: Option<&VkAllocationCallbacks>,
    ) {
        unsafe { proc(instance, messenger, option_to_ptr(allocator)) }
    }
}

impl Display for VkObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name: &str = match self {
            VkObjectType::VK_OBJECT_TYPE_UNKNOWN => "Unknown",
            VkObjectType::VK_OBJECT_TYPE_INSTANCE => "VkInstance",
            VkObjectType::VK_OBJECT_TYPE_PHYSICAL_DEVICE => "VkPhysicalDevice",
            VkObjectType::VK_OBJECT_TYPE_DEVICE => "VkDevice",
            VkObjectType::VK_OBJECT_TYPE_QUEUE => "VkQueue",
            VkObjectType::VK_OBJECT_TYPE_SEMAPHORE => "VkSemaphore",
            VkObjectType::VK_OBJECT_TYPE_COMMAND_BUFFER => "VkCommandBuffer",
            VkObjectType::VK_OBJECT_TYPE_FENCE => "VkFence",
            VkObjectType::VK_OBJECT_TYPE_DEVICE_MEMORY => "VkDeviceMemory",
            VkObjectType::VK_OBJECT_TYPE_BUFFER => "VkBuffer",
            VkObjectType::VK_OBJECT_TYPE_IMAGE => "VkImage",
            VkObjectType::VK_OBJECT_TYPE_EVENT => "VkEvent",
            VkObjectType::VK_OBJECT_TYPE_QUERY_POOL => "VkQueryPool",
            VkObjectType::VK_OBJECT_TYPE_BUFFER_VIEW => "VkBufferView",
            VkObjectType::VK_OBJECT_TYPE_IMAGE_VIEW => "VkImageView",
            VkObjectType::VK_OBJECT_TYPE_SHADER_MODULE => "VkShaderModule",
            VkObjectType::VK_OBJECT_TYPE_PIPELINE_CACHE => "VkPipelineCache",
            VkObjectType::VK_OBJECT_TYPE_PIPELINE_LAYOUT => "VkPipelineLayout",
            VkObjectType::VK_OBJECT_TYPE_RENDER_PASS => "VkRenderPass",
            VkObjectType::VK_OBJECT_TYPE_PIPELINE => "VkPipeline",
            VkObjectType::VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT => "VkDescriptorSetLayout",
            VkObjectType::VK_OBJECT_TYPE_SAMPLER => "VkSampler",
            VkObjectType::VK_OBJECT_TYPE_DESCRIPTOR_POOL => "VkDescriptorPool",
            VkObjectType::VK_OBJECT_TYPE_DESCRIPTOR_SET => "VkDescriptorSet",
            VkObjectType::VK_OBJECT_TYPE_FRAMEBUFFER => "VkFramebuffer",
            VkObjectType::VK_OBJECT_TYPE_COMMAND_POOL => "VkCommandPool",
            VkObjectType::VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION => "VkSamplerYcbcrConversionKHR",
            VkObjectType::VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE => "VkDescriptorUpdateTemplate",
            VkObjectType::VK_OBJECT_TYPE_PRIVATE_DATA_SLOT => "VkPrivateDataSlot",
            VkObjectType::VK_OBJECT_TYPE_SURFACE_KHR => "VkSurfaceKHR",
            VkObjectType::VK_OBJECT_TYPE_SWAPCHAIN_KHR => "VkSwapchainKHR",
            VkObjectType::VK_OBJECT_TYPE_DISPLAY_KHR => "VkDisplayKHR",
            VkObjectType::VK_OBJECT_TYPE_DISPLAY_MODE_KHR => "VkDisplayModeKHR",
            VkObjectType::VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT => "VkDebugReportCallbackEXT",
            VkObjectType::VK_OBJECT_TYPE_VIDEO_SESSION_KHR => "VkVideoSessionKHR",
            VkObjectType::VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR => {
                "VkVideoSessionParametersKHR"
            }
            VkObjectType::VK_OBJECT_TYPE_CU_MODULE_NVX => "VkCuModuleNVX",
            VkObjectType::VK_OBJECT_TYPE_CU_FUNCTION_NVX => "VkCuFunctionNVX",
            VkObjectType::VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT => "VkDebugUtilsMessengerEXT",
            VkObjectType::VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR => "VkAccelerationStructureKHR",
            VkObjectType::VK_OBJECT_TYPE_VALIDATION_CACHE_EXT => "VkValidationCacheEXT",
            VkObjectType::VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV => "VkAccelerationStructureNV",
            VkObjectType::VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL => {
                "VkPerformanceConfigurationINTEL"
            }
            VkObjectType::VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR => "VkDeferredOperationKHR",
            VkObjectType::VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV => {
                "VkIndirectCommandsLayoutNV"
            }
            VkObjectType::VK_OBJECT_TYPE_CUDA_MODULE_NV => "VkCudaModuleNV",
            VkObjectType::VK_OBJECT_TYPE_CUDA_FUNCTION_NV => "VkCudaFunctionNV",
            VkObjectType::VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA => "BufferCollectionFUCHSIA",
            VkObjectType::VK_OBJECT_TYPE_MICROMAP_EXT => "VkMicromapEXT",
            VkObjectType::VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV => "VkOpticalFlowSessionNV",
            VkObjectType::VK_OBJECT_TYPE_SHADER_EXT => "VkShaderEXT",
            _ => {
                log::warn!("Unknown VkObjectType: {:#?}", self);
                "Unknown"
            }
        };

        write!(f, "{}", name)
    }
}

unsafe impl Sync for VkPipelineViewportStateCreateInfo {}
unsafe impl Sync for VkPipelineMultisampleStateCreateInfo {}
unsafe impl Sync for VkPipelineDepthStencilStateCreateInfo {}
unsafe impl Sync for VkPipelineColorBlendStateCreateInfo {}
unsafe impl Sync for VkPipelineDynamicStateCreateInfo {}
