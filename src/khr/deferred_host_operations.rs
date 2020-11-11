#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

handle!(VkDeferredOperationKHR, NonDispatchableHandle);

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDeferredOperationInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub operationHandle: VkDeferredOperationKHR,
}
impl Default for VkDeferredOperationInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::DEFERRED_OPERATION_INFO_KHR,
            pNext: ptr::null(),
            operationHandle: VkDeferredOperationKHR::none(),
        }
    }
}
