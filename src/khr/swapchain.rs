#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt::{Display, Error, Formatter};
use std::ptr;

use crate::khr::surface::*;
use crate::NonDispatchableHandle;
use crate::*;
// use std::os::raw::c_char;

// pub const SPEC_VERSION: u32 = 70;
// pub const EXTENSION_NAME: *const c_char = b"VK_KHR_swapchain\0".as_ptr() as *const c_char;

handle!(
    VkSwapchainKHR,
    NonDispatchableHandle,
    VkObjectType::SWAPCHAIN_KHR
);

bitmasks! {
    VkDeviceGroupPresentModeFlagsKHR = enum VkDeviceGroupPresentModeFlagBitsKHR{
        LOCAL_BIT_KHR = 0x00000001,
        REMOTE_BIT_KHR = 0x00000002,
        SUM_BIT_KHR = 0x00000004,
        LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
    },
    VkSwapchainCreateFlagsKHR = enum VkSwapchainCreateFlagBitsKHR{
        SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
        PROTECTED_BIT_KHR = 0x00000002,
        MUTABLE_FORMAT_BIT_KHR = 0x00000004,
    },
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}
impl Default for VkSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkSwapchainCreateInfoKHR {
            sType: VkStructureType::SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            minImageCount: Default::default(),
            imageFormat: VkFormat::UNDEFINED,
            imageColorSpace: VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
            imageExtent: Default::default(),
            imageArrayLayers: 1,
            imageUsage: Default::default(),
            imageSharingMode: VkSharingMode::EXCLUSIVE,
            queueFamilyIndexCount: Default::default(),
            pQueueFamilyIndices: ptr::null(),
            preTransform: VkSurfaceTransformFlagBitsKHR::IDENTITY_BIT_KHR,
            compositeAlpha: VkCompositeAlphaFlagBitsKHR::OPAQUE_BIT_KHR,
            presentMode: VkPresentModeKHR::IMMEDIATE_KHR,
            clipped: Default::default(),
            oldSwapchain: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}
impl Default for VkPresentInfoKHR {
    fn default() -> Self {
        VkPresentInfoKHR {
            sType: VkStructureType::PRESENT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreCount: Default::default(),
            pWaitSemaphores: ptr::null(),
            swapchainCount: Default::default(),
            pSwapchains: ptr::null(),
            pImageIndices: ptr::null(),
            pResults: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupPresentCapabilitiesKHR {
    fn default() -> Self {
        VkDeviceGroupPresentCapabilitiesKHR {
            sType: VkStructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            pNext: ptr::null(),
            presentMask: Default::default(),
            modes: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkImageSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
}
impl Default for VkImageSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkImageSwapchainCreateInfoKHR {
            sType: VkStructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            swapchain: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkBindImageMemorySwapchainInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}
impl Default for VkBindImageMemorySwapchainInfoKHR {
    fn default() -> Self {
        VkBindImageMemorySwapchainInfoKHR {
            sType: VkStructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            pNext: ptr::null(),
            swapchain: Default::default(),
            imageIndex: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAcquireNextImageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}
impl Default for VkAcquireNextImageInfoKHR {
    fn default() -> Self {
        VkAcquireNextImageInfoKHR {
            sType: VkStructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            pNext: ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            deviceMask: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDeviceGroupPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}
impl Default for VkDeviceGroupPresentInfoKHR {
    fn default() -> Self {
        VkDeviceGroupPresentInfoKHR {
            sType: VkStructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            swapchainCount: Default::default(),
            pDeviceMasks: ptr::null(),
            mode: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}
impl Default for VkDeviceGroupSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkDeviceGroupSwapchainCreateInfoKHR {
            sType: VkStructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            modes: Default::default(),
        }
    }
}
