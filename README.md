# vulkan-sys

A ffi wrapper around the Vulkan graphics and compute API.

## Supported Platforms

As of now, only Windows is being tested and supported.
Implementing support for other platforms will be relatively easy as only some platform primitives will most likely need
to be defined.

## Building

This crate currently requires the Vulkan SDK to be installed on your system.
The VULKAN_SDK environment variable is used to locate the SDK.
You can download it from the [LunarG website](https://vulkan.lunarg.com/sdk/home).

## Future

In the future this crate will build from the vk.xml file instead of bindgen output.
This will allow for more control of the generate code and allow for generated wrappers instead of handwritten ones.