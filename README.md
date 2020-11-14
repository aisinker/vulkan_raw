# vulkan_raw

[![Docs status](https://docs.rs/vulkan_raw/badge.svg)](https://docs.rs/vulkan_raw)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/aisinker/vulkan_raw/master/LICENSE)

## Usage

1. Almost names are the same as the Vulkan C API. But for some simplification reasons, a little change must be taken. The `enum` variant name is changed to without `VK_` prefix and enum name. For example the `VkFormat.VK_FORMAT_UNDEFINED` is changed to `VkFormat::UNDEFINED`. Because of language limitations, some exceptions exist. They are the following:

    | C version | Corresponding vulkan_raw version|
    | ------ | ------ |
    | `VkImageCreateFlagBits.VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` | `VkImageCreateFlagBits::IC_2D_ARRAY_COMPATIBLE_BIT` |
    | `VkQueryResultFlagBits.VK_QUERY_RESULT_64_BIT` | `VkQueryResultFlagBits::U64_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_1_BIT` | `VkSampleCountFlagBits::SC_1_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_2_BIT` | `VkSampleCountFlagBits::SC_2_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_4_BIT` | `VkSampleCountFlagBits::SC_4_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_8_BIT` | `VkSampleCountFlagBits::SC_8_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_16_BIT` | `VkSampleCountFlagBits::SC_16_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_32_BIT` | `VkSampleCountFlagBits::SC_32_BIT` |
    | `VkSampleCountFlagBits.VK_SAMPLE_COUNT_64_BIT` | `VkSampleCountFlagBits::SC_64_BIT` |
    | `VkImageType.VK_IMAGE_TYPE_1D` | `VkImageType::IT_1D` |
    | `VkImageType.VK_IMAGE_TYPE_2D` | `VkImageType::IT_2D` |
    | `VkImageType.VK_IMAGE_TYPE_3D` | `VkImageType::IT_3D` |
    | `VkImageViewType.VK_IMAGE_VIEW_TYPE_1D` | `VkImageViewType::IVT_1D` |
    | `VkImageViewType.VK_IMAGE_VIEW_TYPE_2D` | `VkImageViewType::IVT_2D` |
    | `VkImageViewType.VK_IMAGE_VIEW_TYPE_3D` | `VkImageViewType::IVT_3D` |
    | `VkImageViewType.VK_IMAGE_VIEW_TYPE_1D_ARRAY` | `VkImageViewType::IVT_1D_ARRAY` |
    | `VkImageViewType.VK_IMAGE_VIEW_TYPE_2D_ARRAY` | `VkImageViewType::IVT_2D_ARRAY` |
    | `VkShaderFloatControlsIndependence.VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY` | `VkShaderFloatControlsIndependence::F32_BIT_ONLY` |

2. Most functions must need to obtain a corresponding function pointer before use. We provide `InstanceLevelFunctions` object and/or `DeviceLevelFunctions` object to do this on every module, each one corresponding to the module's functions set. The `InstanceLevelFunctions` only can be loaded from `VkInstance`, but the `DeviceLevelFunctions` also can be loaded from `VkDevice`.
3. All structures have `Debug` trait and `Default` trait. You don't need to set the `sType` field manually if it contains.
## Example

```rust
use std::ffi::CStr;
use std::ptr;
use vulkan_raw::*;

fn main() {
    let mut instance_version: u32 = 0;
    unsafe { vkEnumerateInstanceVersion(&mut instance_version) };
    println!("Instance version: {}", ApiVersion::from(instance_version));

    // Create Vulkan instance
    let app_info = VkApplicationInfo {
        apiVersion: ApiVersion::new(1, 2, 0).into(),
        ..Default::default()
    };
    let create_info = VkInstanceCreateInfo {
        pApplicationInfo: &app_info,
        ..Default::default()
    };
    let mut instance: VkInstance = VkInstance::none();
    let result = unsafe { vkCreateInstance(&create_info, ptr::null(), &mut instance) };
    assert_eq!(result, VkResult::SUCCESS);

    let functions = InstanceLevelFunctions::load_from_instance(instance);

    // Enumerate all devices
    let mut count: u32 = 0;
    let result =
        unsafe { functions.vkEnumeratePhysicalDevices(instance, &mut count, ptr::null_mut()) };
    assert_eq!(result, VkResult::SUCCESS);

    let mut physical_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(count as usize);
    let result = unsafe {
        functions.vkEnumeratePhysicalDevices(instance, &mut count, physical_devices.as_mut_ptr())
    };
    assert_eq!(result, VkResult::SUCCESS);
    unsafe {
        physical_devices.set_len(count as usize);
    }

    let mut physical_device_properties = Default::default();
    for physical_device in physical_devices {
        unsafe {
            functions
                .vkGetPhysicalDeviceProperties2(physical_device, &mut physical_device_properties);
        }
        println!(
            "Physical Device: {}; \
            Supported vulkan version: {}",
            unsafe { CStr::from_ptr(physical_device_properties.properties.deviceName.as_ptr()) }
                .to_str()
                .unwrap(),
            ApiVersion::from(physical_device_properties.properties.apiVersion)
        );
    }

    unsafe { functions.vkDestroyInstance(instance, ptr::null()) };
}
```

## Supported API

- [x] Vulkan 1.2 core API
- [x] VK_KHR_external_fence_fd(Revision: 1)
- [x] VK_KHR_external_fence_win32(Revision: 1)
- [x] VK_KHR_surface(Revision: 25)
- [x] VK_KHR_swapchain(Revision: 70)
- [x] VK_KHR_win32_surface(Revision: 6)
- [x] VK_KHR_pipeline_library(Revision: 1)
- [x] VK_KHR_deferred_host_operations(Revision: 3)
- [x] VK_KHR_ray_tracing(Revision: 8)
- [x] VK_EXT_debug_utils(Revision: 2)
- [x] VK_EXT_index_type_uint8(Revision: 1)
- [x] VK_EXT_memory_budget(Revision: 1)