# vulkan_rs
A very thin rust wrapper for vulkan c api. current support Windows/Linux

## Usage

Almost names are same as the Vulkan C API. But for some simplification reasons, a little change must be taked. The `enum` variant name is changed to without `VK_` prefix and enum name. For example the `VkFormat.VK_FORMAT_UNDEFINED` is changed to `VkFormat::UNDEFINED`. Because language limition, some exceptions exit. They are follow: 

| C version | Corresponding vulkan_rs version|
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


## Example
```rust
use vulkan_rs::*;
use std::ptr;
use std::ffi::CStr;

fn main(){
    let mut instance_version: u32 = 0;
    unsafe {vkEnumerateInstanceVersion(&mut instance_version)};
    println!("instance version: {}", tools::ApiVersion::from(instance_version));

    // Create Vulkan instance
    let app_info = VkApplicationInfo{
        sType: VkStructureType::APPLICATION_INFO,
        pNext: ptr::null(),
        pApplicationName: ptr::null(),
        applicationVersion: 1,
        pEngineName: ptr::null(),
        engineVersion: 1,
        apiVersion: tools::ApiVersion::new(1, 2, 0).into(),
    };
    let create_info = VkInstanceCreateInfo{
        sType: VkStructureType::INSTANCE_CREATE_INFO,
        pNext: ptr::null(),
        flags: Default::default(),
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: ptr::null(),
    };
    let mut instance: VkInstance = Default::default();
    let result = unsafe {vkCreateInstance(&create_info, ptr::null(), &mut instance)};
    if result != VkResult::SUCCESS { panic!("error!") }

    // Enumerate all devices
    let mut count: u32 = 0;
    unsafe {
        let result = vkEnumeratePhysicalDevices(instance, &mut count, ptr::null_mut());
        if result != VkResult::SUCCESS { panic!("error!") }
    }
    let mut physical_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(count as usize);
    unsafe {
        let result = vkEnumeratePhysicalDevices(instance, &mut count, physical_devices.as_mut_ptr());
        if result != VkResult::SUCCESS { panic!("error!") }
        physical_devices.set_len(count as usize);
    }
    for physical_device  in physical_devices{
        let mut physical_device_properties = VkPhysicalDeviceProperties2{
            sType: VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            pNext: ptr::null(),
            properties: Default::default(),
        };
        unsafe { vkGetPhysicalDeviceProperties2(physical_device, &mut physical_device_properties); }
        println!(
            "device: {}, supported vulkan version: {}",
            unsafe {CStr::from_ptr(physical_device_properties.properties.deviceName.as_ptr())}.to_str().unwrap(),
            tools::ApiVersion::from(physical_device_properties.properties.apiVersion)
        );
    }

    unsafe { vkDestroyInstance(instance, ptr::null())};
}
```
## Setup

For linking dynamic library, vulkan_rs needs [VulkanSDK](https://vulkan.lunarg.com/sdk/home) be installed.

## Supported API
- [x] Vulkan 1.2 core API
- [ ] VK_KHR_android_surface
- [ ] VK_KHR_display
- [ ] VK_KHR_display_swapchain
- [ ] VK_KHR_external_fence_fd
- [ ] VK_KHR_external_fence_win32
- [ ] VK_KHR_external_memory_fd
- [ ] VK_KHR_external_memory_win32
- [ ] VK_KHR_external_semaphore_fd
- [ ] VK_KHR_external_semaphore_win32
- [ ] VK_KHR_get_display_properties2
- [ ] VK_KHR_get_surface_capabilities2
- [ ] VK_KHR_incremental_present
- [ ] VK_KHR_performance_query
- [ ] VK_KHR_pipeline_executable_properties
- [ ] VK_KHR_push_descriptor
- [ ] VK_KHR_shader_clock
- [ ] VK_KHR_shared_presentable_image
- [x] VK_KHR_surface(Revision: 25)
- [ ] VK_KHR_surface_protected_capabilities
- [x] VK_KHR_swapchain(Revision: 70)
- [ ] VK_KHR_swapchain_mutable_format
- [ ] VK_KHR_wayland_surface
- [ ] VK_KHR_win32_keyed_mutex
- [x] VK_KHR_win32_surface(Revision: 6)
- [ ] VK_KHR_xcb_surface
- [ ] VK_KHR_xlib_surface
- [ ] VK_EXT_acquire_xlib_display
- [ ] VK_EXT_astc_decode_mode
- [ ] VK_EXT_blend_operation_advanced
- [ ] VK_EXT_calibrated_timestamps
- [ ] VK_EXT_conditional_rendering
- [ ] VK_EXT_conservative_rasterization
- [x] VK_EXT_debug_utils(Revision: 1)
- [ ] VK_EXT_depth_clip_enable
- [ ] VK_EXT_depth_range_unrestricted
- [ ] VK_EXT_direct_mode_display
- [ ] VK_EXT_discard_rectangles
- [ ] VK_EXT_display_control
- [ ] VK_EXT_display_surface_counter
- [ ] VK_EXT_external_memory_dma_buf
- [ ] VK_EXT_external_memory_host
- [ ] VK_EXT_filter_cubic
- [ ] VK_EXT_fragment_density_map
- [ ] VK_EXT_fragment_shader_interlock
- [ ] VK_EXT_full_screen_exclusive
- [ ] VK_EXT_global_priority
- [ ] VK_EXT_hdr_metadata
- [ ] VK_EXT_headless_surface
- [ ] VK_EXT_image_drm_format_modifier
- [ ] VK_EXT_index_type_uint8
- [ ] VK_EXT_inline_uniform_block
- [ ] VK_EXT_line_rasterization
- [ ] VK_EXT_memory_budget
- [ ] VK_EXT_memory_priority
- [ ] VK_EXT_metal_surface
- [ ] VK_EXT_pci_bus_info
- [ ] VK_EXT_pipeline_creation_feedback
- [ ] VK_EXT_post_depth_coverage
- [ ] VK_EXT_queue_family_foreign
- [ ] VK_EXT_sample_locations
- [ ] VK_EXT_shader_demote_to_helper_invocation
- [ ] VK_EXT_shader_stencil_export
- [ ] VK_EXT_shader_subgroup_ballot
- [ ] VK_EXT_shader_subgroup_vote
- [ ] VK_EXT_subgroup_size_control
- [ ] VK_EXT_swapchain_colorspace
- [ ] VK_EXT_texel_buffer_alignment
- [ ] VK_EXT_texture_compression_astc_hdr
- [ ] VK_EXT_tooling_info
- [ ] VK_EXT_transform_feedback
- [ ] VK_EXT_validation_cache
- [ ] VK_EXT_validation_features
- [ ] VK_EXT_vertex_attribute_divisor
- [ ] VK_EXT_ycbcr_image_arrays
- [ ] VK_AMD_buffer_marker
- [ ] VK_AMD_device_coherent_memory
- [ ] VK_AMD_display_native_hdr
- [ ] VK_AMD_gcn_shader
- [ ] VK_AMD_memory_overallocation_behavior
- [ ] VK_AMD_mixed_attachment_samples
- [ ] VK_AMD_pipeline_compiler_control
- [ ] VK_AMD_rasterization_order
- [ ] VK_AMD_shader_ballot
- [ ] VK_AMD_shader_core_properties
- [ ] VK_AMD_shader_core_properties2
- [ ] VK_AMD_shader_explicit_vertex_parameter
- [ ] VK_AMD_shader_fragment_mask
- [ ] VK_AMD_shader_image_load_store_lod
- [ ] VK_AMD_shader_info
- [ ] VK_AMD_shader_trinary_minmax
- [ ] VK_AMD_texture_gather_bias_lod
- [ ] VK_ANDROID_external_memory_android_hardware_buffer
- [ ] VK_FUCHSIA_imagepipe_surface
- [ ] VK_GGP_frame_token
- [ ] VK_GGP_stream_descriptor_surface
- [ ] VK_GOOGLE_decorate_string
- [ ] VK_GOOGLE_display_timing
- [ ] VK_GOOGLE_hlsl_functionality1
- [ ] VK_GOOGLE_user_type
- [ ] VK_IMG_filter_cubic
- [ ] VK_IMG_format_pvrtc
- [ ] VK_INTEL_performance_query
- [ ] VK_INTEL_shader_integer_functions2
- [ ] VK_MVK_ios_surface
- [ ] VK_MVK_macos_surface
- [ ] VK_NN_vi_surface
- [ ] VK_NV_clip_space_w_scaling
- [ ] VK_NV_compute_shader_derivatives
- [ ] VK_NV_cooperative_matrix
- [ ] VK_NV_corner_sampled_image
- [ ] VK_NV_coverage_reduction_mode
- [ ] VK_NV_dedicated_allocation_image_aliasing
- [ ] VK_NV_device_diagnostic_checkpoints
- [ ] VK_NV_fill_rectangle
- [ ] VK_NV_fragment_coverage_to_color
- [ ] VK_NV_fragment_shader_barycentric
- [ ] VK_NV_framebuffer_mixed_samples
- [ ] VK_NV_geometry_shader_passthrough
- [ ] VK_NV_mesh_shader
- [ ] VK_NV_ray_tracing
- [ ] VK_NV_representative_fragment_test
- [ ] VK_NV_sample_mask_override_coverage
- [ ] VK_NV_scissor_exclusive
- [ ] VK_NV_shader_image_footprint
- [ ] VK_NV_shader_sm_builtins
- [ ] VK_NV_shader_subgroup_partitioned
- [ ] VK_NV_shading_rate_image
- [ ] VK_NV_viewport_array2
- [ ] VK_NV_viewport_swizzle
- [ ] VK_NVX_device_generated_commands
- [ ] VK_NVX_image_view_handle
- [ ] VK_NVX_multiview_per_view_attributes