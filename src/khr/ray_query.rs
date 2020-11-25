#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub rayQuery: VkBool32,
}
impl Default for VkPhysicalDeviceRayQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
            pNext: ptr::null_mut(),
            rayQuery: VkBool32::FALSE,
        }
    }
}
