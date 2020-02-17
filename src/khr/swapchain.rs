#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt::{Display, Formatter, Error};

use crate::core;
use crate::khr::surface::*;
use crate::NonDispatchableHandle;

handle!(VkSwapchainKHR, NonDispatchableHandle);

bitmasks!{
    {
        VkDeviceGroupPresentModeFlagsKHR,
        enum VkDeviceGroupPresentModeFlagBitsKHR{
            LOCAL_BIT_KHR = 0x00000001,
            REMOTE_BIT_KHR = 0x00000002,
            SUM_BIT_KHR = 0x00000004,
            LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
        }
    },
    {
        VkSwapchainCreateFlagsKHR,
        enum VkSwapchainCreateFlagBitsKHR{
            SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
            PROTECTED_BIT_KHR = 0x00000002,
            MUTABLE_FORMAT_BIT_KHR = 0x00000004,
        }
    },
}

extend_core_enums!{
    enum VkStructureType{
        SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
        PRESENT_INFO_KHR = 1000001001,
        DEVICE_GROUP_PRESENT_CAPABILITIES_KHR = 1000060007,
        IMAGE_SWAPCHAIN_CREATE_INFO_KHR = 1000060008,
        BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR = 1000060009,
        ACQUIRE_NEXT_IMAGE_INFO_KHR = 1000060010,
        DEVICE_GROUP_PRESENT_INFO_KHR = 1000060011,
        DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR = 1000060012,
    },
    enum VkImageLayout{
        PRESENT_SRC_KHR = 1000001002,
    },
    enum VkResult{
        SUBOPTIMAL_KHR = 1000001003,
        ERROR_OUT_OF_DATE_KHR = -1000001004,
    },
}

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: crate::VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: core::VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: crate::VkImageUsageFlags,
    pub imageSharingMode: crate::VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: crate::VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

#[repr(C)]
pub struct VkPresentInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const core::VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut crate::VkResult,
}

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; crate::VK_MAX_DEVICE_GROUP_SIZE],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
}

#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: core::VkSemaphore,
    pub fence: core::VkFence,
    pub deviceMask: u32,
}

#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR{
    pub sType: crate::VkStructureType,
    pub pNext: *const c_void,
    pub mode: VkDeviceGroupPresentModeFlagsKHR,
}

extension_functions! {
    fn vkCreateSwapchainKHR(device: core::VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR)->crate::VkResult;
    fn vkDestroySwapchainKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const core::VkAllocationCallbacks);
    fn vkGetSwapchainImagesKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut core::VkImage)->crate::VkResult;
    fn vkAcquireNextImageKHR(device: core::VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: core::VkSemaphore, fence: core::VkFence, pImageIndex: *mut u32)->crate::VkResult;
    fn vkQueuePresentKHR(queue: core::VkQueue, pPresentInfo: *const VkPresentInfoKHR)->crate::VkResult;
    fn vkGetDeviceGroupPresentCapabilitiesKHR(device: core::VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR)->crate::VkResult;
    fn vkGetDeviceGroupSurfacePresentModesKHR(device: core::VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR)->crate::VkResult;
    fn vkGetPhysicalDevicePresentRectanglesKHR(physicalDevice: core::VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut core::VkRect2D)->crate::VkResult;
    fn vkAcquireNextImage2KHR(device: core::VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32)->crate::VkResult;
}
