#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::fmt::{Display, Error, Formatter};
// use std::os::raw::c_char;

// pub const SPEC_VERSION: u32 = 25;
// pub const EXTENSION_NAME: *const c_char = b"VK_KHR_surface\0".as_ptr() as *const c_char;

handle!(
    VkSurfaceKHR,
    NonDispatchableHandle,
    VkObjectType::SURFACE_KHR
);

bitmasks! {
    VkSurfaceTransformFlagsKHR = enum VkSurfaceTransformFlagBitsKHR{
        IDENTITY_BIT_KHR = 0x00000001,
        ROTATE_90_BIT_KHR = 0x00000002,
        ROTATE_180_BIT_KHR = 0x00000004,
        ROTATE_270_BIT_KHR = 0x00000008,
        HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
        HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
        HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
        HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
        INHERIT_BIT_KHR = 0x00000100,
    },
    VkCompositeAlphaFlagsKHR = enum VkCompositeAlphaFlagBitsKHR{
        OPAQUE_BIT_KHR = 0x00000001,
        PRE_MULTIPLIED_BIT_KHR = 0x00000002,
        POST_MULTIPLIED_BIT_KHR = 0x00000004,
        INHERIT_BIT_KHR = 0x00000008,
    },
}

enums! {
    enum VkPresentModeKHR{
        IMMEDIATE_KHR = 0,
        MAILBOX_KHR = 1,
        FIFO_KHR = 2,
        FIFO_RELAXED_KHR = 3,
        SHARED_DEMAND_REFRESH_KHR = 1000111000,
        SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
    },
    enum VkColorSpaceKHR{
        SRGB_NONLINEAR_KHR = 0,
        DISPLAY_P3_NONLINEAR_EXT = 1000104001,
        EXTENDED_SRGB_LINEAR_EXT = 1000104002,
        DISPLAY_P3_LINEAR_EXT = 1000104003,
        DCI_P3_NONLINEAR_EXT = 1000104004,
        BT709_LINEAR_EXT = 1000104005,
        BT709_NONLINEAR_EXT = 1000104006,
        BT2020_LINEAR_EXT = 1000104007,
        HDR10_ST2084_EXT = 1000104008,
        DOLBYVISION_EXT = 1000104009,
        HDR10_HLG_EXT = 1000104010,
        ADOBERGB_LINEAR_EXT = 1000104011,
        ADOBERGB_NONLINEAR_EXT = 1000104012,
        PASS_THROUGH_EXT = 1000104013,
        EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
        DISPLAY_NATIVE_AMD = 1000213000,
    },
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}
impl Default for VkSurfaceFormatKHR {
    fn default() -> Self {
        VkSurfaceFormatKHR {
            format: VkFormat::UNDEFINED,
            colorSpace: VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}
