#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr;

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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkWin32SurfaceCreateInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl Default for VkWin32SurfaceCreateInfoKHR{
    fn default() -> Self {
        VkWin32SurfaceCreateInfoKHR{
            sType: extend_core_enums::VkStructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            hinstance: Default::default(),
            hwnd: Default::default(),
        }
    }
}

instance_level_functions! {
    fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR)->VkResult;
    fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32)->VkBool32;
}