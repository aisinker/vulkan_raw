#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fmt::{Display, Formatter, Error};

use super::super::core::*;
use super::super::NonDispatchableHandle;

use bitflags::bitflags;

handle!(VkSurfaceKHR, NonDispatchableHandle);

pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;
bitflags! {
        #[repr(C)]
        #[derive(Default)]
        pub struct VkSurfaceTransformFlagBitsKHR: u32 {
            const IDENTITY_BIT_KHR = 0x00000001;
            const ROTATE_90_BIT_KHR = 0x00000002;
            const ROTATE_180_BIT_KHR = 0x00000004;
            const ROTATE_270_BIT_KHR = 0x00000008;
            const HORIZONTAL_MIRROR_BIT_KHR = 0x00000010;
            const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020;
            const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040;
            const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080;
            const INHERIT_BIT_KHR = 0x00000100;
        }
    }

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkPresentModeKHR{
    IMMEDIATE_KHR = 0,
    MAILBOX_KHR = 1,
    FIFO_KHR = 2,
    FIFO_RELAXED_KHR = 3,
    SHARED_DEMAND_REFRESH_KHR = 1000111000,
    SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VkColorSpaceKHR{
    SRGB_NONLINEAR_KHR = 0,
    DISPLAY_P3_NONLINEAR_EXT = 1000104001,
    EXTENDED_SRGB_LINEAR_EXT = 1000104002,
    DCI_P3_LINEAR_EXT = 1000104003,
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
}

pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkCompositeAlphaFlagBitsKHR: u32 {
    	    const OPAQUE_BIT_KHR = 0x00000001;
            const PRE_MULTIPLIED_BIT_KHR = 0x00000002;
            const POST_MULTIPLIED_BIT_KHR = 0x00000004;
            const INHERIT_BIT_KHR = 0x00000008;
        }
    }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct VkSurfaceFormatKHR{
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR{
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

extern_c_functions! {
    fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
    fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32)->VkResult;
    fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR)->VkResult;
    fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR)->VkResult;
    fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR)->VkResult;
}