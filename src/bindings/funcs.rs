#![allow(nonstandard_style)]
#![allow(unused)]

use super::*;

pub type PFN_vkAllocationFunction = unsafe extern "C" fn(
    pUserData: *mut ::std::os::raw::c_void,
    size: usize,
    alignment: usize,
    allocationScope: VkSystemAllocationScope,
) -> *mut ::std::os::raw::c_void;
pub type PFN_vkFreeFunction = unsafe extern "C" fn(
    pUserData: *mut ::std::os::raw::c_void,
    pMemory: *mut ::std::os::raw::c_void,
);
pub type PFN_vkInternalAllocationNotification = unsafe extern "C" fn(
    pUserData: *mut ::std::os::raw::c_void,
    size: usize,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
);
pub type PFN_vkInternalFreeNotification = unsafe extern "C" fn(
    pUserData: *mut ::std::os::raw::c_void,
    size: usize,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
);
pub type PFN_vkReallocationFunction = unsafe extern "C" fn(
    pUserData: *mut ::std::os::raw::c_void,
    pOriginal: *mut ::std::os::raw::c_void,
    size: usize,
    alignment: usize,
    allocationScope: VkSystemAllocationScope,
) -> *mut ::std::os::raw::c_void;
pub type PFN_vkVoidFunction = unsafe extern "C" fn();

pub type PFN_vkCreateInstance = unsafe extern "C" fn(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult;
pub type PFN_vkDestroyInstance =
    unsafe extern "C" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: *mut VkImageFormatProperties,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties,
);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
);
pub type PFN_vkGetInstanceProcAddr = unsafe extern "C" fn(
    instance: VkInstance,
    pName: *const ::std::os::raw::c_char,
) -> crate::PFN_vkVoidFunction;
pub type PFN_vkGetDeviceProcAddr = unsafe extern "C" fn(
    device: VkDevice,
    pName: *const ::std::os::raw::c_char,
) -> crate::PFN_vkVoidFunction;
pub type PFN_vkCreateDevice = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDevice: *mut VkDevice,
) -> VkResult;
pub type PFN_vkDestroyDevice =
    unsafe extern "C" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "C" fn(
    pLayerName: *const ::std::os::raw::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult;
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pLayerName: *const ::std::os::raw::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult;
pub type PFN_vkEnumerateInstanceLayerProperties =
    unsafe extern "C" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult;
pub type PFN_vkGetDeviceQueue = unsafe extern "C" fn(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
);
pub type PFN_vkQueueSubmit = unsafe extern "C" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult;
pub type PFN_vkQueueWaitIdle = unsafe extern "C" fn(queue: VkQueue) -> VkResult;
pub type PFN_vkDeviceWaitIdle = unsafe extern "C" fn(device: VkDevice) -> VkResult;
pub type PFN_vkAllocateMemory = unsafe extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult;
pub type PFN_vkFreeMemory = unsafe extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkMapMemory = unsafe extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkUnmapMemory = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory);
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "C" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "C" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: *mut VkDeviceSize,
);
pub type PFN_vkBindBufferMemory = unsafe extern "C" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
pub type PFN_vkBindImageMemory = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *mut VkMemoryRequirements,
);
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
);
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties,
);
pub type PFN_vkQueueBindSparse = unsafe extern "C" fn(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult;
pub type PFN_vkCreateFence = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
pub type PFN_vkDestroyFence = unsafe extern "C" fn(
    device: VkDevice,
    fence: VkFence,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkResetFences =
    unsafe extern "C" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
pub type PFN_vkGetFenceStatus = unsafe extern "C" fn(device: VkDevice, fence: VkFence) -> VkResult;
pub type PFN_vkWaitForFences = unsafe extern "C" fn(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult;
pub type PFN_vkCreateSemaphore = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphore: *mut VkSemaphore,
) -> VkResult;
pub type PFN_vkDestroySemaphore = unsafe extern "C" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateEvent = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pEvent: *mut VkEvent,
) -> VkResult;
pub type PFN_vkDestroyEvent = unsafe extern "C" fn(
    device: VkDevice,
    event: VkEvent,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetEventStatus = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkSetEvent = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkResetEvent = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkCreateQueryPool = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pQueryPool: *mut VkQueryPool,
) -> VkResult;
pub type PFN_vkDestroyQueryPool = unsafe extern "C" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetQueryPoolResults = unsafe extern "C" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult;
pub type PFN_vkCreateBuffer = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult;
pub type PFN_vkDestroyBuffer = unsafe extern "C" fn(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateBufferView = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkBufferView,
) -> VkResult;
pub type PFN_vkDestroyBufferView = unsafe extern "C" fn(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateImage = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult;
pub type PFN_vkDestroyImage = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource,
    pLayout: *mut VkSubresourceLayout,
);
pub type PFN_vkCreateImageView = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkImageView,
) -> VkResult;
pub type PFN_vkDestroyImageView = unsafe extern "C" fn(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateShaderModule = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pShaderModule: *mut VkShaderModule,
) -> VkResult;
pub type PFN_vkDestroyShaderModule = unsafe extern "C" fn(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreatePipelineCache = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache,
) -> VkResult;
pub type PFN_vkDestroyPipelineCache = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetPipelineCacheData = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkMergePipelineCaches = unsafe extern "C" fn(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const VkPipelineCache,
) -> VkResult;
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
pub type PFN_vkCreateComputePipelines = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
pub type PFN_vkDestroyPipeline = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreatePipelineLayout = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult;
pub type PFN_vkDestroyPipelineLayout = unsafe extern "C" fn(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateSampler = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSampler: *mut VkSampler,
) -> VkResult;
pub type PFN_vkDestroySampler = unsafe extern "C" fn(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSetLayout: *mut VkDescriptorSetLayout,
) -> VkResult;
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "C" fn(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateDescriptorPool = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorPool: *mut VkDescriptorPool,
) -> VkResult;
pub type PFN_vkDestroyDescriptorPool = unsafe extern "C" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkResetDescriptorPool = unsafe extern "C" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult;
pub type PFN_vkAllocateDescriptorSets = unsafe extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    pDescriptorSets: *mut VkDescriptorSet,
) -> VkResult;
pub type PFN_vkFreeDescriptorSets = unsafe extern "C" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
) -> VkResult;
pub type PFN_vkUpdateDescriptorSets = unsafe extern "C" fn(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const VkCopyDescriptorSet,
);
pub type PFN_vkCreateFramebuffer = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult;
pub type PFN_vkDestroyFramebuffer = unsafe extern "C" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateRenderPass = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;
pub type PFN_vkDestroyRenderPass = unsafe extern "C" fn(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetRenderAreaGranularity =
    unsafe extern "C" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
pub type PFN_vkCreateCommandPool = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pCommandPool: *mut VkCommandPool,
) -> VkResult;
pub type PFN_vkDestroyCommandPool = unsafe extern "C" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkResetCommandPool = unsafe extern "C" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult;
pub type PFN_vkAllocateCommandBuffers = unsafe extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult;
pub type PFN_vkFreeCommandBuffers = unsafe extern "C" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
);
pub type PFN_vkBeginCommandBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkCommandBufferBeginInfo,
) -> VkResult;
pub type PFN_vkEndCommandBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer) -> VkResult;
pub type PFN_vkResetCommandBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult;
pub type PFN_vkCmdBindPipeline = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
);
pub type PFN_vkCmdSetViewport = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const VkViewport,
);
pub type PFN_vkCmdSetScissor = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const VkRect2D,
);
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);
pub type PFN_vkCmdSetDepthBias = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
);
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, blendConstants: *const f32);
pub type PFN_vkCmdSetDepthBounds =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
);
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
);
pub type PFN_vkCmdSetStencilReference = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
);
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
);
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
);
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
);
pub type PFN_vkCmdDraw = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
);
pub type PFN_vkCmdDrawIndexed = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
);
pub type PFN_vkCmdDrawIndirect = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDispatch = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
pub type PFN_vkCmdCopyBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferCopy,
);
pub type PFN_vkCmdCopyImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageCopy,
);
pub type PFN_vkCmdBlitImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageBlit,
    filter: VkFilter,
);
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
);
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
);
pub type PFN_vkCmdUpdateBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const ::std::os::raw::c_void,
);
pub type PFN_vkCmdFillBuffer = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
);
pub type PFN_vkCmdClearColorImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: *const VkClearColorValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
);
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
);
pub type PFN_vkCmdClearAttachments = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const VkClearAttachment,
    rectCount: u32,
    pRects: *const VkClearRect,
);
pub type PFN_vkCmdResolveImage = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageResolve,
);
pub type PFN_vkCmdSetEvent = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
);
pub type PFN_vkCmdResetEvent = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
);
pub type PFN_vkCmdWaitEvents = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
);
pub type PFN_vkCmdPipelineBarrier = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
);
pub type PFN_vkCmdBeginQuery = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
);
pub type PFN_vkCmdEndQuery =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdResetQueryPool = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
);
pub type PFN_vkCmdWriteTimestamp = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
);
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
);
pub type PFN_vkCmdPushConstants = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const ::std::os::raw::c_void,
);
pub type PFN_vkCmdBeginRenderPass = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
);
pub type PFN_vkCmdNextSubpass =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
pub type PFN_vkCmdEndRenderPass = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdExecuteCommands = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
);
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "C" fn(pApiVersion: *mut u32) -> VkResult;
pub type PFN_vkBindBufferMemory2 = unsafe extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult;
pub type PFN_vkBindImageMemory2 = unsafe extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult;
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "C" fn(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
);
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type PFN_vkCmdDispatchBase = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "C" fn(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult;
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
);
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
);
pub type PFN_vkTrimCommandPool = unsafe extern "C" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
);
pub type PFN_vkGetDeviceQueue2 = unsafe extern "C" fn(
    device: VkDevice,
    pQueueInfo: *const VkDeviceQueueInfo2,
    pQueue: *mut VkQueue,
);
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "C" fn(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "C" fn(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "C" fn(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const ::std::os::raw::c_void,
);
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
);
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
);
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCreateRenderPass2 = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
);
pub type PFN_vkCmdNextSubpass2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
);
pub type PFN_vkCmdEndRenderPass2 =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type PFN_vkResetQueryPool = unsafe extern "C" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
);
pub type PFN_vkGetSemaphoreCounterValue =
    unsafe extern "C" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
pub type PFN_vkWaitSemaphores = unsafe extern "C" fn(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult;
pub type PFN_vkSignalSemaphore =
    unsafe extern "C" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;
pub type PFN_vkGetBufferOpaqueCaptureAddress =
    unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult;
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult;
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "C" fn(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkSetPrivateData = unsafe extern "C" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult;
pub type PFN_vkGetPrivateData = unsafe extern "C" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
);
pub type PFN_vkCmdSetEvent2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
);
pub type PFN_vkCmdResetEvent2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
);
pub type PFN_vkCmdWaitEvents2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
);
pub type PFN_vkCmdPipelineBarrier2 =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
);
pub type PFN_vkQueueSubmit2 = unsafe extern "C" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult;
pub type PFN_vkCmdCopyBuffer2 =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);
pub type PFN_vkCmdCopyImage2 =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
);
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
);
pub type PFN_vkCmdBlitImage2 =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);
pub type PFN_vkCmdResolveImage2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
);
pub type PFN_vkCmdBeginRendering =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);
pub type PFN_vkCmdEndRendering = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdSetCullMode =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
);
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
);
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
);
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilOp = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
);
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
pub type PFN_vkDestroySurfaceKHR = unsafe extern "C" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: *mut VkBool32,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;
pub type PFN_vkCreateSwapchainKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchain: *mut VkSwapchainKHR,
) -> VkResult;
pub type PFN_vkDestroySwapchainKHR = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult;
pub type PFN_vkAcquireNextImageKHR = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: *mut u32,
) -> VkResult;
pub type PFN_vkQueuePresentKHR =
    unsafe extern "C" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "C" fn(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult;
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "C" fn(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut VkRect2D,
) -> VkResult;
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pAcquireInfo: *const VkAcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPropertiesKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult;
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut VkDisplayKHR,
) -> VkResult;
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult;
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: *const VkDisplayModeCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pMode: *mut VkDisplayModeKHR,
) -> VkResult;
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult;
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "C" fn(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: *const VkVideoProfileInfoKHR,
    pCapabilities: *mut VkVideoCapabilitiesKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR,
) -> VkResult;
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSession: *mut VkVideoSessionKHR,
) -> VkResult;
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "C" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "C" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
) -> VkResult;
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "C" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult;
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSessionParameters: *mut VkVideoSessionParametersKHR,
) -> VkResult;
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "C" fn(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult;
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "C" fn(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkVideoBeginCodingInfoKHR,
);
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: *const VkVideoEndCodingInfoKHR,
);
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: *const VkVideoCodingControlInfoKHR,
);
pub type PFN_vkCmdDecodeVideoKHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDecodeInfo: *const VkVideoDecodeInfoKHR);
pub type PFN_vkCmdBeginRenderingKHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);
pub type PFN_vkCmdEndRenderingKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
);
pub type PFN_vkGetPhysicalDeviceProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
);
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = unsafe extern "C" fn(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
);
pub type PFN_vkCmdSetDeviceMaskKHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type PFN_vkCmdDispatchBaseKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
pub type PFN_vkTrimCommandPoolKHR = unsafe extern "C" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
);
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = unsafe extern "C" fn(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
);
pub type PFN_vkGetMemoryFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult;
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "C" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: ::std::os::raw::c_int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
);
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult;
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult;
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
);
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const ::std::os::raw::c_void,
);
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "C" fn(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "C" fn(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const ::std::os::raw::c_void,
);
pub type PFN_vkCreateRenderPass2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;
pub type PFN_vkCmdBeginRenderPass2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
);
pub type PFN_vkCmdNextSubpass2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
);
pub type PFN_vkCmdEndRenderPass2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
);
pub type PFN_vkImportFenceFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pImportFenceFdInfo: *const VkImportFenceFdInfoKHR,
) -> VkResult;
pub type PFN_vkGetFenceFdKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetFdInfo: *const VkFenceGetFdInfoKHR,
    pFd: *mut ::std::os::raw::c_int,
) -> VkResult;
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        pCounterCount: *mut u32,
        pCounters: *mut VkPerformanceCounterKHR,
        pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
    ) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
    pNumPasses: *mut u32,
);
pub type PFN_vkAcquireProfilingLockKHR =
    unsafe extern "C" fn(device: VkDevice, pInfo: *const VkAcquireProfilingLockInfoKHR) -> VkResult;
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "C" fn(device: VkDevice);
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayProperties2KHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult;
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult;
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
    pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult;
pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
pub type PFN_vkCreateSamplerYcbcrConversionKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;
pub type PFN_vkDestroySamplerYcbcrConversionKHR = unsafe extern "C" fn(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkBindBufferMemory2KHR = unsafe extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult;
pub type PFN_vkBindImageMemory2KHR = unsafe extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult;
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
);
pub type PFN_vkCmdDrawIndirectCountKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkGetSemaphoreCounterValueKHR =
    unsafe extern "C" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
pub type PFN_vkWaitSemaphoresKHR = unsafe extern "C" fn(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult;
pub type PFN_vkSignalSemaphoreKHR =
    unsafe extern "C" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult;
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: *const VkExtent2D,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
);
pub type PFN_vkWaitForPresentKHR = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult;
pub type PFN_vkGetBufferDeviceAddressKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR =
    unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "C" fn(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
    pDeferredOperation: *mut VkDeferredOperationKHR,
) -> VkResult;
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "C" fn(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> u32;
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "C" fn(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult;
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "C" fn(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult;
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "C" fn(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult;
pub type PFN_vkMapMemory2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfoKHR,
    ppData: *mut *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkUnmapMemory2KHR = unsafe extern "C" fn(
    device: VkDevice,
    pMemoryUnmapInfo: *const VkMemoryUnmapInfoKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR,
    ) -> VkResult;
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "C" fn(
    device: VkDevice,
    pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCmdEncodeVideoKHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pEncodeInfo: *const VkVideoEncodeInfoKHR);
pub type PFN_vkCmdSetEvent2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
);
pub type PFN_vkCmdResetEvent2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
);
pub type PFN_vkCmdWaitEvents2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
);
pub type PFN_vkCmdPipelineBarrier2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);
pub type PFN_vkCmdWriteTimestamp2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
);
pub type PFN_vkQueueSubmit2KHR = unsafe extern "C" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult;
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
);
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "C" fn(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointData2NV,
);
pub type PFN_vkCmdCopyBuffer2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);
pub type PFN_vkCmdCopyImage2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);
pub type PFN_vkCmdCopyBufferToImage2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
);
pub type PFN_vkCmdCopyImageToBuffer2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
);
pub type PFN_vkCmdBlitImage2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);
pub type PFN_vkCmdResolveImage2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
);
pub type PFN_vkCmdTraceRaysIndirect2KHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, indirectDeviceAddress: VkDeviceAddress);
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
pub type PFN_vkCmdBindIndexBuffer2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
);
pub type PFN_vkGetRenderingAreaGranularityKHR = unsafe extern "C" fn(
    device: VkDevice,
    pRenderingAreaInfo: *const VkRenderingAreaInfoKHR,
    pGranularity: *mut VkExtent2D,
);
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageSubresourceInfoKHR,
    pLayout: *mut VkSubresourceLayout2KHR,
);
pub type PFN_vkGetImageSubresourceLayout2KHR = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2KHR,
    pLayout: *mut VkSubresourceLayout2KHR,
);
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
) -> VkResult;
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "C" fn(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoKHR,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult;
pub type PFN_vkCmdBindDescriptorSets2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfoKHR,
);
pub type PFN_vkCmdPushConstants2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pPushConstantsInfo: *const VkPushConstantsInfoKHR,
);
pub type PFN_vkCmdPushDescriptorSet2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetInfo: *const VkPushDescriptorSetInfoKHR,
);
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfoKHR,
);

pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pSetDescriptorBufferOffsetsInfo: *const VkSetDescriptorBufferOffsetsInfoEXT,
);
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = :: std :: option :: Option < unsafe extern "C" fn (commandBuffer : VkCommandBuffer , pBindDescriptorBufferEmbeddedSamplersInfo : * const VkBindDescriptorBufferEmbeddedSamplersInfoEXT) > ;
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "C" fn(
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    messageCode: i32,
    pLayerPrefix: *const ::std::os::raw::c_char,
    pMessage: *const ::std::os::raw::c_char,
    pUserData: *mut ::std::os::raw::c_void,
) -> VkBool32;
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult;
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "C" fn(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkDebugReportMessageEXT = unsafe extern "C" fn(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    messageCode: i32,
    pLayerPrefix: *const ::std::os::raw::c_char,
    pMessage: *const ::std::os::raw::c_char,
);
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "C" fn(
    device: VkDevice,
    pTagInfo: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult;
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "C" fn(
    device: VkDevice,
    pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult;
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
);
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
);
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
);
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
);
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
);
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
);
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
);
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
);
pub type PFN_vkCreateCuModuleNVX = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCuModuleNVX,
) -> VkResult;
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCuFunctionNVX,
) -> VkResult;
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "C" fn(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "C" fn(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdCuLaunchKernelNVX =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLaunchInfo: *const VkCuLaunchInfoNVX);
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "C" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> u32;
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "C" fn(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult;
pub type PFN_vkCmdDrawIndirectCountAMD = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkGetShaderInfoAMD = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut usize,
    pInfo: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
) -> VkResult;
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT,
);
pub type PFN_vkCmdEndConditionalRenderingEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const VkViewportWScalingNV,
);
pub type PFN_vkReleaseDisplayEXT =
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult;
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "C" fn(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
) -> VkResult;
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "C" fn(
    device: VkDevice,
    pDeviceEventInfo: *const VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "C" fn(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayEventInfo: *const VkDisplayEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> VkResult;
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult;
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
);
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, discardRectangleEnable: VkBool32);
pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
);
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "C" fn(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
);
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut ::std::os::raw::c_void,
) -> VkBool32;
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "C" fn(
    device: VkDevice,
    pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult;
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "C" fn(
    device: VkDevice,
    pTagInfo: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult;
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "C" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "C" fn(queue: VkQueue);
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "C" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCmdBeginDebugUtilsLabelEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdInsertDebugUtilsLabelEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "C" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "C" fn(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
);
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
);
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
);
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
) -> VkResult;
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult;
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "C" fn(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "C" fn(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult;
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
);
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
);
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
);
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult;
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "C" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2KHR,
);
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult;
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkAccelerationStructureInfoNV,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
);
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
);
pub type PFN_vkCmdTraceRaysNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    raygenShaderBindingTableBuffer: VkBuffer,
    raygenShaderBindingOffset: VkDeviceSize,
    missShaderBindingTableBuffer: VkBuffer,
    missShaderBindingOffset: VkDeviceSize,
    missShaderBindingStride: VkDeviceSize,
    hitShaderBindingTableBuffer: VkBuffer,
    hitShaderBindingOffset: VkDeviceSize,
    hitShaderBindingStride: VkDeviceSize,
    callableShaderBindingTableBuffer: VkBuffer,
    callableShaderBindingOffset: VkDeviceSize,
    callableShaderBindingStride: VkDeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "C" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "C" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureNV,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
pub type PFN_vkCompileDeferredNV =
    unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, shader: u32) -> VkResult;
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "C" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const ::std::os::raw::c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
);
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
) -> VkResult;
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "C" fn(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoKHR,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult;
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, taskCount: u32, firstTask: u32);
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const VkBool32,
);
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const VkRect2D,
);
pub type PFN_vkCmdSetCheckpointNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: *const ::std::os::raw::c_void,
);
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "C" fn(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointDataNV,
);
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "C" fn(
    device: VkDevice,
    pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult;
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "C" fn(device: VkDevice);
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult;
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult;
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult;
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "C" fn(
    device: VkDevice,
    pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
    pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult;
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "C" fn(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult;
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "C" fn(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult;
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "C" fn(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: *mut VkPerformanceValueINTEL,
) -> VkResult;
pub type PFN_vkSetLocalDimmingAMD =
    unsafe extern "C" fn(device: VkDevice, swapChain: VkSwapchainKHR, localDimmingEnable: VkBool32);
pub type PFN_vkGetBufferDeviceAddressEXT = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "C" fn(
        physicalDevice: VkPhysicalDevice,
        pCombinationCount: *mut u32,
        pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
    ) -> VkResult;
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
);
pub type PFN_vkResetQueryPoolEXT = unsafe extern "C" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
);
pub type PFN_vkCmdSetCullModeEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
pub type PFN_vkCmdSetFrontFaceEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
pub type PFN_vkCmdSetPrimitiveTopologyEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
pub type PFN_vkCmdSetViewportWithCountEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
);
pub type PFN_vkCmdSetScissorWithCountEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
);
pub type PFN_vkCmdBindVertexBuffers2EXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
);
pub type PFN_vkCmdSetDepthTestEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
pub type PFN_vkCmdSetDepthWriteEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
pub type PFN_vkCmdSetDepthCompareOpEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilTestEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilOpEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
);
pub type PFN_vkCopyMemoryToImageEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfoEXT,
) -> VkResult;
pub type PFN_vkCopyImageToMemoryEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfoEXT,
) -> VkResult;
pub type PFN_vkCopyImageToImageEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCopyImageToImageInfo: *const VkCopyImageToImageInfoEXT,
) -> VkResult;
pub type PFN_vkTransitionImageLayoutEXT = unsafe extern "C" fn(
    device: VkDevice,
    transitionCount: u32,
    pTransitions: *const VkHostImageLayoutTransitionInfoEXT,
) -> VkResult;
pub type PFN_vkGetImageSubresourceLayout2EXT = unsafe extern "C" fn(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2KHR,
    pLayout: *mut VkSubresourceLayout2KHR,
);
pub type PFN_vkReleaseSwapchainImagesEXT = unsafe extern "C" fn(
    device: VkDevice,
    pReleaseInfo: *const VkReleaseSwapchainImagesInfoEXT,
) -> VkResult;
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
);
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
);
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
);
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult;
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "C" fn(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdSetDepthBias2EXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDepthBiasInfo: *const VkDepthBiasInfoEXT);
pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "C" fn(
    pCallbackData: *const VkDeviceMemoryReportCallbackDataEXT,
    pUserData: *mut ::std::os::raw::c_void,
);
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult;
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut VkDisplayKHR,
) -> VkResult;
pub type PFN_vkCreatePrivateDataSlotEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult;
pub type PFN_vkDestroyPrivateDataSlotEXT = unsafe extern "C" fn(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkSetPrivateDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult;
pub type PFN_vkGetPrivateDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
);
pub type PFN_vkCreateCudaModuleNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkCudaModuleCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCudaModuleNV,
) -> VkResult;
pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "C" fn(
    device: VkDevice,
    module: VkCudaModuleNV,
    pCacheSize: *mut usize,
    pCacheData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkCudaFunctionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCudaFunctionNV,
) -> VkResult;
pub type PFN_vkDestroyCudaModuleNV = unsafe extern "C" fn(
    device: VkDevice,
    module: VkCudaModuleNV,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "C" fn(
    device: VkDevice,
    function: VkCudaFunctionNV,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdCudaLaunchKernelNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLaunchInfo: *const VkCudaLaunchInfoNV);
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "C" fn(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut VkDeviceSize,
);
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "C" fn(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut VkDeviceSize,
);
pub type PFN_vkGetDescriptorEXT = unsafe extern "C" fn(
    device: VkDevice,
    pDescriptorInfo: *const VkDescriptorGetInfoEXT,
    dataSize: usize,
    pDescriptor: *mut ::std::os::raw::c_void,
);
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const VkDescriptorBufferBindingInfoEXT,
);
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: *const u32,
    pOffsets: *const VkDeviceSize,
);
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
);
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "C" fn(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
        pData: *mut ::std::os::raw::c_void,
    ) -> VkResult;
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
);
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "C" fn(
    device: VkDevice,
    pFaultCounts: *mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
) -> VkResult;
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
);
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "C" fn(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: *mut VkExtent2D,
) -> VkResult;
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
);
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "C" fn(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
    pAddress: *mut VkRemoteAddressNV,
) -> VkResult;
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "C" fn(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoEXT,
    pPipelineProperties: *mut VkBaseOutStructure,
) -> VkResult;
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, patchControlPoints: u32);
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
pub type PFN_vkCmdSetDepthBiasEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp);
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const VkBool32,
);
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
);
pub type PFN_vkCreateMicromapEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMicromap: *mut VkMicromapEXT,
) -> VkResult;
pub type PFN_vkDestroyMicromapEXT = unsafe extern "C" fn(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
);
pub type PFN_vkBuildMicromapsEXT = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) -> VkResult;
pub type PFN_vkCopyMicromapEXT = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapInfoEXT,
) -> VkResult;
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) -> VkResult;
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) -> VkResult;
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "C" fn(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: usize,
) -> VkResult;
pub type PFN_vkCmdCopyMicromapEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyMicromapInfoEXT);
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
);
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
);
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "C" fn(
    device: VkDevice,
    pVersionInfo: *const VkMicromapVersionInfoEXT,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
);
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "C" fn(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkMicromapBuildInfoEXT,
    pSizeInfo: *mut VkMicromapBuildSizesInfoEXT,
);
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, priority: f32);
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "C" fn(
    device: VkDevice,
    pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
);
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "C" fn(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: *mut *mut ::std::os::raw::c_void,
);
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
);
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: *const VkImageSubresourceLayers,
);
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV,
);
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
);
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkComputePipelineCreateInfo,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
);
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkPipelineIndirectDeviceAddressInfoNV,
) -> VkDeviceAddress;
pub type PFN_vkCmdSetTessellationDomainOriginEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, domainOrigin: VkTessellationDomainOrigin);
pub type PFN_vkCmdSetDepthClampEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthClampEnable: VkBool32);
pub type PFN_vkCmdSetPolygonModeEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, polygonMode: VkPolygonMode);
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
);
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
);
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, alphaToCoverageEnable: VkBool32);
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, alphaToOneEnable: VkBool32);
pub type PFN_vkCmdSetLogicOpEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, logicOpEnable: VkBool32);
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const VkBool32,
);
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const VkColorBlendEquationEXT,
);
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
);
pub type PFN_vkCmdSetRasterizationStreamEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, rasterizationStream: u32);
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
);
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, extraPrimitiveOverestimationSize: f32);
pub type PFN_vkCmdSetDepthClipEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthClipEnable: VkBool32);
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, sampleLocationsEnable: VkBool32);
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const VkColorBlendAdvancedEXT,
);
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
);
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
);
pub type PFN_vkCmdSetLineStippleEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stippledLineEnable: VkBool32);
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, negativeOneToOne: VkBool32);
pub type PFN_vkCmdSetViewportWScalingEnableNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, viewportWScalingEnable: VkBool32);
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const VkViewportSwizzleNV,
);
pub type PFN_vkCmdSetCoverageToColorEnableNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, coverageToColorEnable: VkBool32);
pub type PFN_vkCmdSetCoverageToColorLocationNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, coverageToColorLocation: u32);
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
);
pub type PFN_vkCmdSetCoverageModulationTableEnableNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, coverageModulationTableEnable: VkBool32);
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
);
pub type PFN_vkCmdSetShadingRateImageEnableNV =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, shadingRateImageEnable: VkBool32);
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
);
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
);
pub type PFN_vkGetInstanceProcAddrLUNARG = unsafe extern "C" fn(
    instance: VkInstance,
    pName: *const ::std::os::raw::c_char,
) -> crate::PFN_vkVoidFunction;
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "C" fn(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
);
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
);
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV,
) -> VkResult;
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkOpticalFlowSessionNV,
) -> VkResult;
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "C" fn(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "C" fn(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult;
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: *const VkOpticalFlowExecuteInfoNV,
);
pub type PFN_vkCreateShadersEXT = unsafe extern "C" fn(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: *const VkShaderCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pShaders: *mut VkShaderEXT,
) -> VkResult;
pub type PFN_vkDestroyShaderEXT = unsafe extern "C" fn(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "C" fn(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: *mut usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCmdBindShadersEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: *const VkShaderStageFlagBits,
    pShaders: *const VkShaderEXT,
);
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "C" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult;
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "C" fn(
    device: VkDevice,
    pRenderingInfo: *const VkRenderingInfo,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult;
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepModeInfo: *const VkLatencySleepModeInfoNV,
) -> VkResult;
pub type PFN_vkLatencySleepNV = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepInfo: *const VkLatencySleepInfoNV,
) -> VkResult;
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *const VkSetLatencyMarkerInfoNV,
);
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "C" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *mut VkGetLatencyMarkerInfoNV,
);
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "C" fn(queue: VkQueue, pQueueTypeInfo: *const VkOutOfBandQueueTypeInfoNV);
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, aspectMask: VkImageAspectFlags);
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult;
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "C" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
);
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pIndirectDeviceAddresses: *const VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const *const u32,
);
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult;
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult;
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult;
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult;
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
    stride: usize,
) -> VkResult;
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
);
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
);
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
);
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "C" fn(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
) -> VkDeviceAddress;
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "C" fn(
    device: VkDevice,
    pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
);
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "C" fn(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "C" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut ::std::os::raw::c_void,
) -> VkResult;
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
);
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "C" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize;
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineStackSize: u32);
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "C" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "C" fn(
    instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "C" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "C" fn(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult;
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "C" fn(
    device: VkDevice,
    pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult;
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "C" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut HANDLE,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "C" fn(
    device: VkDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
pub type PFN_vkAcquireWinrtDisplayNV =
    unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult;
