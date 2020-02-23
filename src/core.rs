#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_char, c_void};
use std::fmt::{Display, Formatter, Error};
use std::mem::transmute;

use crate::*;

bitmasks!{
    {
        VkCullModeFlags,
        enum VkCullModeFlagBits{
            NONE = 0,
            FRONT_BIT = 0x00000001,
            BACK_BIT = 0x00000002,
            FRONT_AND_BACK = 0x00000003,
        }
    },
    {
        VkQueueFlags,
        enum VkQueueFlagBits{
            GRAPHICS_BIT = 0x00000001,
            COMPUTE_BIT = 0x00000002,
            TRANSFER_BIT = 0x00000004,
            SPARSE_BINDING_BIT = 0x00000008,
            PROTECTED_BIT = 0x00000010,
        }
    },
    {
        VkDeviceQueueCreateFlags,
        enum VkDeviceQueueCreateFlagBits{
            PROTECTED_BIT = 0x00000001,
        }
    },
    {
        VkMemoryPropertyFlags,
        enum VkMemoryPropertyFlagBits{
            DEVICE_LOCAL_BIT = 0x00000001,
            HOST_VISIBLE_BIT = 0x00000002,
            HOST_COHERENT_BIT = 0x00000004,
            HOST_CACHED_BIT = 0x00000008,
            LAZILY_ALLOCATED_BIT = 0x00000010,
            PROTECTED_BIT = 0x00000020,
        }
    },
    {
        VkMemoryHeapFlags,
        enum VkMemoryHeapFlagBits{
            DEVICE_LOCAL_BIT = 0x00000001,
            MULTI_INSTANCE_BIT = 0x00000002,
        }
    },
    {
        VkAccessFlags,
        enum VkAccessFlagBits{
            INDIRECT_COMMAND_READ_BIT = 0x00000001,
            INDEX_READ_BIT = 0x00000002,
            VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
            UNIFORM_READ_BIT = 0x00000008,
            INPUT_ATTACHMENT_READ_BIT = 0x00000010,
            SHADER_READ_BIT = 0x00000020,
            SHADER_WRITE_BIT = 0x00000040,
            COLOR_ATTACHMENT_READ_BIT = 0x00000080,
            COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
            DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
            DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
            TRANSFER_READ_BIT = 0x00000800,
            TRANSFER_WRITE_BIT = 0x00001000,
            HOST_READ_BIT = 0x00002000,
            HOST_WRITE_BIT = 0x00004000,
            MEMORY_READ_BIT = 0x00008000,
            MEMORY_WRITE_BIT = 0x00010000,
        }
    },
    {
        VkBufferUsageFlags,
        enum VkBufferUsageFlagBits{
            TRANSFER_SRC_BIT = 0x00000001,
            TRANSFER_DST_BIT = 0x00000002,
            UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
            STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
            UNIFORM_BUFFER_BIT = 0x00000010,
            STORAGE_BUFFER_BIT = 0x00000020,
            INDEX_BUFFER_BIT = 0x00000040,
            VERTEX_BUFFER_BIT = 0x00000080,
            INDIRECT_BUFFER_BIT = 0x00000100,
            SHADER_DEVICE_ADDRESS_BIT = 0x00020000,
        }
    },
    {
        VkBufferCreateFlags,
        enum VkBufferCreateFlagBits{
            SPARSE_BINDING_BIT = 0x00000001,
            SPARSE_RESIDENCY_BIT = 0x00000002,
            SPARSE_ALIASED_BIT = 0x00000004,
            PROTECTED_BIT = 0x00000008,
            DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000010,
        }
    },
    {
        VkShaderStageFlags,
        enum VkShaderStageFlagBits{
            VERTEX_BIT = 0x00000001,
            TESSELLATION_CONTROL_BIT = 0x00000002,
            TESSELLATION_EVALUATION_BIT = 0x00000004,
            GEOMETRY_BIT = 0x00000008,
            FRAGMENT_BIT = 0x00000010,
            COMPUTE_BIT = 0x00000020,
            ALL_GRAPHICS = 0x0000001F,
            ALL = 0x7FFFFFFF,
        }
    },
    {
        VkImageUsageFlags,
        enum VkImageUsageFlagBits{
            TRANSFER_SRC_BIT = 0x00000001,
            TRANSFER_DST_BIT = 0x00000002,
            SAMPLED_BIT = 0x00000004,
            STORAGE_BIT = 0x00000008,
            COLOR_ATTACHMENT_BIT = 0x00000010,
            DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
            TRANSIENT_ATTACHMENT_BIT = 0x00000040,
            INPUT_ATTACHMENT_BIT = 0x00000080,
        }
    },
    {
        VkImageCreateFlags,
        enum VkImageCreateFlagBits{
            SPARSE_BINDING_BIT = 0x00000001,
            SPARSE_RESIDENCY_BIT = 0x00000002,
            SPARSE_ALIASED_BIT = 0x00000004,
            MUTABLE_FORMAT_BIT = 0x00000008,
            CUBE_COMPATIBLE_BIT = 0x00000010,
            ALIAS_BIT = 0x00000400,
            SPLIT_INSTANCE_BIND_REGIONS_BIT = 0x00000040,
            IC_2D_ARRAY_COMPATIBLE_BIT = 0x00000020, // VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT
            BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 0x00000080,
            EXTENDED_USAGE_BIT = 0x00000100,
            PROTECTED_BIT = 0x00000800,
            DISJOINT_BIT = 0x00000200,
        }
    },
    {
        VkPipelineCreateFlags,
        enum VkPipelineCreateFlagBits{
            DISABLE_OPTIMIZATION_BIT = 0x00000001,
            ALLOW_DERIVATIVES_BIT = 0x00000002,
            DERIVATIVE_BIT = 0x00000004,
            VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 0x00000008,
            DISPATCH_BASE_BIT = 0x00000010,
        }
    },
    {
        VkColorComponentFlags,
        enum VkColorComponentFlagBits{
            R_BIT = 0x00000001,
            G_BIT = 0x00000002,
            B_BIT = 0x00000004,
            A_BIT = 0x00000008,
        }
    },
    {
        VkFenceCreateFlags,
        enum VkFenceCreateFlagBits{
            SIGNALED_BIT = 0x00000001,
        }
    },
    {
        VkFormatFeatureFlags,
        enum VkFormatFeatureFlagBits{
            SAMPLED_IMAGE_BIT = 0x00000001,
            STORAGE_IMAGE_BIT = 0x00000002,
            STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,
            UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,
            STORAGE_TEXEL_BUFFER_BIT = 0x00000010,
            STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,
            VERTEX_BUFFER_BIT = 0x00000040,
            COLOR_ATTACHMENT_BIT = 0x00000080,
            COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,
            DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,
            BLIT_SRC_BIT = 0x00000400,
            BLIT_DST_BIT = 0x00000800,
            SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,
            TRANSFER_SRC_BIT = 0x00004000,
            TRANSFER_DST_BIT = 0x00008000,
            MIDPOINT_CHROMA_SAMPLES_BIT = 0x00020000,
            SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 0x00040000,
            SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 0x00080000,
            SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 0x00100000,
            SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 0x00200000,
            DISJOINT_BIT = 0x00400000,
            COSITED_CHROMA_SAMPLES_BIT = 0x00800000,
            SAMPLED_IMAGE_FILTER_MINMAX_BIT = 0x00010000,
        }
    },
    {
        VkQueryControlFlags,
        enum VkQueryControlFlagBits{
            PRECISE_BIT = 0x00000001,
        }
    },
    {
        VkQueryResultFlags,
        enum VkQueryResultFlagBits{
            U64_BIT = 0x00000001, // VK_QUERY_RESULT_64_BIT
            WAIT_BIT = 0x00000002,
            WITH_AVAILABILITY_BIT = 0x00000004,
            PARTIAL_BIT = 0x00000008,
        }
    },
    {
        VkCommandBufferUsageFlags,
        enum VkCommandBufferUsageFlagBits{
            ONE_TIME_SUBMIT_BIT = 0x00000001,
            RENDER_PASS_CONTINUE_BIT = 0x00000002,
            SIMULTANEOUS_USE_BIT = 0x00000004,
        }
    },
    {
        VkQueryPipelineStatisticFlags,
        enum VkQueryPipelineStatisticFlagBits{
            INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
            INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
            VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
            GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
            GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
            CLIPPING_INVOCATIONS_BIT = 0x00000020,
            CLIPPING_PRIMITIVES_BIT = 0x00000040,
            FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
            TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
            TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
            COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
        }
    },
    {
        VkImageAspectFlags,
        enum VkImageAspectFlagBits{
            COLOR_BIT = 0x00000001,
            DEPTH_BIT = 0x00000002,
            STENCIL_BIT = 0x00000004,
            METADATA_BIT = 0x00000008,
            PLANE_0_BIT = 0x00000010,
            PLANE_1_BIT = 0x00000020,
            PLANE_2_BIT = 0x00000040,
        }
    },
    {
        VkSparseImageFormatFlags,
        enum VkSparseImageFormatFlagBits{
            SINGLE_MIPTAIL_BIT = 0x00000001,
            ALIGNED_MIP_SIZE_BIT = 0x00000002,
            NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004,
        }
    },
    {
        VkSparseMemoryBindFlags,
        enum VkSparseMemoryBindFlagBits{
            METADATA_BIT = 0x00000001,
        }
    },
    {
        VkPipelineStageFlags,
        enum VkPipelineStageFlagBits{
            TOP_OF_PIPE_BIT = 0x00000001,
            DRAW_INDIRECT_BIT = 0x00000002,
            VERTEX_INPUT_BIT = 0x00000004,
            VERTEX_SHADER_BIT = 0x00000008,
            TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
            TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
            GEOMETRY_SHADER_BIT = 0x00000040,
            FRAGMENT_SHADER_BIT = 0x00000080,
            EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
            LATE_FRAGMENT_TESTS_BIT = 0x00000200,
            COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
            COMPUTE_SHADER_BIT = 0x00000800,
            TRANSFER_BIT = 0x00001000,
            BOTTOM_OF_PIPE_BIT = 0x00002000,
            HOST_BIT = 0x00004000,
            ALL_GRAPHICS_BIT = 0x00008000,
            ALL_COMMANDS_BIT = 0x00010000,
        }
    },
    {
        VkCommandPoolCreateFlags,
        enum VkCommandPoolCreateFlagBits{
            TRANSIENT_BIT = 0x00000001,
            RESET_COMMAND_BUFFER_BIT = 0x00000002,
            PROTECTED_BIT = 0x00000004,
        }
    },
    {
        VkCommandPoolResetFlags,
        enum VkCommandPoolResetFlagBits{
            RELEASE_RESOURCES_BIT = 0x00000001,
        }
    },
    {
        VkCommandBufferResetFlags,
        enum VkCommandBufferResetFlagBits{
            RELEASE_RESOURCES_BIT = 0x00000001,
        }
    },
    {
        VkSampleCountFlags,
        enum VkSampleCountFlagBits{
            SC_1_BIT = 0x00000001, // VK_SAMPLE_COUNT_1_BIT
            SC_2_BIT = 0x00000002, // VK_SAMPLE_COUNT_2_BIT
            SC_4_BIT = 0x00000004, // VK_SAMPLE_COUNT_4_BIT
            SC_8_BIT = 0x00000008, // VK_SAMPLE_COUNT_8_BIT
            SC_16_BIT = 0x00000010, // VK_SAMPLE_COUNT_16_BIT
            SC_32_BIT = 0x00000020, // VK_SAMPLE_COUNT_32_BIT
            SC_64_BIT = 0x00000040, // VK_SAMPLE_COUNT_64_BIT
        }
    },
    {
        VkAttachmentDescriptionFlags,
        enum VkAttachmentDescriptionFlagBits{
            MAY_ALIAS_BIT = 0x00000001,
        }
    },
    {
        VkStencilFaceFlags,
        enum VkStencilFaceFlagBits{
            FRONT_BIT = 0x00000001,
            BACK_BIT = 0x00000002,
            FRONT_AND_BACK = 0x00000003,
        }
    },
    {
        VkDescriptorPoolCreateFlags,
        enum VkDescriptorPoolCreateFlagBits{
            FREE_DESCRIPTOR_SET_BIT = 0x00000001,
            UPDATE_AFTER_BIND_BIT = 0x00000002,
        }
    },
    {
        VkDependencyFlags,
        enum VkDependencyFlagBits{
            BY_REGION_BIT = 0x00000001,
            VIEW_LOCAL_BIT = 0x00000002,
            DEVICE_GROUP_BIT = 0x00000004,
        }
    },
    {
        VkSemaphoreWaitFlags,
        enum VkSemaphoreWaitFlagBits{
            ANY_BIT = 0x00000001,
        }
    },
    {
        VkSubgroupFeatureFlags,
        enum VkSubgroupFeatureFlagBits{
            BASIC_BIT = 0x00000001,
            VOTE_BIT = 0x00000002,
            ARITHMETIC_BIT = 0x00000004,
            BALLOT_BIT = 0x00000008,
            SHUFFLE_BIT = 0x00000010,
            SHUFFLE_RELATIVE_BIT = 0x00000020,
            CLUSTERED_BIT = 0x00000040,
            QUAD_BIT = 0x00000080,
        }
    },
    {
        VkDescriptorSetLayoutCreateFlags,
        enum VkDescriptorSetLayoutCreateFlagBits{
            UPDATE_AFTER_BIND_POOL_BIT = 0x00000002,
        }
    },
    {
        VkExternalMemoryHandleTypeFlags,
        enum VkExternalMemoryHandleTypeFlagBits{
            OPAQUE_FD_BIT = 0x00000001,
            OPAQUE_WIN32_BIT = 0x00000002,
            OPAQUE_WIN32_KMT_BIT = 0x00000004,
            D3D11_TEXTURE_BIT = 0x00000008,
            D3D11_TEXTURE_KMT_BIT = 0x00000010,
            D3D12_HEAP_BIT = 0x00000020,
            D3D12_RESOURCE_BIT = 0x00000040,
        }
    },
    {
        VkExternalMemoryFeatureFlags,
        enum VkExternalMemoryFeatureFlagBits{
            DEDICATED_ONLY_BIT = 0x00000001,
            EXPORTABLE_BIT = 0x00000002,
            IMPORTABLE_BIT = 0x00000004,
        }
    },
    {
        VkExternalSemaphoreHandleTypeFlags,
        enum VkExternalSemaphoreHandleTypeFlagBits{
            OPAQUE_FD_BIT = 0x00000001,
            OPAQUE_WIN32_BIT = 0x00000002,
            OPAQUE_WIN32_KMT_BIT = 0x00000004,
            D3D12_FENCE_BIT = 0x00000008,
            SYNC_FD_BIT = 0x00000010,
        }
    },
    {
        VkExternalSemaphoreFeatureFlags,
        enum VkExternalSemaphoreFeatureFlagBits{
            EXPORTABLE_BIT = 0x00000001,
            IMPORTABLE_BIT = 0x00000002,
        }
    },
    {
        VkExternalFenceHandleTypeFlags,
        enum VkExternalFenceHandleTypeFlagBits{
            OPAQUE_FD_BIT = 0x00000001,
            OPAQUE_WIN32_BIT = 0x00000002,
            OPAQUE_WIN32_KMT_BIT = 0x00000004,
            SYNC_FD_BIT = 0x00000008,
        }
    },
    {
        VkExternalFenceFeatureFlags,
        enum VkExternalFenceFeatureFlagBits{
            EXPORTABLE_BIT = 0x00000001,
            IMPORTABLE_BIT = 0x00000002,
        }
    },
    {
        VkPeerMemoryFeatureFlags,
        enum VkPeerMemoryFeatureFlagBits{
            COPY_SRC_BIT = 0x00000001,
            COPY_DST_BIT = 0x00000002,
            GENERIC_SRC_BIT = 0x00000004,
            GENERIC_DST_BIT = 0x00000008,
        }
    },
    {
        VkMemoryAllocateFlags,
        enum VkMemoryAllocateFlagBits{
            DEVICE_MASK_BIT = 0x00000001,
            DEVICE_ADDRESS_BIT = 0x00000002,
            DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000004,
        }
    },
    {
        VkDescriptorBindingFlags,
        enum VkDescriptorBindingFlagBits{
            UPDATE_AFTER_BIND_BIT = 0x00000001,
            UPDATE_UNUSED_WHILE_PENDING_BIT = 0x00000002,
            PARTIALLY_BOUND_BIT = 0x00000004,
            VARIABLE_DESCRIPTOR_COUNT_BIT = 0x00000008,
        }
    },
    {
        VkResolveModeFlags,
        enum VkResolveModeFlagBits{
            NONE = 0,
            SAMPLE_ZERO_BIT = 0x00000001,
            AVERAGE_BIT = 0x00000002,
            MIN_BIT = 0x00000004,
            MAX_BIT = 0x00000008,
        }
    },
    {
        VkFramebufferCreateFlags,
        enum VkFramebufferCreateFlagBits{
            IMAGELESS_BIT = 0x00000001,
        }
    },
    {
        VkDeviceCreateFlags,
        enum VkDeviceCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkInstanceCreateFlags,
        enum VkInstanceCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkBufferViewCreateFlags,
        enum VkBufferViewCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkImageViewCreateFlags,
        enum VkImageViewCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkShaderModuleCreateFlags,
        enum VkShaderModuleCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineShaderStageCreateFlags,
        enum VkPipelineShaderStageCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineVertexInputStateCreateFlags,
        enum VkPipelineVertexInputStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineInputAssemblyStateCreateFlags,
        enum VkPipelineInputAssemblyStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineTessellationStateCreateFlags,
        enum VkPipelineTessellationStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineViewportStateCreateFlags,
        enum VkPipelineViewportStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineRasterizationStateCreateFlags,
        enum VkPipelineRasterizationStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineMultisampleStateCreateFlags,
        enum VkPipelineMultisampleStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineColorBlendStateCreateFlags,
        enum VkPipelineColorBlendStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineDynamicStateCreateFlags,
        enum VkPipelineDynamicStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineDepthStencilStateCreateFlags,
        enum VkPipelineDepthStencilStateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineCacheCreateFlags,
        enum VkPipelineCacheCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkPipelineLayoutCreateFlags,
        enum VkPipelineLayoutCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkSamplerCreateFlags,
        enum VkSamplerCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkRenderPassCreateFlags,
        enum VkRenderPassCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkEventCreateFlags,
        enum VkEventCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkSemaphoreCreateFlags,
        enum VkSemaphoreCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkQueryPoolCreateFlags,
        enum VkQueryPoolCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkDescriptorUpdateTemplateCreateFlags,
        enum VkDescriptorUpdateTemplateCreateFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkSubpassDescriptionFlags,
        enum VkSubpassDescriptionFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkMemoryMapFlags,
        enum VkMemoryMapFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkDescriptorPoolResetFlags,
        enum VkDescriptorPoolResetFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkCommandPoolTrimFlags,
        enum VkCommandPoolTrimFlagBits{
            _RESERVED = 0,
        }
    },
    {
        VkFenceImportFlags,
        enum VkFenceImportFlagBits{
            TEMPORARY_BIT = 0x00000001,
        }
    },
}

core_enums!{
    enum VkImageLayout{
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
        DEPTH_ATTACHMENT_OPTIMAL = 1000241000,
        DEPTH_READ_ONLY_OPTIMAL = 1000241001,
        STENCIL_ATTACHMENT_OPTIMAL = 1000241002,
        STENCIL_READ_ONLY_OPTIMAL = 1000241003,
    },
    enum VkAttachmentLoadOp{
        LOAD = 0,
        CLEAR = 1,
        DONT_CARE = 2,
    },
    enum VkAttachmentStoreOp{
        STORE = 0,
        DONT_CARE = 1,
    },
    enum VkImageType{
        IT_1D = 0, // VK_IMAGE_TYPE_1D
        IT_2D = 1, // VK_IMAGE_TYPE_2D
        IT_3D = 2, // VK_IMAGE_TYPE_3D
    },
    enum VkImageTiling{
        OPTIMAL = 0,
        LINEAR = 1,
    },
    enum VkImageViewType{
        IVT_1D = 0, // VK_IMAGE_VIEW_TYPE_1D
        IVT_2D = 1, // VK_IMAGE_VIEW_TYPE_2D
        IVT_3D = 2, // VK_IMAGE_VIEW_TYPE_3D
        CUBE = 3,
        IVT_1D_ARRAY = 4, // VK_IMAGE_VIEW_TYPE_1D_ARRAY
        IVT_2D_ARRAY = 5, // VK_IMAGE_VIEW_TYPE_2D_ARRAY
        CUBE_ARRAY = 6,
    },
    enum VkCommandBufferLevel{
        PRIMARY = 0,
        SECONDARY = 1,
    },
    enum VkComponentSwizzle{
        IDENTITY = 0,
        ZERO = 1,
        ONE = 2,
        R = 3,
        G = 4,
        B = 5,
        A = 6,
    },
    enum VkDescriptorType{
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
    },
    enum VkQueryType{
        OCCLUSION = 0,
        PIPELINE_STATISTICS = 1,
        TIMESTAMP = 2,
    },
    enum VkBorderColor{
        FLOAT_TRANSPARENT_BLACK = 0,
        INT_TRANSPARENT_BLACK = 1,
        FLOAT_OPAQUE_BLACK = 2,
        INT_OPAQUE_BLACK = 3,
        FLOAT_OPAQUE_WHITE = 4,
        INT_OPAQUE_WHITE = 5,
    },
    enum VkPipelineBindPoint{
        GRAPHICS = 0,
        COMPUTE = 1,
    },
    enum VkPipelineCacheHeaderVersion{
        ONE = 1,
    },
    enum VkPrimitiveTopology{
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
    },
    enum VkSharingMode{
        EXCLUSIVE = 0,
        CONCURRENT = 1,
    },
    enum VkIndexType{
        UINT16 = 0,
        UINT32 = 1,
    },
    enum VkFilter{
        NEAREST = 0,
        LINEAR = 1,
    },
    enum VkSamplerMipmapMode{
        NEAREST = 0,
        LINEAR = 1,
    },
    enum VkSamplerAddressMode{
        REPEAT = 0,
        MIRRORED_REPEAT = 1,
        CLAMP_TO_EDGE = 2,
        CLAMP_TO_BORDER = 3,
        MIRROR_CLAMP_TO_EDGE = 4,
    },
    enum VkCompareOp{
        NEVER = 0,
        LESS = 1,
        EQUAL = 2,
        LESS_OR_EQUAL = 3,
        GREATER = 4,
        NOT_EQUAL = 5,
        GREATER_OR_EQUAL = 6,
        ALWAYS = 7,
    },
    enum VkPolygonMode{
        FILL = 0,
        LINE = 1,
        POINT = 2,
    },
    enum VkFrontFace{
        COUNTER_CLOCKWISE = 0,
        CLOCKWISE = 1,
    },
    enum VkBlendFactor{
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
    },
    enum VkBlendOp{
        ADD = 0,
        SUBTRACT = 1,
        REVERSE_SUBTRACT = 2,
        MIN = 3,
        MAX = 4,
    },
    enum VkStencilOp{
        KEEP = 0,
        ZERO = 1,
        REPLACE = 2,
        INCREMENT_AND_CLAMP = 3,
        DECREMENT_AND_CLAMP = 4,
        INVERT = 5,
        INCREMENT_AND_WRAP = 6,
        DECREMENT_AND_WRAP = 7,
    },
    enum VkLogicOp{
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
    },
    enum VkInternalAllocationType{
        EXECUTABLE = 0,
    },
    enum VkSystemAllocationScope{
        COMMAND = 0,
        OBJECT = 1,
        CACHE = 2,
        DEVICE = 3,
        INSTANCE = 4,
    },
    enum VkPhysicalDeviceType{
        OTHER = 0,
        INTEGRATED_GPU = 1,
        DISCRETE_GPU = 2,
        VIRTUAL_GPU = 3,
        CPU = 4,
    },
    enum VkVertexInputRate{
        VERTEX = 0,
        INSTANCE = 1,
    },
    enum VkFormat{
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
    },
    enum VkStructureType{
        APPLICATION_INFO = 0,
        INSTANCE_CREATE_INFO = 1,
        DEVICE_QUEUE_CREATE_INFO = 2,
        DEVICE_CREATE_INFO = 3,
        SUBMIT_INFO = 4,
        MEMORY_ALLOCATE_INFO = 5,
        MAPPED_MEMORY_RANGE = 6,
        BIND_SPARSE_INFO = 7,
        FENCE_CREATE_INFO = 8,
        SEMAPHORE_CREATE_INFO = 9,
        EVENT_CREATE_INFO = 10,
        QUERY_POOL_CREATE_INFO = 11,
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
        COMPUTE_PIPELINE_CREATE_INFO = 29,
        PIPELINE_LAYOUT_CREATE_INFO = 30,
        SAMPLER_CREATE_INFO = 31,
        DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
        DESCRIPTOR_POOL_CREATE_INFO = 33,
        DESCRIPTOR_SET_ALLOCATE_INFO = 34,
        WRITE_DESCRIPTOR_SET = 35,
        COPY_DESCRIPTOR_SET = 36,
        FRAMEBUFFER_CREATE_INFO = 37,
        RENDER_PASS_CREATE_INFO = 38,
        COMMAND_POOL_CREATE_INFO = 39,
        COMMAND_BUFFER_ALLOCATE_INFO = 40,
        COMMAND_BUFFER_INHERITANCE_INFO = 41,
        COMMAND_BUFFER_BEGIN_INFO = 42,
        RENDER_PASS_BEGIN_INFO = 43,
        BUFFER_MEMORY_BARRIER = 44,
        IMAGE_MEMORY_BARRIER = 45,
        MEMORY_BARRIER = 46,
        LOADER_INSTANCE_CREATE_INFO = 47, // reserved for internal use by the loader
        LOADER_DEVICE_CREATE_INFO = 48, // // reserved for internal use by the loader
        PHYSICAL_DEVICE_SUBGROUP_PROPERTIES = 1000094000,
        BIND_BUFFER_MEMORY_INFO = 1000157000,
        BIND_IMAGE_MEMORY_INFO = 1000157001,
        PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES = 1000083000,
        MEMORY_DEDICATED_REQUIREMENTS = 1000127000,
        MEMORY_DEDICATED_ALLOCATE_INFO = 1000127001,
        MEMORY_ALLOCATE_FLAGS_INFO = 1000060000,
        DEVICE_GROUP_RENDER_PASS_BEGIN_INFO = 1000060003,
        DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO = 1000060004,
        DEVICE_GROUP_SUBMIT_INFO = 1000060005,
        DEVICE_GROUP_BIND_SPARSE_INFO = 1000060006,
        BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO = 1000060013,
        BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO = 1000060014,
        PHYSICAL_DEVICE_GROUP_PROPERTIES = 1000070000,
        DEVICE_GROUP_DEVICE_CREATE_INFO = 1000070001,
        BUFFER_MEMORY_REQUIREMENTS_INFO_2 = 1000146000,
        IMAGE_MEMORY_REQUIREMENTS_INFO_2 = 1000146001,
        IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 = 1000146002,
        MEMORY_REQUIREMENTS_2 = 1000146003,
        SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 = 1000146004,
        PHYSICAL_DEVICE_FEATURES_2 = 1000059000,
        PHYSICAL_DEVICE_PROPERTIES_2 = 1000059001,
        FORMAT_PROPERTIES_2 = 1000059002,
        IMAGE_FORMAT_PROPERTIES_2 = 1000059003,
        PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 = 1000059004,
        QUEUE_FAMILY_PROPERTIES_2 = 1000059005,
        PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 = 1000059006,
        SPARSE_IMAGE_FORMAT_PROPERTIES_2 = 1000059007,
        PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 = 1000059008,
        PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES = 1000117000,
        RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO = 1000117001,
        IMAGE_VIEW_USAGE_CREATE_INFO = 1000117002,
        PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO = 1000117003,
        RENDER_PASS_MULTIVIEW_CREATE_INFO = 1000053000,
        PHYSICAL_DEVICE_MULTIVIEW_FEATURES = 1000053001,
        PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES = 1000053002,
        PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES = 1000120000,
        PROTECTED_SUBMIT_INFO = 1000145000,
        PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES = 1000145001,
        PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES = 1000145002,
        DEVICE_QUEUE_INFO_2 = 1000145003,
        SAMPLER_YCBCR_CONVERSION_CREATE_INFO = 1000156000,
        SAMPLER_YCBCR_CONVERSION_INFO = 1000156001,
        BIND_IMAGE_PLANE_MEMORY_INFO = 1000156002,
        IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO = 1000156003,
        PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES = 1000156004,
        SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES = 1000156005,
        DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO = 1000085000,
        PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO = 1000071000,
        EXTERNAL_IMAGE_FORMAT_PROPERTIES = 1000071001,
        PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO = 1000071002,
        EXTERNAL_BUFFER_PROPERTIES = 1000071003,
        PHYSICAL_DEVICE_ID_PROPERTIES = 1000071004,
        EXTERNAL_MEMORY_BUFFER_CREATE_INFO = 1000072000,
        EXTERNAL_MEMORY_IMAGE_CREATE_INFO = 1000072001,
        EXPORT_MEMORY_ALLOCATE_INFO = 1000072002,
        PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO = 1000112000,
        EXTERNAL_FENCE_PROPERTIES = 1000112001,
        EXPORT_FENCE_CREATE_INFO = 1000113000,
        EXPORT_SEMAPHORE_CREATE_INFO = 1000077000,
        PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO = 1000076000,
        EXTERNAL_SEMAPHORE_PROPERTIES = 1000076001,
        PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES = 1000168000,
        DESCRIPTOR_SET_LAYOUT_SUPPORT = 1000168001,
        PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES = 1000063000,
        PHYSICAL_DEVICE_VULKAN_1_1_FEATURES = 49,
        PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES = 50,
        PHYSICAL_DEVICE_VULKAN_1_2_FEATURES = 51,
        PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES = 52,
        IMAGE_FORMAT_LIST_CREATE_INFO = 1000147000,
        ATTACHMENT_DESCRIPTION_2 = 1000109000,
        ATTACHMENT_REFERENCE_2 = 1000109001,
        SUBPASS_DESCRIPTION_2 = 1000109002,
        SUBPASS_DEPENDENCY_2 = 1000109003,
        RENDER_PASS_CREATE_INFO_2 = 1000109004,
        SUBPASS_BEGIN_INFO = 1000109005,
        SUBPASS_END_INFO = 1000109006,
        PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES = 1000177000,
        PHYSICAL_DEVICE_DRIVER_PROPERTIES = 1000196000,
        PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES = 1000180000,
        PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES = 1000082000,
        PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES = 1000197000,
        DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO = 1000161000,
        PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES = 1000161001,
        PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES = 1000161002,
        DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO = 1000161003,
        DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT = 1000161004,
        PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES = 1000199000,
        SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE = 1000199001,
        PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES = 1000221000,
        IMAGE_STENCIL_USAGE_CREATE_INFO = 1000246000,
        PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES = 1000130000,
        SAMPLER_REDUCTION_MODE_CREATE_INFO = 1000130001,
        PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES = 1000211000,
        PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES = 1000108000,
        FRAMEBUFFER_ATTACHMENTS_CREATE_INFO = 1000108001,
        FRAMEBUFFER_ATTACHMENT_IMAGE_INFO = 1000108002,
        RENDER_PASS_ATTACHMENT_BEGIN_INFO = 1000108003,
        PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES = 1000253000,
        PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES = 1000175000,
        PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES = 1000241000,
        ATTACHMENT_REFERENCE_STENCIL_LAYOUT = 1000241001,
        ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT = 1000241002,
        PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES = 1000261000,
        PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES = 1000207000,
        PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES = 1000207001,
        SEMAPHORE_TYPE_CREATE_INFO = 1000207002,
        TIMELINE_SEMAPHORE_SUBMIT_INFO = 1000207003,
        SEMAPHORE_WAIT_INFO = 1000207004,
        SEMAPHORE_SIGNAL_INFO = 1000207005,
        PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES = 1000257000,
        BUFFER_DEVICE_ADDRESS_INFO = 1000244001,
        BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO = 1000257002,
        MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO = 1000257003,
        DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO = 1000257004,
    },
    enum VkSubpassContents{
        INLINE = 0,
        SECONDARY_COMMAND_BUFFERS = 1,
    },
    enum VkResult{
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
        ERROR_UNKNOWN = -13,
        ERROR_OUT_OF_POOL_MEMORY = -1000069000,
        ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,
        ERROR_FRAGMENTATION = -1000161000,
        ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS = -1000257000,
    },
    enum VkDynamicState{
        VIEWPORT = 0,
        SCISSOR = 1,
        LINE_WIDTH = 2,
        DEPTH_BIAS = 3,
        BLEND_CONSTANTS = 4,
        DEPTH_BOUNDS = 5,
        STENCIL_COMPARE_MASK = 6,
        STENCIL_WRITE_MASK = 7,
        STENCIL_REFERENCE = 8,
    },
    enum VkDescriptorUpdateTemplateType{
        DESCRIPTOR_SET = 0,
    },
    enum VkObjectType{
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
    },
    enum VkSemaphoreType{
        BINARY = 0,
        TIMELINE = 1,
    },
    enum VkPointClippingBehavior{
        ALL_CLIP_PLANES = 0,
        USER_CLIP_PLANES_ONLY = 1,
    },
    enum VkSamplerReductionMode{
        WEIGHTED_AVERAGE = 0,
        MIN = 1,
        MAX = 2,
    },
    enum VkTessellationDomainOrigin{
        UPPER_LEFT = 0,
        LOWER_LEFT = 1,
    },
    enum VkSamplerYcbcrModelConversion{
        RGB_IDENTITY = 0,
        YCBCR_IDENTITY = 1,
        YCBCR_709 = 2,
        YCBCR_601 = 3,
        YCBCR_2020 = 4,
    },
    enum VkSamplerYcbcrRange{
        ITU_FULL = 0,
        ITU_NARROW = 1,
    },
    enum VkChromaLocation{
        COSITED_EVEN = 0,
        MIDPOINT = 1,
    },
    enum VkVendorId{
        VIV = 0x10001,
        VSI = 0x10002,
        KAZAN = 0x10003,
    },
    enum VkDriverId{
        AMD_PROPRIETARY = 1,
        AMD_OPEN_SOURCE = 2,
        MESA_RADV = 3,
        NVIDIA_PROPRIETARY = 4,
        INTEL_PROPRIETARY_WINDOWS = 5,
        INTEL_OPEN_SOURCE_MESA = 6,
        IMAGINATION_PROPRIETARY = 7,
        QUALCOMM_PROPRIETARY = 8,
        ARM_PROPRIETARY = 9,
        GOOGLE_SWIFTSHADER = 10,
        GGP_PROPRIETARY = 11,
        BROADCOM_PROPRIETARY = 12,
    },
    enum VkShaderFloatControlsIndependence{
        F32_BIT_ONLY = 0, // VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY
        ALL = 1,
        NONE = 2,
    },
}

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

pub type PFN_vkVoidFunction = extern "C" fn();
pub type PFN_vkAllocationFunction = extern "C" fn(pUserData: *mut c_void, size: isize, alignment: isize, allocationScope: VkSystemAllocationScope)->*mut c_void;
pub type PFN_vkReallocationFunction = extern "C" fn(pUserData: *mut c_void, pOriginal: *mut c_void, size: isize, alignment: isize, allocationScope: VkSystemAllocationScope)->*mut c_void;
pub type PFN_vkFreeFunction = extern "C" fn(pUserData: *mut c_void, pMemory: *mut c_void);
pub type PFN_vkInternalAllocationNotification = extern "C" fn(pUserData: *mut c_void, size: isize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);
pub type PFN_vkInternalFreeNotification = extern "C" fn(pUserData: *mut c_void, size: isize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

#[repr(C)]
pub struct VkBaseOutStructure{
    pub sType: VkStructureType,
    pub pNext: *mut VkBaseOutStructure,
}

#[repr(C)]
pub struct VkBaseInStructure{
    pub sType: VkStructureType,
    pub pNext: *const VkBaseInStructure,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkOffset2D{
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
pub struct VkRect2D{
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[repr(C)]
pub struct VkClearRect{
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
pub struct VkComponentMapping{
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties{
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl Default for VkPhysicalDeviceProperties{
    fn default() -> Self {
        VkPhysicalDeviceProperties{
            apiVersion: Default::default(),
            driverVersion: Default::default(),
            vendorID: Default::default(),
            deviceID: Default::default(),
            deviceType: Default::default(),
            deviceName: [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
            pipelineCacheUUID: Default::default(),
            limits: Default::default(),
            sparseProperties: Default::default(),
        }
    }
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
pub struct VkApplicationInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
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

#[derive(Debug)]
#[repr(C)]
pub struct VkQueueFamilyProperties{
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
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

#[repr(C)]
#[derive(Default)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties{
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements{
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
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
pub struct VkMappedMemoryRange{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Default)]
pub struct VkFormatProperties{
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags
}

#[repr(C)]
pub struct VkImageFormatProperties{
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
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
pub struct VkImageSubresource{
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
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
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange{
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
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
pub struct VkSubresourceLayout{
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkBufferCopy{
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
pub struct VkSparseMemoryBind{
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryBind{
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo{
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo{
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageMemoryBindInfo{
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

#[repr(C)]
pub struct VkBindSparseInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
pub struct VkImageCopy{
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkImageBlit{
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
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

#[repr(C)]
pub struct VkImageResolve{
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
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

#[repr(C)]
pub struct VkComputePipelineCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
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

#[derive(Clone)]
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
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
    pub flags: VkPipelineDepthStencilStateCreateFlags,
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

#[repr(C)]
pub struct VkPipelineCacheCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: isize,
    pub pInitialData: *const c_void,
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
pub struct VkCommandBufferBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
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
pub union VkClearColorValue{
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue{
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
pub union VkClearValue{
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

#[repr(C)]
pub struct VkClearAttachment{
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
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
pub struct VkEventCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlags,
}

#[repr(C)]
pub struct VkFenceCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
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
pub struct VkSemaphoreCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
pub struct VkQueryPoolCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
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
pub struct VkDrawIndirectCommand{
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

#[repr(C)]
pub struct VkDrawIndexedIndirectCommand{
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[repr(C)]
pub struct VkDispatchIndirectCommand{
    pub x: u32,
    pub y: u32,
    pub z: u32,
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

#[repr(C)]
pub struct VkPhysicalDeviceFeatures2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
pub struct VkFormatProperties2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub formatProperties: VkFormatProperties,
}

#[repr(C)]
pub struct VkImageFormatProperties2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}

#[repr(C)]
pub struct VkQueueFamilyProperties2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkSparseImageFormatProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

#[repr(C)]
pub struct VkConformanceVersion{
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub driverID: VkDriverId,
    pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE],
    pub driverInfo: [c_char; VK_MAX_DRIVER_NAME_SIZE],
    pub conformanceVersion: VkConformanceVersion,
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
}
pub type VkPhysicalDeviceVariablePointerFeatures = VkPhysicalDeviceVariablePointersFeatures;

#[repr(C)]
pub struct VkExternalMemoryProperties{
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalImageFormatProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalBufferProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceIDProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalSemaphoreProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
}

#[repr(C)]
pub struct VkExportSemaphoreCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalFenceProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
    pub externalFenceFeatures: VkExternalFenceFeatureFlags,
}

#[repr(C)]
pub struct VkExportFenceCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalFenceHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
}

#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
    pub subsetAllocation: VkBool32,
}

#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlags,
    pub deviceMask: u32,
}

#[repr(C)]
pub struct VkBindBufferMemoryInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

#[repr(C)]
pub struct VkBindImageMemoryInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub splitInstanceBindRegionCount: u32,
    pub pSplitInstanceBindRegions: *const VkRect2D,
}

#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}

#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

#[repr(C)]
pub struct VkDeviceGroupSubmitInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry{
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: isize,
    pub stride: isize,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlags,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
    pub templateType: VkDescriptorUpdateTemplateType,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}

#[repr(C)]
pub struct VkInputAttachmentAspectReference{
    pub subpass: u32,
    pub inputAttachmentIndex: u32,
    pub aspectMask: VkImageAspectFlags,
}

#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub aspectReferenceCount: u32,
    pub pAspectReferences: *const VkInputAttachmentAspectReference,
}

#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub subgroupSize: u32,
    pub supportedStages: VkShaderStageFlags,
    pub supportedOperations: VkSubgroupFeatureFlags,
    pub quadOperationsInAllStages: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderSubgroupExtendedTypes: VkBool32,
}

#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
}

#[repr(C)]
pub struct VkMemoryRequirements2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements2{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkSparseImageMemoryRequirements,
}

#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub pointClippingBehavior: VkPointClippingBehavior,
}

#[repr(C)]
pub struct VkMemoryDedicatedRequirements{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
}

#[repr(C)]
pub struct VkImageViewUsageCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub usage: VkImageUsageFlags,
}

#[derive(Eq, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub domainOrigin: VkTessellationDomainOrigin,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub conversion: VkSamplerYcbcrConversion,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub ycbcrModel: VkSamplerYcbcrModelConversion,
    pub ycbcrRange: VkSamplerYcbcrRange,
    pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocation,
    pub yChromaOffset: VkChromaLocation,
    pub chromaFilter: VkFilter,
    pub forceExplicitReconstruction: VkBool32,
}

#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub samplerYcbcrConversion: VkBool32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub combinedImageSamplerDescriptorCount: u32,
}

#[repr(C)]
pub struct VkProtectedSubmitInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub protectedSubmit: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub protectedMemory: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub protectedNoFault: VkBool32,
}

#[repr(C)]
pub struct VkDeviceQueueInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueIndex: u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
}

#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub reductionMode: VkSamplerReductionMode,
}

#[repr(C)]
pub struct VkImageFormatListCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutSupport{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supported: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderDrawParameters: VkBool32,
}
pub type VkPhysicalDeviceShaderDrawParameterFeatures = VkPhysicalDeviceShaderDrawParametersFeatures;

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub hostQueryReset: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlags,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxVariableDescriptorCount: u32,
}

#[repr(C)]
pub struct VkAttachmentDescription2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
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
pub struct VkAttachmentReference2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachment: u32,
    pub layout: VkImageLayout,
    pub aspectMask: VkImageAspectFlags,
}

#[repr(C)]
pub struct VkSubpassDescription2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub viewMask: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference2,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference2,
    pub pResolveAttachments: *const VkAttachmentReference2,
    pub pDepthStencilAttachment: *const VkAttachmentReference2,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[repr(C)]
pub struct VkSubpassDependency2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
    pub viewOffset: i32,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo2{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription2,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription2,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency2,
    pub correlatedViewMaskCount: u32,
    pub pCorrelatedViewMasks: *const u32,
}

#[repr(C)]
pub struct VkSubpassBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub contents: VkSubpassContents,
}

#[repr(C)]
pub struct VkSubpassEndInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub timelineSemaphore: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub maxTimelineSemaphoreValueDifference: u64,
}

#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphoreType: VkSemaphoreType,
    pub initialValue: u64,
}

#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreValueCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValueCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

#[repr(C)]
pub struct VkSemaphoreWaitInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreWaitFlags,
    pub semaphoreCount: u32,
    pub pSemaphores: *const VkSemaphore,
    pub pValues: *const u64,
}

#[repr(C)]
pub struct VkSemaphoreSignalInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
}

#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
}

#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub depthResolveMode: VkResolveModeFlagBits,
    pub stencilResolveMode: VkResolveModeFlagBits,
    pub pDepthStencilResolveAttachment: *const VkAttachmentReference2,
}

#[repr(C)]
pub struct VkImageStencilUsageCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub stencilUsage: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub scalarBlockLayout: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub uniformBufferStandardLayout: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub opaqueCaptureAddress: u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub imagelessFramebuffer: VkBool32,
}

#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentImageInfoCount: u32,
    pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
}

#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub usage: VkImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layerCount: u32,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
}

#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub separateDepthStencilLayouts: VkBool32,
}

#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stencilLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub stencilInitialLayout: VkImageLayout,
    pub stencilFinalLayout: VkImageLayout,
}

#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub opaqueCaptureAddress: u64,
}

#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
    pub protectedMemory: VkBool32,
    pub samplerYcbcrConversion: VkBool32,
    pub shaderDrawParameters: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
    pub subgroupSize: u32,
    pub subgroupSupportedStages: VkShaderStageFlags,
    pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
    pub subgroupQuadOperationsInAllStages: VkBool32,
    pub pointClippingBehavior: VkPointClippingBehavior,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
    pub protectedNoFault: VkBool32,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features{
    pub sType: VkStructureType,
    pub pNext: c_void,
    pub samplerMirrorClampToEdge: VkBool32,
    pub drawIndirectCount: VkBool32,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
    pub descriptorIndexing: VkBool32,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
    pub samplerFilterMinmax: VkBool32,
    pub scalarBlockLayout: VkBool32,
    pub imagelessFramebuffer: VkBool32,
    pub uniformBufferStandardLayout: VkBool32,
    pub shaderSubgroupExtendedTypes: VkBool32,
    pub separateDepthStencilLayouts: VkBool32,
    pub hostQueryReset: VkBool32,
    pub timelineSemaphore: VkBool32,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
    pub shaderOutputViewportIndex: VkBool32,
    pub shaderOutputLayer: VkBool32,
    pub subgroupBroadcastDynamicId: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties{
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub driverID: VkDriverId,
    pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE],
    pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE],
    pub conformanceVersion: VkConformanceVersion,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
    pub maxTimelineSemaphoreValueDifference: u64,
    pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
}

#[cfg(windows)]
static LIBRARY_NAME: &'static str = "vulkan-1";
#[cfg(linux)]
static LIBRARY_NAME: &'static str = "vulkan";

lazy_static! {
    static ref vulkan_lib: libloading::Library = libloading::Library::new(LIBRARY_NAME).unwrap();
    pub static ref vkGetInstanceProcAddr: libloading::Symbol<'static, unsafe extern "C" fn(instance: VkInstance, pName: *const c_char)->PFN_vkVoidFunction> = unsafe { vulkan_lib.get(b"vkGetInstanceProcAddr") }.unwrap();
    pub static ref vkEnumerateInstanceExtensionProperties: unsafe extern "C" fn(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties)->VkResult = unsafe {transmute(vkGetInstanceProcAddr(VkInstance::none(), b"vkEnumerateInstanceExtensionProperties\0".as_ptr() as *const c_char))};
    pub static ref vkEnumerateInstanceLayerProperties: unsafe extern "C" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties)->VkResult = unsafe {transmute(vkGetInstanceProcAddr(VkInstance::none(), b"vkEnumerateInstanceLayerProperties\0".as_ptr() as *const c_char))};
    pub static ref vkCreateInstance: unsafe extern "C" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance)->VkResult = unsafe {transmute(vkGetInstanceProcAddr(VkInstance::none(), b"vkCreateInstance\0".as_ptr() as *const c_char))};
    // vulkan 1.1
    pub static ref vkEnumerateInstanceVersion: unsafe extern "C" fn (pApiVersion: *mut u32) = unsafe {transmute(vkGetInstanceProcAddr(VkInstance::none(), b"vkEnumerateInstanceVersion\0".as_ptr() as *const c_char))};
}

core_functions! {
    fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice)->VkResult;
    fn vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char)->PFN_vkVoidFunction;
    fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
    fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
    fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
    fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
    fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties)->VkResult;
    fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice)->VkResult;
    fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
    fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties)->VkResult;
    fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties)->VkResult;
    fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
    fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence)->VkResult;
    fn vkQueueWaitIdle(queue: VkQueue)->VkResult;
    fn vkDeviceWaitIdle(device: VkDevice)->VkResult;
    fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory)->VkResult;
    fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
    fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void)->VkResult;
    fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
    fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange)->VkResult;
    fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange)->VkResult;
    fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
    fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *const VkMemoryRequirements);
    fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize)->VkResult;
    fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
    fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize)->VkResult;
    fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
    fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
    fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence)->VkResult;
    fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence)->VkResult;
    fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
    fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence)->VkResult;
    fn vkGetFenceStatus(device: VkDevice, fence: VkFence)->VkResult;
    fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64)->VkResult;
    fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore)->VkResult;
    fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent)->VkResult;
    fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
    fn vkGetEventStatus(device: VkDevice, event: VkEvent)->VkResult;
    fn vkSetEvent(device: VkDevice, event: VkEvent)->VkResult;
    fn vkResetEvent(device: VkDevice, event: VkEvent)->VkResult;
    fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool)->VkResult;
    fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
    fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: isize, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags)->VkResult;
    fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer)->VkResult;
    fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView)->VkResult;
    fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage)->VkResult;
    fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
    fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
    fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView)->VkResult;
    fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule)->VkResult;
    fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
    fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache)->VkResult;
    fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
    fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut isize, pData: *mut c_void)->VkResult;
    fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache)->VkResult;
    fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline)->VkResult;
    fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline)->VkResult;
    fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
    fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout)->VkResult;
    fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler)->VkResult;
    fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout)->VkResult;
    fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool)->VkResult;
    fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
    fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags)->VkResult;
    fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet)->VkResult;
    fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet)->VkResult;
    fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
    fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer)->VkResult;
    fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
    fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass)->VkResult;
    fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
    fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
    fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool)->VkResult;
    fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
    fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags)->VkResult;
    fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer)->VkResult;
    fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo)->VkResult;
    fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer)->VkResult;
    fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags)->VkResult;
    fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
    fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
    fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
    fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32);
    fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);
    fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const [f32; 4]);
    fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
    fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
    fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
    fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
    fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
    fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
    fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
    fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
    fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
    fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
    fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
    fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
    fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
    fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
    fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
    fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const c_void);
    fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
    fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
    fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
    fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
    fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
    fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
    fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
    fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
    fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
    fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const c_void);
    fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
    fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
    fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
    // vulkan 1.1
    fn vkBindBufferMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo)->VkResult;
    fn vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo)->VkResult;
    fn vkCmdDispatchBase(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32);
    fn vkCreateDescriptorUpdateTemplate(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate)->VkResult;
    fn vkCreateSamplerYcbcrConversion(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion)->VkResult;
    fn vkDestroyDescriptorUpdateTemplate(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);
    fn vkDestroySamplerYcbcrConversion(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);
    fn vkEnumeratePhysicalDeviceGroups(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties)->VkResult;
    fn vkGetBufferMemoryRequirements2(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
    fn vkGetDescriptorSetLayoutSupport(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
    fn vkGetDeviceGroupPeerMemoryFeatures(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
    fn vkGetDeviceQueue2(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);
    fn vkGetImageMemoryRequirements2(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
    fn vkGetImageSparseMemoryRequirements2(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
    fn vkGetPhysicalDeviceExternalBufferProperties(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);
    fn vkGetPhysicalDeviceExternalFenceProperties(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);
    fn vkGetPhysicalDeviceExternalSemaphoreProperties(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);
    fn vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
    fn vkGetPhysicalDeviceFormatProperties2(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
    fn vkGetPhysicalDeviceImageFormatProperties2(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2)->VkResult;
    fn vkGetPhysicalDeviceMemoryProperties2(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
    fn vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
    fn vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
    fn vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);
    fn vkTrimCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
    fn vkUpdateDescriptorSetWithTemplate(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const c_void);
    // vulkan 1.2
    fn vkGetBufferDeviceAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo)->VkDeviceAddress;
    fn vkGetBufferOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo)->u64;
    fn vkGetDeviceMemoryOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo)->u64;
    fn vkCreateRenderPass2(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass)->VkResult;
    fn vkCmdBeginRenderPass2(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);
    fn vkCmdNextSubpass2(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);
    fn vkCmdEndRenderPass2(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
    fn vkCmdDrawIndirectCount(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    fn vkCmdDrawIndexedIndirectCount(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
    fn vkGetSemaphoreCounterValue(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64)->VkResult;
    fn vkWaitSemaphores(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64)->VkResult;
    fn vkSignalSemaphore(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo)->VkResult;
    fn vkResetQueryPool(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
}