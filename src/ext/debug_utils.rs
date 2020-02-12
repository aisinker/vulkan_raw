#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::os::raw::c_char;
use std::fmt::{Display, Formatter, Error};

use super::super::core::*;
use super::super::NonDispatchableHandle;

use bitflags::bitflags;

handle!(VkDebugUtilsMessengerEXT, NonDispatchableHandle);

pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "C" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut c_void)->VkBool32;

reserved_flags!(
    VkDebugUtilsMessengerCreateFlagsEXT,
    VkDebugUtilsMessengerCallbackDataFlagsEXT
);

pub type VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDebugUtilsMessageSeverityFlagBitsEXT: u32 {
    	    const VERBOSE_BIT_EXT = 0x00000001;
            const INFO_BIT_EXT = 0x00000010;
            const WARNING_BIT_EXT = 0x00000100;
            const ERROR_BIT_EXT = 0x00001000;
        }
    }

pub type VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagBitsEXT;
bitflags! {
        #[repr(C)]
	    #[derive(Default)]
        pub struct VkDebugUtilsMessageTypeFlagBitsEXT: u32 {
    	    const GENERAL_BIT_EXT = 0x00000001;
            const VALIDATION_BIT_EXT = 0x00000002;
            const PERFORMANCE_BIT_EXT = 0x00000004;
        }
    }

#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: isize,
    pub pTag: *const c_void,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const char,
    pub color: [f32; 4],
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT{
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

extern_c_functions! {
    fn vkSetDebugUtilsObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT)->VkResult;
    fn vkSetDebugUtilsObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT)->VkResult;
    fn vkQueueBeginDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue);
    fn vkQueueInsertDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCmdBeginDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer);
    fn vkCmdInsertDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCreateDebugUtilsMessengerEXT(instance: VkInstance, pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pMessenger: *mut VkDebugUtilsMessengerEXT)->VkResult;
    fn vkDestroyDebugUtilsMessengerEXT(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks);
    fn vkSubmitDebugUtilsMessageEXT(instance: VkInstance, messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT);
}
