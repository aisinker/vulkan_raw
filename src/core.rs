#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_void};
use std::ffi::{CStr};
use std::fmt::{Display, Formatter, Error};

use bitflags::bitflags;

use super::{DispatchableHandle, NonDispatchableHandle};

reserved_flags!(
    VkShaderModuleCreateFlags,
    VkPipelineShaderStageCreateFlags,
    VkPipelineMultisampleStateCreateFlags,
    VkPipelineColorBlendStateCreateFlags,
    VkPipelineDynamicStateCreateFlags,
    VkPipelineLayoutCreateFlags,
    VkPipelineDepthStencilStateCreateFlags,
    VkRenderPassCreateFlags,
    VkFramebufferCreateFlags,
    VkSemaphoreCreateFlags,
    VkMemoryMapFlags,
    VkBufferViewCreateFlags,
    VkInstanceCreateFlags,
    VkDeviceCreateFlags,
    VkPipelineCacheCreateFlags,
    VkPipelineVertexInputStateCreateFlags,
    VkPipelineInputAssemblyStateCreateFlags,
    VkPipelineTessellationStateCreateFlags,
    VkPipelineViewportStateCreateFlags,
    VkPipelineRasterizationStateCreateFlags
);

pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;
bitflags! {
    #[repr(C)]
	#[derive(Default)]
    pub struct VkSamplerCreateFlagBits: u32 {
        const SUBSAMPLED_BIT_EXT = 0x00000001;
        const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 0x00000002;
    }
}

pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkImageViewCreateFlagBits: u32 {
    	    const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 0x00000001;
        }
    }

pub type VkQueueFlags = VkQueueFlagBits;
bitflags! {
        #[repr(C)]
        #[derive(Default)]
        pub struct VkQueueFlagBits: u32 {
            const GRAPHICS_BIT = 0x00000001;
            const COMPUTE_BIT = 0x00000002;
            const TRANSFER_BIT = 0x00000004;
            const SPARSE_BINDING_BIT = 0x00000008;
            const PROTECTED_BIT = 0x00000010;
        }
    }

pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;
bitflags! {
        #[repr(C)]
        #[derive(Default)]
        pub struct VkDeviceQueueCreateFlagBits: u32 {
            const PROTECTED_BIT = 0x00000001;
        }
    }

pub type VkImageUsageFlags = VkImageUsageFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkImageUsageFlagBits: u32 {
    	    const TRANSFER_SRC_BIT = 0x00000001;
            const TRANSFER_DST_BIT = 0x00000002;
            const SAMPLED_BIT = 0x00000004;
            const STORAGE_BIT = 0x00000008;
            const COLOR_ATTACHMENT_BIT = 0x00000010;
            const DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020;
            const TRANSIENT_ATTACHMENT_BIT = 0x00000040;
            const INPUT_ATTACHMENT_BIT = 0x00000080;
            const SHADING_RATE_IMAGE_BIT_NV = 0x00000100;
        }
    }

pub type VkImageAspectFlags = VkImageAspectFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkImageAspectFlagBits: u32 {
    	    const COLOR_BIT = 0x00000001;
            const DEPTH_BIT = 0x00000002;
            const STENCIL_BIT = 0x00000004;
            const METADATA_BIT = 0x00000008;
            const PLANE_0_BIT = 0x00000010;
            const PLANE_1_BIT = 0x00000020;
            const PLANE_2_BIT = 0x00000040;
            const MEMORY_PLANE_0_BIT_EXT = 0x00000080;
            const MEMORY_PLANE_1_BIT_EXT = 0x00000100;
            const MEMORY_PLANE_2_BIT_EXT = 0x00000200;
            const MEMORY_PLANE_3_BIT_EXT = 0x00000400;
        }
    }

pub type VkShaderStageFlags = VkShaderStageFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkShaderStageFlagBits: u32 {
    	    const VERTEX_BIT = 0x00000001;
            const TESSELLATION_CONTROL_BIT = 0x00000002;
            const TESSELLATION_EVALUATION_BIT = 0x00000004;
            const GEOMETRY_BIT = 0x00000008;
            const FRAGMENT_BIT = 0x00000010;
            const COMPUTE_BIT = 0x00000020;
            const ALL_GRAPHICS = 0x0000001F;
            const ALL = 0x7FFFFFFF;
            const RAYGEN_BIT_NV = 0x00000100;
            const ANY_HIT_BIT_NV = 0x00000200;
            const CLOSEST_HIT_BIT_NV = 0x00000400;
            const MISS_BIT_NV = 0x00000800;
            const INTERSECTION_BIT_NV = 0x00001000;
            const CALLABLE_BIT_NV = 0x00002000;
            const TASK_BIT_NV = 0x00000040;
            const MESH_BIT_NV = 0x00000080;
        }
    }

pub type VkCullModeFlags = VkCullModeFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkCullModeFlagBits: u32 {
		    const NONE = 0;
            const FRONT_BIT = 0x00000001;
            const BACK_BIT = 0x00000002;
            const FRONT_AND_BACK = 0x00000003;
	    }
    }

pub type VkSampleCountFlags = VkSampleCountFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkSampleCountFlagBits: u32 {
		    const COUNT_1_BIT = 0x00000001;
            const COUNT_2_BIT = 0x00000002;
            const COUNT_4_BIT = 0x00000004;
            const COUNT_8_BIT = 0x00000008;
            const COUNT_16_BIT = 0x00000010;
            const COUNT_32_BIT = 0x00000020;
            const COUNT_64_BIT = 0x00000040;
	    }
    }

pub type VkColorComponentFlags = VkColorComponentFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkColorComponentFlagBits: u32 {
		    const R_BIT = 0x00000001;
            const G_BIT = 0x00000002;
            const B_BIT = 0x00000004;
            const A_BIT = 0x00000008;
	    }
    }

pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkPipelineCreateFlagBits: u32 {
		    const DISABLE_OPTIMIZATION_BIT = 0x00000001;
            const ALLOW_DERIVATIVES_BIT = 0x00000002;
            const DERIVATIVE_BIT = 0x00000004;
            const VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 0x00000008;
            const DISPATCH_BASE = 0x00000010;
            const DEFER_COMPILE_BIT_NV = 0x00000020;
	    }
    }

pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkAttachmentDescriptionFlagBits: u32 {
		    const MAY_ALIAS_BIT = 0x00000001;
	    }
    }

pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkSubpassDescriptionFlagBits: u32 {
    	    const PER_VIEW_ATTRIBUTES_BIT_NVX = 0x00000001;
            const PER_VIEW_POSITION_X_ONLY_BIT_NVX = 0x00000002;
        }
    }

pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkPipelineStageFlagBits: u32 {
    	    const TOP_OF_PIPE_BIT = 0x00000001;
            const DRAW_INDIRECT_BIT = 0x00000002;
            const VERTEX_INPUT_BIT = 0x00000004;
            const VERTEX_SHADER_BIT = 0x00000008;
            const TESSELLATION_CONTROL_SHADER_BIT = 0x00000010;
            const TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020;
            const GEOMETRY_SHADER_BIT = 0x00000040;
            const FRAGMENT_SHADER_BIT = 0x00000080;
            const EARLY_FRAGMENT_TESTS_BIT = 0x00000100;
            const LATE_FRAGMENT_TESTS_BIT = 0x00000200;
            const COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400;
            const COMPUTE_SHADER_BIT = 0x00000800;
            const TRANSFER_BIT = 0x00001000;
            const BOTTOM_OF_PIPE_BIT = 0x00002000;
            const HOST_BIT = 0x00004000;
            const ALL_GRAPHICS_BIT = 0x00008000;
            const ALL_COMMANDS_BIT = 0x00010000;
            const TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000;
            const CONDITIONAL_RENDERING_BIT_EXT = 0x00040000;
            const COMMAND_PROCESS_BIT_NVX = 0x00020000;
            const SHADING_RATE_IMAGE_BIT_NV = 0x00400000;
            const RAY_TRACING_SHADER_BIT_NV = 0x00200000;
            const ACCELERATION_STRUCTURE_BUILD_BIT_NV = 0x02000000;
            const TASK_SHADER_BIT_NV = 0x00080000;
            const MESH_SHADER_BIT_NV = 0x00100000;
        }
    }

pub type VkAccessFlags = VkAccessFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkAccessFlagBits: u32 {
    	    const INDIRECT_COMMAND_READ_BIT = 0x00000001;
            const INDEX_READ_BIT = 0x00000002;
            const VERTEX_ATTRIBUTE_READ_BIT = 0x00000004;
            const UNIFORM_READ_BIT = 0x00000008;
            const INPUT_ATTACHMENT_READ_BIT = 0x00000010;
            const SHADER_READ_BIT = 0x00000020;
            const SHADER_WRITE_BIT = 0x00000040;
            const COLOR_ATTACHMENT_READ_BIT = 0x00000080;
            const COLOR_ATTACHMENT_WRITE_BIT = 0x00000100;
            const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200;
            const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400;
            const TRANSFER_READ_BIT = 0x00000800;
            const TRANSFER_WRITE_BIT = 0x00001000;
            const HOST_READ_BIT = 0x00002000;
            const HOST_WRITE_BIT = 0x00004000;
            const MEMORY_READ_BIT = 0x00008000;
            const MEMORY_WRITE_BIT = 0x00010000;
            const TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000;
            const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000;
            const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000;
            const CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000;
            const COMMAND_PROCESS_READ_BIT_NVX = 0x00020000;
            const COMMAND_PROCESS_WRITE_BIT_NVX = 0x00040000;
            const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000;
            const SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000;
            const ACCELERATION_STRUCTURE_READ_BIT_NV = 0x00200000;
            const ACCELERATION_STRUCTURE_WRITE_BIT_NV = 0x00400000;
        }
    }

pub type VkDependencyFlags = VkDependencyFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDependencyFlagBits: u32 {
    	    const BY_REGION_BIT = 0x00000001;
    	    const VIEW_LOCAL_BIT = 0x00000002;
            const DEVICE_GROUP_BIT = 0x00000004;
        }
    }

pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkCommandPoolCreateFlagBits: u32 {
    	    const TRANSIENT_BIT = 0x00000001;
            const RESET_COMMAND_BUFFER_BIT = 0x00000002;
            const PROTECTED_BIT = 0x00000004;
        }
    }

pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkCommandBufferUsageFlagBits: u32 {
    	    const ONE_TIME_SUBMIT_BIT = 0x00000001;
            const RENDER_PASS_CONTINUE_BIT = 0x00000002;
            const SIMULTANEOUS_USE_BIT = 0x00000004;
        }
    }

pub type VkQueryControlFlags = VkQueryControlFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkQueryControlFlagBits: u32 {
    	    const PRECISE_BIT = 0x00000001;
        }
    }

pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkQueryPipelineStatisticFlagBits: u32 {
    	    const INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001;
            const INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002;
            const VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004;
            const GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008;
            const GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010;
            const CLIPPING_INVOCATIONS_BIT = 0x00000020;
            const CLIPPING_PRIMITIVES_BIT = 0x00000040;
            const FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080;
            const TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100;
            const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200;
            const COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400;
        }
    }

pub type VkFenceCreateFlags = VkFenceCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkFenceCreateFlagBits: u32 {
    	    const SIGNALED_BIT = 0x00000001;
        }
    }

pub type VkBufferCreateFlags = VkBufferCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkBufferCreateFlagBits: u32 {
    	    const SPARSE_BINDING_BIT = 0x00000001;
            const SPARSE_RESIDENCY_BIT = 0x00000002;
            const SPARSE_ALIASED_BIT = 0x00000004;
            const PROTECTED_BIT = 0x00000008;
        }
    }

pub type VkBufferUsageFlags = VkBufferUsageFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkBufferUsageFlagBits: u32 {
    	    const TRANSFER_SRC_BIT = 0x00000001;
            const TRANSFER_DST_BIT = 0x00000002;
            const UNIFORM_TEXEL_BUFFER_BIT = 0x00000004;
            const STORAGE_TEXEL_BUFFER_BIT = 0x00000008;
            const UNIFORM_BUFFER_BIT = 0x00000010;
            const STORAGE_BUFFER_BIT = 0x00000020;
            const INDEX_BUFFER_BIT = 0x00000040;
            const VERTEX_BUFFER_BIT = 0x00000080;
            const INDIRECT_BUFFER_BIT = 0x00000100;
            const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x00000800;
            const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x00001000;
            const CONDITIONAL_RENDERING_BIT_EXT = 0x00000200;
            const RAY_TRACING_BIT_NV = 0x00000400;
        }
    }

pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkMemoryHeapFlagBits: u32 {
    	    const DEVICE_LOCAL_BIT = 0x00000001;
            const MULTI_INSTANCE_BIT = 0x00000002;
        }
    }

pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkMemoryPropertyFlagBits: u32 {
    	    const DEVICE_LOCAL_BIT = 0x00000001;
            const HOST_VISIBLE_BIT = 0x00000002;
            const HOST_COHERENT_BIT = 0x00000004;
            const HOST_CACHED_BIT = 0x00000008;
            const LAZILY_ALLOCATED_BIT = 0x00000010;
            const PROTECTED_BIT = 0x00000020;
        }
    }

pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDescriptorSetLayoutCreateFlagBits: u32 {
    	    const PUSH_DESCRIPTOR_BIT_KHR = 0x00000001;
            const UPDATE_AFTER_BIND_POOL_BIT_EXT = 0x00000002;
        }
    }

pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDescriptorPoolCreateFlagBits: u32 {
    	    const FREE_DESCRIPTOR_SET_BIT = 0x00000001;
            const UPDATE_AFTER_BIND_BIT_EXT = 0x00000002;
        }
    }

pub type VkImageCreateFlags = VkImageCreateFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkImageCreateFlagBits: u32 {
    	    const SPARSE_BINDING_BIT = 0x00000001;
            const SPARSE_RESIDENCY_BIT = 0x00000002;
            const SPARSE_ALIASED_BIT = 0x00000004;
            const MUTABLE_FORMAT_BIT = 0x00000008;
            const CUBE_COMPATIBLE_BIT = 0x00000010;
            const ALIAS_BIT = 0x00000400;
            const SPLIT_INSTANCE_BIND_REGIONS_BIT = 0x00000040;
            const TWO_DIMENSION_ARRAY_COMPATIBLE_BIT = 0x00000020;
            const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 0x00000080;
            const EXTENDED_USAGE_BIT = 0x00000100;
            const PROTECTED_BIT = 0x00000800;
            const DISJOINT_BIT = 0x00000200;
            const CORNER_SAMPLED_BIT_NV = 0x00002000;
            const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 0x00001000;
        }
    }

pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkFormatFeatureFlagBits: u32 {
    	    const SAMPLED_IMAGE_BIT = 0x00000001;
            const STORAGE_IMAGE_BIT = 0x00000002;
            const STORAGE_IMAGE_ATOMIC_BIT = 0x00000004;
            const UNIFORM_TEXEL_BUFFER_BIT = 0x00000008;
            const STORAGE_TEXEL_BUFFER_BIT = 0x00000010;
            const STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020;
            const VERTEX_BUFFER_BIT = 0x00000040;
            const COLOR_ATTACHMENT_BIT = 0x00000080;
            const COLOR_ATTACHMENT_BLEND_BIT = 0x00000100;
            const DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200;
            const BLIT_SRC_BIT = 0x00000400;
            const BLIT_DST_BIT = 0x00000800;
            const SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000;
            const TRANSFER_SRC_BIT = 0x00004000;
            const TRANSFER_DST_BIT = 0x00008000;
            const MIDPOINT_CHROMA_SAMPLES_BIT = 0x00020000;
            const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 0x00040000;
            const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 0x00080000;
            const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 0x00100000;
            const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 0x00200000;
            const DISJOINT_BIT = 0x00400000;
            const COSITED_CHROMA_SAMPLES_BIT = 0x00800000;
            const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000;
            const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = 0x00010000;
        }
    }

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkInternalAllocationType{
    EXECUTABLE = 0,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkBool32{
    TRUE = 1,
    FALSE = 0,
}
impl Default for VkBool32{
    fn default() -> Self {
        VkBool32::FALSE
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkResult{
    SUCCESS = 0,
    NOT_READY = 1,
    TIMEOUT = 2,
    EVENT_SET = 3,
    EVENT_RESET = 4,
    INCOMPLETE = 5,
    ERROR_OUT_OF_HOST_MEMORY = -1,
    ERROR_OUT_OF_DEVICE_MEMORY = -2,
    ERROR_INITIALIZATION_FAILED = -3,
    ERROR_DEVICE_LOST = -4,
    ERROR_MEMORY_MAP_FAILED = -5,
    ERROR_LAYER_NOT_PRESENT = -6,
    ERROR_EXTENSION_NOT_PRESENT = -7,
    ERROR_FEATURE_NOT_PRESENT = -8,
    ERROR_INCOMPATIBLE_DRIVER = -9,
    ERROR_TOO_MANY_OBJECTS = -10,
    ERROR_FORMAT_NOT_SUPPORTED = -11,
    ERROR_FRAGMENTED_POOL = -12,
    ERROR_OUT_OF_POOL_MEMORY = -1000069000,
    ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,

    // VK_KHR_swapchain
    SUBOPTIMAL_KHR = 1000001003,
    ERROR_OUT_OF_DATE_KHR = -1000001004,

    // VK_KHR_surface
    SURFACE_LOST_KHR = -1000000000,
    NATIVE_WINDOW_IN_USE_KHR = -1000000001,


    // VK_EXT_debug_utils
    ERROR_VALIDATION_FAILED_EXT = -1000011001,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkStructureType{
    APPLICATION_INFO = 0,
    INSTANCE_CREATE_INFO = 1,
    DEVICE_QUEUE_CREATE_INFO = 2,
    DEVICE_CREATE_INFO = 3,
    SUBMIT_INFO = 4,
    MEMORY_ALLOCATE_INFO = 5,
    MAPPED_MEMORY_RANGE = 6,
    FENCE_CREATE_INFO = 8,
    SEMAPHORE_CREATE_INFO = 9,
    BUFFER_CREATE_INFO = 12,
    BUFFER_VIEW_CREATE_INFO = 13,
    IMAGE_CREATE_INFO = 14,
    IMAGE_VIEW_CREATE_INFO = 15,
    SHADER_MODULE_CREATE_INFO = 16,
    PIPELINE_CACHE_CREATE_INFO = 17,
    PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    GRAPHICS_PIPELINE_CREATE_INFO = 28,
    PIPELINE_LAYOUT_CREATE_INFO = 30,
    SAMPLER_CREATE_INFO = 31,
    DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    DESCRIPTOR_POOL_CREATE_INFO = 33,
    DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    WRITE_DESCRIPTOR_SET = 35,
    FRAMEBUFFER_CREATE_INFO = 37,
    RENDER_PASS_CREATE_INFO = 38,
    COMMAND_POOL_CREATE_INFO = 39,
    COMMAND_BUFFER_ALLOCATE_INFO = 40,
    COMMAND_BUFFER_BEGIN_INFO = 42,
    RENDER_PASS_BEGIN_INFO = 43,
    BUFFER_MEMORY_BARRIER = 44,
    IMAGE_MEMORY_BARRIER = 45,
    MEMORY_BARRIER = 46,
    PHYSICAL_DEVICE_PROPERTIES_2 = 1000059001,
    PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO = 1000117003,
    BIND_IMAGE_MEMORY_INFO = 1000157001,

    // VK_KHR_swapchain
    SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    PRESENT_INFO_KHR = 1000001001,
    DEVICE_GROUP_PRESENT_CAPABILITIES_KHR = 1000060007,
    IMAGE_SWAPCHAIN_CREATE_INFO_KHR = 1000060008,
    BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR = 1000060009,
    ACQUIRE_NEXT_IMAGE_INFO_KHR = 1000060010,
    DEVICE_GROUP_PRESENT_INFO_KHR = 1000060011,
    DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR = 1000060012,

    // VK_KHR_win32_surface
    WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,

    // VK_EXT_debug_utils
    DEBUG_UTILS_OBJECT_NAME_INFO_EXT = 1000128000,
    DEBUG_UTILS_OBJECT_TAG_INFO_EXT = 1000128001,
    DEBUG_UTILS_LABEL_EXT = 1000128002,
    DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT = 1000128003,
    DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT = 1000128004,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkObjectType{
    UNKNOWN = 0,
    INSTANCE = 1,
    PHYSICAL_DEVICE = 2,
    DEVICE = 3,
    QUEUE = 4,
    SEMAPHORE = 5,
    COMMAND_BUFFER = 6,
    FENCE = 7,
    DEVICE_MEMORY = 8,
    BUFFER = 9,
    IMAGE = 10,
    EVENT = 11,
    QUERY_POOL = 12,
    BUFFER_VIEW = 13,
    IMAGE_VIEW = 14,
    SHADER_MODULE = 15,
    PIPELINE_CACHE = 16,
    PIPELINE_LAYOUT = 17,
    RENDER_PASS = 18,
    PIPELINE = 19,
    DESCRIPTOR_SET_LAYOUT = 20,
    SAMPLER = 21,
    DESCRIPTOR_POOL = 22,
    DESCRIPTOR_SET = 23,
    FRAMEBUFFER = 24,
    COMMAND_POOL = 25,
    SAMPLER_YCBCR_CONVERSION = 1000156000,
    DESCRIPTOR_UPDATE_TEMPLATE = 1000085000,
    SURFACE_KHR = 1000000000,
    SWAPCHAIN_KHR = 1000001000,
    DISPLAY_KHR = 1000002000,
    DISPLAY_MODE_KHR = 1000002001,
    DEBUG_REPORT_CALLBACK_EXT = 1000011000,
    OBJECT_TABLE_NVX = 1000086000,
    INDIRECT_COMMANDS_LAYOUT_NVX = 1000086001,
    DEBUG_UTILS_MESSENGER_EXT = 1000128000,
    VALIDATION_CACHE_EXT = 1000160000,
    ACCELERATION_STRUCTURE_NV = 1000165000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkPhysicalDeviceType{
    OTHER = 0,
    INTEGRATED_GPU = 1,
    DISCRETE_GPU = 2,
    VIRTUAL_GPU = 3,
    CPU = 4,
}
impl Default for VkPhysicalDeviceType{
    fn default() -> Self {
        VkPhysicalDeviceType::OTHER
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum VkFormat{
    UNDEFINED = 0,
    R4G4_UNORM_PACK8 = 1,
    R4G4B4A4_UNORM_PACK16 = 2,
    B4G4R4A4_UNORM_PACK16 = 3,
    R5G6B5_UNORM_PACK16 = 4,
    B5G6R5_UNORM_PACK16 = 5,
    R5G5B5A1_UNORM_PACK16 = 6,
    B5G5R5A1_UNORM_PACK16 = 7,
    A1R5G5B5_UNORM_PACK16 = 8,
    R8_UNORM = 9,
    R8_SNORM = 10,
    R8_USCALED = 11,
    R8_SSCALED = 12,
    R8_UINT = 13,
    R8_SINT = 14,
    R8_SRGB = 15,
    R8G8_UNORM = 16,
    R8G8_SNORM = 17,
    R8G8_USCALED = 18,
    R8G8_SSCALED = 19,
    R8G8_UINT = 20,
    R8G8_SINT = 21,
    R8G8_SRGB = 22,
    R8G8B8_UNORM = 23,
    R8G8B8_SNORM = 24,
    R8G8B8_USCALED = 25,
    R8G8B8_SSCALED = 26,
    R8G8B8_UINT = 27,
    R8G8B8_SINT = 28,
    R8G8B8_SRGB = 29,
    B8G8R8_UNORM = 30,
    B8G8R8_SNORM = 31,
    B8G8R8_USCALED = 32,
    B8G8R8_SSCALED = 33,
    B8G8R8_UINT = 34,
    B8G8R8_SINT = 35,
    B8G8R8_SRGB = 36,
    R8G8B8A8_UNORM = 37,
    R8G8B8A8_SNORM = 38,
    R8G8B8A8_USCALED = 39,
    R8G8B8A8_SSCALED = 40,
    R8G8B8A8_UINT = 41,
    R8G8B8A8_SINT = 42,
    R8G8B8A8_SRGB = 43,
    B8G8R8A8_UNORM = 44,
    B8G8R8A8_SNORM = 45,
    B8G8R8A8_USCALED = 46,
    B8G8R8A8_SSCALED = 47,
    B8G8R8A8_UINT = 48,
    B8G8R8A8_SINT = 49,
    B8G8R8A8_SRGB = 50,
    A8B8G8R8_UNORM_PACK32 = 51,
    A8B8G8R8_SNORM_PACK32 = 52,
    A8B8G8R8_USCALED_PACK32 = 53,
    A8B8G8R8_SSCALED_PACK32 = 54,
    A8B8G8R8_UINT_PACK32 = 55,
    A8B8G8R8_SINT_PACK32 = 56,
    A8B8G8R8_SRGB_PACK32 = 57,
    A2R10G10B10_UNORM_PACK32 = 58,
    A2R10G10B10_SNORM_PACK32 = 59,
    A2R10G10B10_USCALED_PACK32 = 60,
    A2R10G10B10_SSCALED_PACK32 = 61,
    A2R10G10B10_UINT_PACK32 = 62,
    A2R10G10B10_SINT_PACK32 = 63,
    A2B10G10R10_UNORM_PACK32 = 64,
    A2B10G10R10_SNORM_PACK32 = 65,
    A2B10G10R10_USCALED_PACK32 = 66,
    A2B10G10R10_SSCALED_PACK32 = 67,
    A2B10G10R10_UINT_PACK32 = 68,
    A2B10G10R10_SINT_PACK32 = 69,
    R16_UNORM = 70,
    R16_SNORM = 71,
    R16_USCALED = 72,
    R16_SSCALED = 73,
    R16_UINT = 74,
    R16_SINT = 75,
    R16_SFLOAT = 76,
    R16G16_UNORM = 77,
    R16G16_SNORM = 78,
    R16G16_USCALED = 79,
    R16G16_SSCALED = 80,
    R16G16_UINT = 81,
    R16G16_SINT = 82,
    R16G16_SFLOAT = 83,
    R16G16B16_UNORM = 84,
    R16G16B16_SNORM = 85,
    R16G16B16_USCALED = 86,
    R16G16B16_SSCALED = 87,
    R16G16B16_UINT = 88,
    R16G16B16_SINT = 89,
    R16G16B16_SFLOAT = 90,
    R16G16B16A16_UNORM = 91,
    R16G16B16A16_SNORM = 92,
    R16G16B16A16_USCALED = 93,
    R16G16B16A16_SSCALED = 94,
    R16G16B16A16_UINT = 95,
    R16G16B16A16_SINT = 96,
    R16G16B16A16_SFLOAT = 97,
    R32_UINT = 98,
    R32_SINT = 99,
    R32_SFLOAT = 100,
    R32G32_UINT = 101,
    R32G32_SINT = 102,
    R32G32_SFLOAT = 103,
    R32G32B32_UINT = 104,
    R32G32B32_SINT = 105,
    R32G32B32_SFLOAT = 106,
    R32G32B32A32_UINT = 107,
    R32G32B32A32_SINT = 108,
    R32G32B32A32_SFLOAT = 109,
    R64_UINT = 110,
    R64_SINT = 111,
    R64_SFLOAT = 112,
    R64G64_UINT = 113,
    R64G64_SINT = 114,
    R64G64_SFLOAT = 115,
    R64G64B64_UINT = 116,
    R64G64B64_SINT = 117,
    R64G64B64_SFLOAT = 118,
    R64G64B64A64_UINT = 119,
    R64G64B64A64_SINT = 120,
    R64G64B64A64_SFLOAT = 121,
    B10G11R11_UFLOAT_PACK32 = 122,
    E5B9G9R9_UFLOAT_PACK32 = 123,
    D16_UNORM = 124,
    X8_D24_UNORM_PACK32 = 125,
    D32_SFLOAT = 126,
    S8_UINT = 127,
    D16_UNORM_S8_UINT = 128,
    D24_UNORM_S8_UINT = 129,
    D32_SFLOAT_S8_UINT = 130,
    BC1_RGB_UNORM_BLOCK = 131,
    BC1_RGB_SRGB_BLOCK = 132,
    BC1_RGBA_UNORM_BLOCK = 133,
    BC1_RGBA_SRGB_BLOCK = 134,
    BC2_UNORM_BLOCK = 135,
    BC2_SRGB_BLOCK = 136,
    BC3_UNORM_BLOCK = 137,
    BC3_SRGB_BLOCK = 138,
    BC4_UNORM_BLOCK = 139,
    BC4_SNORM_BLOCK = 140,
    BC5_UNORM_BLOCK = 141,
    BC5_SNORM_BLOCK = 142,
    BC6H_UFLOAT_BLOCK = 143,
    BC6H_SFLOAT_BLOCK = 144,
    BC7_UNORM_BLOCK = 145,
    BC7_SRGB_BLOCK = 146,
    ETC2_R8G8B8_UNORM_BLOCK = 147,
    ETC2_R8G8B8_SRGB_BLOCK = 148,
    ETC2_R8G8B8A1_UNORM_BLOCK = 149,
    ETC2_R8G8B8A1_SRGB_BLOCK = 150,
    ETC2_R8G8B8A8_UNORM_BLOCK = 151,
    ETC2_R8G8B8A8_SRGB_BLOCK = 152,
    EAC_R11_UNORM_BLOCK = 153,
    EAC_R11_SNORM_BLOCK = 154,
    EAC_R11G11_UNORM_BLOCK = 155,
    EAC_R11G11_SNORM_BLOCK = 156,
    ASTC_4x4_UNORM_BLOCK = 157,
    ASTC_4x4_SRGB_BLOCK = 158,
    ASTC_5x4_UNORM_BLOCK = 159,
    ASTC_5x4_SRGB_BLOCK = 160,
    ASTC_5x5_UNORM_BLOCK = 161,
    ASTC_5x5_SRGB_BLOCK = 162,
    ASTC_6x5_UNORM_BLOCK = 163,
    ASTC_6x5_SRGB_BLOCK = 164,
    ASTC_6x6_UNORM_BLOCK = 165,
    ASTC_6x6_SRGB_BLOCK = 166,
    ASTC_8x5_UNORM_BLOCK = 167,
    ASTC_8x5_SRGB_BLOCK = 168,
    ASTC_8x6_UNORM_BLOCK = 169,
    ASTC_8x6_SRGB_BLOCK = 170,
    ASTC_8x8_UNORM_BLOCK = 171,
    ASTC_8x8_SRGB_BLOCK = 172,
    ASTC_10x5_UNORM_BLOCK = 173,
    ASTC_10x5_SRGB_BLOCK = 174,
    ASTC_10x6_UNORM_BLOCK = 175,
    ASTC_10x6_SRGB_BLOCK = 176,
    ASTC_10x8_UNORM_BLOCK = 177,
    ASTC_10x8_SRGB_BLOCK = 178,
    ASTC_10x10_UNORM_BLOCK = 179,
    ASTC_10x10_SRGB_BLOCK = 180,
    ASTC_12x10_UNORM_BLOCK = 181,
    ASTC_12x10_SRGB_BLOCK = 182,
    ASTC_12x12_UNORM_BLOCK = 183,
    ASTC_12x12_SRGB_BLOCK = 184,
    G8B8G8R8_422_UNORM = 1000156000,
    B8G8R8G8_422_UNORM = 1000156001,
    G8_B8_R8_3PLANE_420_UNORM = 1000156002,
    G8_B8R8_2PLANE_420_UNORM = 1000156003,
    G8_B8_R8_3PLANE_422_UNORM = 1000156004,
    G8_B8R8_2PLANE_422_UNORM = 1000156005,
    G8_B8_R8_3PLANE_444_UNORM = 1000156006,
    R10X6_UNORM_PACK16 = 1000156007,
    R10X6G10X6_UNORM_2PACK16 = 1000156008,
    R10X6G10X6B10X6A10X6_UNORM_4PACK16 = 1000156009,
    G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 = 1000156010,
    B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 = 1000156011,
    G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 = 1000156012,
    G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 = 1000156013,
    G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 = 1000156014,
    G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 = 1000156015,
    G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 = 1000156016,
    R12X4_UNORM_PACK16 = 1000156017,
    R12X4G12X4_UNORM_2PACK16 = 1000156018,
    R12X4G12X4B12X4A12X4_UNORM_4PACK16 = 1000156019,
    G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 = 1000156020,
    B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 = 1000156021,
    G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 = 1000156022,
    G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 = 1000156023,
    G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 = 1000156024,
    G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 = 1000156025,
    G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 = 1000156026,
    G16B16G16R16_422_UNORM = 1000156027,
    B16G16R16G16_422_UNORM = 1000156028,
    G16_B16_R16_3PLANE_420_UNORM = 1000156029,
    G16_B16R16_2PLANE_420_UNORM = 1000156030,
    G16_B16_R16_3PLANE_422_UNORM = 1000156031,
    G16_B16R16_2PLANE_422_UNORM = 1000156032,
    G16_B16_R16_3PLANE_444_UNORM = 1000156033,
    PVRTC1_2BPP_UNORM_BLOCK_IMG = 1000054000,
    PVRTC1_4BPP_UNORM_BLOCK_IMG = 1000054001,
    PVRTC2_2BPP_UNORM_BLOCK_IMG = 1000054002,
    PVRTC2_4BPP_UNORM_BLOCK_IMG = 1000054003,
    PVRTC1_2BPP_SRGB_BLOCK_IMG = 1000054004,
    PVRTC1_4BPP_SRGB_BLOCK_IMG = 1000054005,
    PVRTC2_2BPP_SRGB_BLOCK_IMG = 1000054006,
    PVRTC2_4BPP_SRGB_BLOCK_IMG = 1000054007,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkSharingMode{
    EXCLUSIVE = 0,
    CONCURRENT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkImageViewType{
    ONE_DIMENSION = 0,
    TWO_DIMENSION = 1,
    THREE_DIMENSION = 2,
    CUBE = 3,
    ONE_DIMENSION_ARRAY = 4,
    TWO_DIMENSION_ARRAY = 5,
    CUBE_ARRAY = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkComponentSwizzle{
    IDENTITY = 0,
    ZERO = 1,
    ONE = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum VkVertexInputRate{
    VERTEX = 0,
    INSTANCE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkPrimitiveTopology{
    POINT_LIST = 0,
    LINE_LIST = 1,
    LINE_STRIP = 2,
    TRIANGLE_LIST = 3,
    TRIANGLE_STRIP = 4,
    TRIANGLE_FAN = 5,
    LINE_LIST_WITH_ADJACENCY = 6,
    LINE_STRIP_WITH_ADJACENCY = 7,
    TRIANGLE_LIST_WITH_ADJACENCY = 8,
    TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    PATCH_LIST = 10,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkPolygonMode{
    FILL = 0,
    LINE = 1,
    POINT = 2,
    FILL_RECTANGLE_NV = 1000153000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkFrontFace{
    COUNTER_CLOCKWISE = 0,
    CLOCKWISE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkBlendFactor{
    ZERO = 0,
    ONE = 1,
    SRC_COLOR = 2,
    ONE_MINUS_SRC_COLOR = 3,
    DST_COLOR = 4,
    ONE_MINUS_DST_COLOR = 5,
    SRC_ALPHA = 6,
    ONE_MINUS_SRC_ALPHA = 7,
    DST_ALPHA = 8,
    ONE_MINUS_DST_ALPHA = 9,
    CONSTANT_COLOR = 10,
    ONE_MINUS_CONSTANT_COLOR = 11,
    CONSTANT_ALPHA = 12,
    ONE_MINUS_CONSTANT_ALPHA = 13,
    SRC_ALPHA_SATURATE = 14,
    SRC1_COLOR = 15,
    ONE_MINUS_SRC1_COLOR = 16,
    SRC1_ALPHA = 17,
    ONE_MINUS_SRC1_ALPHA = 18,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkBlendOp{
    ADD = 0,
    SUBTRACT = 1,
    REVERSE_SUBTRACT = 2,
    MIN = 3,
    MAX = 4,
    ZERO_EXT = 1000148000,
    SRC_EXT = 1000148001,
    DST_EXT = 1000148002,
    SRC_OVER_EXT = 1000148003,
    DST_OVER_EXT = 1000148004,
    SRC_IN_EXT = 1000148005,
    DST_IN_EXT = 1000148006,
    SRC_OUT_EXT = 1000148007,
    DST_OUT_EXT = 1000148008,
    SRC_ATOP_EXT = 1000148009,
    DST_ATOP_EXT = 1000148010,
    XOR_EXT = 1000148011,
    MULTIPLY_EXT = 1000148012,
    SCREEN_EXT = 1000148013,
    OVERLAY_EXT = 1000148014,
    DARKEN_EXT = 1000148015,
    LIGHTEN_EXT = 1000148016,
    COLORDODGE_EXT = 1000148017,
    COLORBURN_EXT = 1000148018,
    HARDLIGHT_EXT = 1000148019,
    SOFTLIGHT_EXT = 1000148020,
    DIFFERENCE_EXT = 1000148021,
    EXCLUSION_EXT = 1000148022,
    INVERT_EXT = 1000148023,
    INVERT_RGB_EXT = 1000148024,
    LINEARDODGE_EXT = 1000148025,
    LINEARBURN_EXT = 1000148026,
    VIVIDLIGHT_EXT = 1000148027,
    LINEARLIGHT_EXT = 1000148028,
    PINLIGHT_EXT = 1000148029,
    HARDMIX_EXT = 1000148030,
    HSL_HUE_EXT = 1000148031,
    HSL_SATURATION_EXT = 1000148032,
    HSL_COLOR_EXT = 1000148033,
    HSL_LUMINOSITY_EXT = 1000148034,
    PLUS_EXT = 1000148035,
    PLUS_CLAMPED_EXT = 1000148036,
    PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    PLUS_DARKER_EXT = 1000148038,
    MINUS_EXT = 1000148039,
    MINUS_CLAMPED_EXT = 1000148040,
    CONTRAST_EXT = 1000148041,
    INVERT_OVG_EXT = 1000148042,
    RED_EXT = 1000148043,
    GREEN_EXT = 1000148044,
    BLUE_EXT = 1000148045,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkLogicOp{
    CLEAR = 0,
    AND = 1,
    AND_REVERSE = 2,
    COPY = 3,
    AND_INVERTED = 4,
    NO_OP = 5,
    XOR = 6,
    OR = 7,
    NOR = 8,
    EQUIVALENT = 9,
    INVERT = 10,
    OR_REVERSE = 11,
    COPY_INVERTED = 12,
    OR_INVERTED = 13,
    NAND = 14,
    SET = 15,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkDynamicState{
    VIEWPORT = 0,
    SCISSOR = 1,
    LINE_WIDTH = 2,
    DEPTH_BIAS = 3,
    BLEND_CONSTANTS = 4,
    DEPTH_BOUNDS = 5,
    STENCIL_COMPARE_MASK = 6,
    STENCIL_WRITE_MASK = 7,
    STENCIL_REFERENCE = 8,
    VIEWPORT_W_SCALING_NV = 1000087000,
    DISCARD_RECTANGLE_EXT = 1000099000,
    SAMPLE_LOCATIONS_EXT = 1000143000,
    VIEWPORT_SHADING_RATE_PALETTE_NV = 1000164004,
    VIEWPORT_COARSE_SAMPLE_ORDER_NV = 1000164006,
    EXCLUSIVE_SCISSOR_NV = 1000205001,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkCompareOp{
    NEVER = 0,
    LESS = 1,
    EQUAL = 2,
    LESS_OR_EQUAL = 3,
    GREATER = 4,
    NOT_EQUAL = 5,
    GREATER_OR_EQUAL = 6,
    ALWAYS = 7,
}
impl Default for VkCompareOp{
    fn default() -> Self {
        VkCompareOp::NEVER
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkStencilOp{
    KEEP = 0,
    ZERO = 1,
    REPLACE = 2,
    INCREMENT_AND_CLAMP = 3,
    DECREMENT_AND_CLAMP = 4,
    INVERT = 5,
    INCREMENT_AND_WRAP = 6,
    DECREMENT_AND_WRAP = 7,
}
impl Default for VkStencilOp{
    fn default() -> Self {
        VkStencilOp::KEEP
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkAttachmentLoadOp{
    LOAD = 0,
    CLEAR = 1,
    DONT_CARE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkAttachmentStoreOp{
    STORE = 0,
    DONT_CARE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkImageLayout{
    UNDEFINED = 0,
    GENERAL = 1,
    COLOR_ATTACHMENT_OPTIMAL = 2,
    DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    SHADER_READ_ONLY_OPTIMAL = 5,
    TRANSFER_SRC_OPTIMAL = 6,
    TRANSFER_DST_OPTIMAL = 7,
    PREINITIALIZED = 8,
    DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
    DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,

    // VK_KHR_swapchain
    PRESENT_SRC_KHR = 1000001002,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkPipelineBindPoint{
    GRAPHICS = 0,
    COMPUTE = 1,
    RAY_TRACING_NV = 1000165000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkCommandBufferLevel{
    PRIMARY = 0,
    SECONDARY = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkSubpassContents{
    INLINE = 0,
    SECONDARY_COMMAND_BUFFERS = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkIndexType{
    UINT16 = 0,
    UINT32 = 1,
    NONE_NV = 1000165000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkDescriptorType{
    SAMPLER = 0,
    COMBINED_IMAGE_SAMPLER = 1,
    SAMPLED_IMAGE = 2,
    STORAGE_IMAGE = 3,
    UNIFORM_TEXEL_BUFFER = 4,
    STORAGE_TEXEL_BUFFER = 5,
    UNIFORM_BUFFER = 6,
    STORAGE_BUFFER = 7,
    UNIFORM_BUFFER_DYNAMIC = 8,
    STORAGE_BUFFER_DYNAMIC = 9,
    INPUT_ATTACHMENT = 10,
    INLINE_UNIFORM_BLOCK_EXT = 1000138000,
    ACCELERATION_STRUCTURE_NV = 1000165000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkImageType{
    ONE_DIMENSION = 0,
    TWO_DIMENSION = 1,
    THREE_DIMENSION = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkImageTiling{
    OPTIMAL = 0,
    LINEAR = 1,
    DRM_FORMAT_MODIFIER_EXT = 1000158000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkFilter{
    NEAREST = 0,
    LINEAR = 1,
    CUBIC_IMG = 1000015000,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkSamplerMipmapMode{
    NEAREST = 0,
    LINEAR = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkSamplerAddressMode{
    REPEAT = 0,
    MIRRORED_REPEAT = 1,
    CLAMP_TO_EDGE = 2,
    CLAMP_TO_BORDER = 3,
    MIRROR_CLAMP_TO_EDGE = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkBorderColor{
    FLOAT_TRANSPARENT_BLACK = 0,
    INT_TRANSPARENT_BLACK = 1,
    FLOAT_OPAQUE_BLACK = 2,
    INT_OPAQUE_BLACK = 3,
    FLOAT_OPAQUE_WHITE = 4,
    INT_OPAQUE_WHITE = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkTessellationDomainOrigin{
    UPPER_LEFT = 0,
    LOWER_LEFT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkSystemAllocationScope{
    COMMAND = 0,
    OBJECT = 1,
    CACHE = 2,
    DEVICE = 3,
    INSTANCE = 4,
}

pub const VK_LOD_CLAMP_NONE: f32 = 1000f32;
pub const VK_REMAINING_MIP_LEVELS: u32 = 0xFFFF_FFFF;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 0xFFFF_FFFF;
pub const VK_WHOLE_SIZE: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const VK_ATTACHMENT_UNUSED: u32 = 0xFFFF_FFFF;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xFFFF_FFFF;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xFFFF_FFFF;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;

handle!(VkInstance, DispatchableHandle);
handle!(VkPhysicalDevice, DispatchableHandle);
handle!(VkDevice, DispatchableHandle);
handle!(VkQueue, DispatchableHandle);
handle!(VkCommandBuffer, DispatchableHandle);

handle!(VkSemaphore, NonDispatchableHandle);
handle!(VkFence, NonDispatchableHandle);
handle!(VkDeviceMemory, NonDispatchableHandle);
handle!(VkBuffer, NonDispatchableHandle);
handle!(VkImage, NonDispatchableHandle);
handle!(VkEvent, NonDispatchableHandle);
handle!(VkQueryPool, NonDispatchableHandle);
handle!(VkBufferView, NonDispatchableHandle);
handle!(VkImageView, NonDispatchableHandle);
handle!(VkShaderModule, NonDispatchableHandle);
handle!(VkPipelineCache, NonDispatchableHandle);
handle!(VkPipelineLayout, NonDispatchableHandle);
handle!(VkRenderPass, NonDispatchableHandle);
handle!(VkPipeline, NonDispatchableHandle);
handle!(VkDescriptorSetLayout, NonDispatchableHandle);
handle!(VkSampler, NonDispatchableHandle);
handle!(VkDescriptorPool, NonDispatchableHandle);
handle!(VkDescriptorSet, NonDispatchableHandle);
handle!(VkFramebuffer, NonDispatchableHandle);
handle!(VkCommandPool, NonDispatchableHandle);

handle!(VkSamplerYcbcrConversion, NonDispatchableHandle);
handle!(VkDescriptorUpdateTemplate, NonDispatchableHandle);

pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

pub type HINSTANCE = usize;
pub type HWND = usize;
pub type PFN_vkVoidFunction = extern "C" fn();
pub type PFN_vkAllocationFunction = extern "C" fn(pUserData: *mut c_void, size: isize, alignment: isize, allocationScope: VkSystemAllocationScope)->*mut c_void;
pub type PFN_vkReallocationFunction = extern "C" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: isize, alignment: isize, allocationScope: VkSystemAllocationScope)->*mut c_void;
pub type PFN_vkFreeFunction = extern "C" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type PFN_vkInternalAllocationNotification = extern "C" fn(pUserData: *mut c_void, size: isize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type PFN_vkInternalFreeNotification = extern "C" fn(pUserData: *mut c_void, size: isize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

#[repr(C)]
pub struct VkApplicationInfo{
    pub sType :VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
pub struct VkInstanceCreateInfo{
    pub sType :VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[repr(C)]
pub struct VkDeviceCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkAllocationCallbacks{
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct VkExtensionProperties{
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32
}

#[repr(C)]
pub struct VkLayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct VkPhysicalDeviceLimits{
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: isize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct VkPhysicalDeviceSparseProperties{
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct VkPhysicalDeviceFeatures{
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CCharBuffer([i8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]); // TODO 使用泛型常量，更换类型

impl CCharBuffer {
    pub fn to_str(&self)->&str{
        let text = unsafe{ CStr::from_ptr(self.0.as_ptr()) };
        text.to_str().unwrap() // TODO 错误传递
    }
}

impl Default for CCharBuffer {
    fn default() -> Self {
        CCharBuffer([0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]) // TODO 使用泛型常量
    }
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties{
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: CCharBuffer,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct VkExtent2D{
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct VkExtent3D{
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct VkQueueFamilyProperties{
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

#[repr(C)]
pub struct VkImageViewCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkComponentMapping{
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange{
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

#[repr(C)]
pub struct VkSpecializationMapEntry{
    constantID: u32,
    offset: u32,
    size: usize,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VkSpecializationInfo{
    mapEntryCount: u32,
    pMapEntries: *const VkSpecializationMapEntry,
    dataSize: usize,
    pData: *const c_void,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[repr(C)]
pub struct VkVertexInputBindingDescription{
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[repr(C)]
pub struct VkVertexInputAttributeDescription{
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkViewport{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkOffset2D{
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkRect2D{
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState{
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkPushConstantRange{
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

#[derive(Eq, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub domainOrigin: VkTessellationDomainOrigin,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct VkStencilOpState{
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
    compareMask: u32,
    writeMask: u32,
    reference: u32,
}

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags, // flags is reserved for future use.
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

#[repr(C)]
pub struct VkPipelineCacheCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: isize,
    pub pInitialData: *const c_void,
}

#[repr(C)]
pub struct VkAttachmentDescription{
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentReference{
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[repr(C)]
pub struct VkSubpassDescription{
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[repr(C)]
pub struct VkSubpassDependency{
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

#[repr(C)]
pub struct VkFramebufferCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkRenderPassBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue{
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue{
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
pub union VkClearValue{
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
pub struct VkSubmitInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

impl Default for VkSubmitInfo{
    fn default() -> Self {
        VkSubmitInfo{
            sType: VkStructureType::SUBMIT_INFO,
            pNext: 0 as *const c_void,
            waitSemaphoreCount: 0,
            pWaitSemaphores: 0 as *const VkSemaphore,
            pWaitDstStageMask: 0 as *const VkPipelineStageFlags,
            commandBufferCount: 0,
            pCommandBuffers: 0 as *const VkCommandBuffer,
            signalSemaphoreCount: 0,
            pSignalSemaphores: 0 as *const VkSemaphore,
        }
    }
}

#[repr(C)]
pub struct VkFenceCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
pub struct VkBufferCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
pub struct VkBufferViewCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Default)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct VkMemoryType{
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct VkMemoryHeap{
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Default)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

#[repr(C)]
pub struct VkMemoryAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkBufferCopy{
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBinding{
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct VkDescriptorPoolSize{
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
}

#[repr(C)]
pub struct VkDescriptorPoolCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
pub struct VkDescriptorSetAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
pub struct VkDescriptorBufferInfo{
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorImageInfo{
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkWriteDescriptorSet{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

#[repr(C)]
pub struct VkCopyDescriptorSet{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
pub struct VkImageCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkBindImageMemoryInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkImageMemoryBarrier{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkMemoryBarrier{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkBufferImageCopy{
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkImageSubresourceLayers{
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
pub struct VkSamplerCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: f32,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: f32,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

#[repr(C)]
#[derive(Default)]
pub struct VkFormatProperties{
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkImageBlit{
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

#[repr(C)]
pub struct VkMappedMemoryRange{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[cfg_attr(windows, link(name = "vulkan-1"))]
#[cfg_attr(linux, link(name = "vulkan"))]
extern "C" {
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char)->PFN_vkVoidFunction;
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char)->PFN_vkVoidFunction;

    pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32);
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance)->VkResult;
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties)->VkResult;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties)->VkResult;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties)->VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice)->VkResult;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice)->VkResult;
    pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    pub fn vkDeviceWaitIdle(device: VkDevice)->VkResult;

    pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView)->VkResult;
    pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule)->VkResult;
    pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout)->VkResult;
    pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass)->VkResult;
    pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline)->VkResult;
    pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache)->VkResult;
    pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache)->VkResult;
    pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut isize, pData: *mut c_void)->VkResult;
    pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer)->VkResult;
    pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool)->VkResult;
    pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer)->VkResult;
    pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore)->VkResult;
    pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence)->VkResult;
    pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64)->VkResult;
    pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence)->VkResult;
    pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView)->VkResult;
    pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer)->VkResult;
    pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *const VkMemoryRequirements);
    pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory)->VkResult;
    pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize)->VkResult;
    pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void)->VkResult;
    pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange)->VkResult;
    pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange)->VkResult;
    pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout)->VkResult;
    pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool)->VkResult;
    pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet)->VkResult;
    pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage)->VkResult;
    pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize)->VkResult;
    pub fn vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo)->VkResult;
    pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler)->VkResult;
    pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);

    pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo)->VkResult;
    pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer)->VkResult;

    pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence)->VkResult;
    pub fn vkQueueWaitIdle(queue: VkQueue)->VkResult;
}