#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_int, c_void};
use std::ptr;

use crate::*;

// pub const SPEC_VERSION: u32 = 1;
// pub const EXTENSION_NAME: *const c_char = b"VK_KHR_external_fence_fd\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkImportFenceFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub fd: c_int,
}
impl Default for VkImportFenceFdInfoKHR {
    fn default() -> Self {
        VkImportFenceFdInfoKHR {
            sType: VkStructureType::IMPORT_FENCE_FD_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            fd: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkFenceGetFdInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetFdInfoKHR {
    fn default() -> Self {
        VkFenceGetFdInfoKHR {
            sType: VkStructureType::FENCE_GET_FD_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            handleType: Default::default(),
        }
    }
}
