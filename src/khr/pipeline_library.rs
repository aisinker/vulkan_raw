#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPipelineLibraryCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub libraryCount: u32,
    pub pLibraries: *const VkPipeline,
}
impl Default for VkPipelineLibraryCreateInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR,
            pNext: ptr::null(),
            libraryCount: 0,
            pLibraries: ptr::null(),
        }
    }
}
