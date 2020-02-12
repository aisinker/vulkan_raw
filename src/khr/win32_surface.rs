#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

use super::super::core::*;
use super::surface::*;

use bitflags::bitflags;

reserved_flags!(VkWin32SurfaceCreateFlagsKHR);

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

extern_c_functions! {
    fn vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR)->VkResult;
    fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32)->VkBool32;
}