#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::ffi::c_void;
// use std::os::raw::c_char;
use std::ptr;

// pub const SPEC_VERSION: u32 = 1;
// pub const EXTENSION_NAME: *const c_char = b"VK_EXT_index_type_uint8\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub indexTypeUint8: VkBool32,
}
impl Default for VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT,
            pNext: ptr::null_mut(),
            indexTypeUint8: VkBool32::FALSE,
        }
    }
}
