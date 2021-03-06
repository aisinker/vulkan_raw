#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

// pub const SPEC_VERSION: u32 = 1;
// pub const EXTENSION_NAME: *const c_char = b"VK_KHR_external_fence_win32\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub flags: VkFenceImportFlags,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl Default for VkImportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        VkImportFenceWin32HandleInfoKHR {
            sType: VkStructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handleType: Default::default(),
            handle: Default::default(),
            name: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pAttributes: *const SECURITY_ATTRIBUTES,
    pub dwAccess: DWORD,
    pub name: LPCWSTR,
}
impl Default for VkExportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        VkExportFenceWin32HandleInfoKHR {
            sType: VkStructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            pAttributes: ptr::null(),
            dwAccess: Default::default(),
            name: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub fence: VkFence,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}
impl Default for VkFenceGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            pNext: ptr::null(),
            fence: Default::default(),
            handleType: Default::default(),
        }
    }
}
