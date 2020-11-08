#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::ptr;

use crate::*;
// use std::os::raw::c_char;

// pub const SPEC_VERSION: u32 = 6;
// pub const EXTENSION_NAME: *const c_char = b"VK_KHR_win32_surface\0".as_ptr() as *const c_char;

bitmasks! {
    VkWin32SurfaceCreateFlagsKHR = enum VkWin32SurfaceCreateFlagBitsKHR{
        _RESERVED = 0,
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl Default for VkWin32SurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWin32SurfaceCreateInfoKHR {
            sType: VkStructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            hinstance: Default::default(),
            hwnd: Default::default(),
        }
    }
}
