#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;

use crate::*;
use crate::khr::surface::*;

bitmasks!{
    {
        VkWin32SurfaceCreateFlagsKHR,
        enum VkWin32SurfaceCreateFlagBitsKHR{
            _RESERVED = 0,
        }
    }
}

extend_core_enums!{
    enum VkStructureType{
        WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,
    },
}

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

extension_functions! {
    fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const core::VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR)->VkResult;
    fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32)->VkBool32;
}