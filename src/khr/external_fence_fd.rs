#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::{c_int, c_void};

use crate::*;

extend_core_enums!{
    enum VkStructureType{
        IMPORT_FENCE_FD_INFO_KHR = 1000115000,
        FENCE_GET_FD_INFO_KHR = 1000115001,
    }
}

#[repr(C)]
pub struct VkImportFenceFdInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub fd: c_int,
}

#[repr(C)]
pub struct VkFenceGetFdInfoKHR{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

device_extension_functions!{
    fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR)->VkResult;
    fn vkGetFenceFdKHR(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut c_int)->VkResult;
}