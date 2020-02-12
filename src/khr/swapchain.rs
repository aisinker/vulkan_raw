#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::fmt::{Display, Formatter, Error};

use super::super::core::*;
use super::surface::*;
use super::super::NonDispatchableHandle;

use bitflags::bitflags;

handle!(VkSwapchainKHR, NonDispatchableHandle);

pub type VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagBitsKHR;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDeviceGroupPresentModeFlagBitsKHR: u32 {
    	    const LOCAL_BIT_KHR = 0x00000001;
            const REMOTE_BIT_KHR = 0x00000002;
            const SUM_BIT_KHR = 0x00000004;
            const LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008;
        }
    }

pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkSwapchainCreateFlagBitsKHR: u32 {
    	    const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001;
            const PROTECTED_BIT_KHR = 0x00000002;
            const MUTABLE_FORMAT_BIT_KHR = 0x00000004;
        }
    }

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR{
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

#[repr(C)]
pub struct VkPresentInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
}

#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}

#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub mode: VkDeviceGroupPresentModeFlagsKHR,
}

extern_c_functions! {
    fn vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR)->VkResult;
    fn vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
    fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage)->VkResult;
    fn vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32)->VkResult;
    fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR)->VkResult;
    fn vkGetDeviceGroupPresentCapabilitiesKHR(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR)->VkResult;
    fn vkGetDeviceGroupSurfacePresentModesKHR(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR)->VkResult;
    fn vkGetPhysicalDevicePresentRectanglesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D)->VkResult;
    fn vkAcquireNextImage2KHR(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32)->VkResult;
}
