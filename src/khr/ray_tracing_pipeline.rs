#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

enums! {
    enum VkRayTracingShaderGroupTypeKHR {
        GENERAL_KHR = 0,
        TRIANGLES_HIT_GROUP_KHR = 1,
        PROCEDURAL_HIT_GROUP_KHR = 2,
    },
    enum VkShaderGroupShaderKHR {
        GENERAL_KHR = 0,
        CLOSEST_HIT_KHR = 1,
        ANY_HIT_KHR = 2,
        INTERSECTION_KHR = 3,
    },
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkRayTracingPipelineCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub groupCount: u32,
    pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
    pub maxPipelineRayRecursionDepth: u32,
    pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
    pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}
impl Default for VkRayTracingPipelineCreateInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            stageCount: Default::default(),
            pStages: ptr::null(),
            groupCount: Default::default(),
            pGroups: ptr::null(),
            maxPipelineRayRecursionDepth: Default::default(),
            pLibraryInfo: ptr::null(),
            pLibraryInterface: ptr::null(),
            pDynamicState: ptr::null(),
            layout: Default::default(),
            basePipelineHandle: Default::default(),
            basePipelineIndex: -1,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxPipelineRayPayloadSize: u32,
    pub maxPipelineRayHitAttributeSize: u32,
}
impl Default for VkRayTracingPipelineInterfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            maxPipelineRayPayloadSize: Default::default(),
            maxPipelineRayHitAttributeSize: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub r#type: VkRayTracingShaderGroupTypeKHR,
    pub generalShader: u32,
    pub closestHitShader: u32,
    pub anyHitShader: u32,
    pub intersectionShader: u32,
    pub pShaderGroupCaptureReplayHandle: *const c_void,
}
impl Default for VkRayTracingShaderGroupCreateInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
            pNext: ptr::null(),
            r#type: VkRayTracingShaderGroupTypeKHR::GENERAL_KHR,
            generalShader: VK_SHADER_UNUSED_KHR,
            closestHitShader: VK_SHADER_UNUSED_KHR,
            anyHitShader: VK_SHADER_UNUSED_KHR,
            intersectionShader: VK_SHADER_UNUSED_KHR,
            pShaderGroupCaptureReplayHandle: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VkStridedDeviceAddressRegionKHR {
    pub deviceAddress: VkDeviceAddress,
    pub stride: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VkTraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayTracingPipeline: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
    pub rayTracingPipelineTraceRaysIndirect: VkBool32,
    pub rayTraversalPrimitiveCulling: VkBool32,
}
impl Default for VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
            pNext: ptr::null_mut(),
            rayTracingPipeline: Default::default(),
            rayTracingPipelineShaderGroupHandleCaptureReplay: Default::default(),
            rayTracingPipelineShaderGroupHandleCaptureReplayMixed: Default::default(),
            rayTracingPipelineTraceRaysIndirect: Default::default(),
            rayTraversalPrimitiveCulling: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRayRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub shaderGroupHandleCaptureReplaySize: u32,
    pub maxRayDispatchInvocationCount: u32,
    pub shaderGroupHandleAlignment: u32,
    pub maxRayHitAttributeSize: u32,
}
impl Default for VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
            pNext: ptr::null_mut(),
            shaderGroupHandleSize: Default::default(),
            maxRayRecursionDepth: Default::default(),
            maxShaderGroupStride: Default::default(),
            shaderGroupBaseAlignment: Default::default(),
            shaderGroupHandleCaptureReplaySize: Default::default(),
            maxRayDispatchInvocationCount: Default::default(),
            shaderGroupHandleAlignment: Default::default(),
            maxRayHitAttributeSize: Default::default(),
        }
    }
}
