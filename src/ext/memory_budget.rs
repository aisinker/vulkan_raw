#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::ptr;

pub const SPEC_VERSION: u32 = 1;
pub const EXTENSION_NAME: *const c_char = b"VK_EXT_memory_budget\0".as_ptr() as *const c_char;

extend_core_enums! {
    enum VkStructureType{
        PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT = 1000237000
    },
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
    pub heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS],
}
impl Default for VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> Self {
        Self {
            sType: extend_core_enums::VkStructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            pNext: ptr::null_mut(),
            heapBudget: Default::default(),
            heapUsage: Default::default(),
        }
    }
}
