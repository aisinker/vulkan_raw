#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::os::raw::c_char;
use std::fmt::{Display, Formatter, Error};

use crate::*;

handle!(VkDebugUtilsMessengerEXT, NonDispatchableHandle);

bitmasks!{
    {
        VkDebugUtilsMessengerCreateFlagsEXT,
        enum VkDebugUtilsMessengerCreateFlagBitsEXT{
            _RESERVED = 0,
        }
    },
    {
        VkDebugUtilsMessengerCallbackDataFlagsEXT,
        enum VkDebugUtilsMessengerCallbackDataFlagBitsEXT{
            _RESERVED = 0,
        }
    },
    {
        VkDebugUtilsMessageSeverityFlagsEXT,
        enum VkDebugUtilsMessageSeverityFlagBitsEXT{
            VERBOSE_BIT_EXT = 0x00000001,
            INFO_BIT_EXT = 0x00000010,
            WARNING_BIT_EXT = 0x00000100,
            ERROR_BIT_EXT = 0x00001000,
        }
    },
    {
        VkDebugUtilsMessageTypeFlagsEXT,
        enum VkDebugUtilsMessageTypeFlagBitsEXT{
            GENERAL_BIT_EXT = 0x00000001,
            VALIDATION_BIT_EXT = 0x00000002,
            PERFORMANCE_BIT_EXT = 0x00000004,
        }
    },
}

extend_core_enums!{
    enum VkStructureType{
        DEBUG_UTILS_OBJECT_NAME_INFO_EXT = 1000128000,
        DEBUG_UTILS_OBJECT_TAG_INFO_EXT = 1000128001,
        DEBUG_UTILS_LABEL_EXT = 1000128002,
        DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT = 1000128003,
        DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT = 1000128004,
    },
    enum VkResult{
        ERROR_VALIDATION_FAILED_EXT = -1000011001,
    },
}

pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "C" fn(messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT, pUserData: *mut c_void)->VkBool32;

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

extension_functions! {
    fn vkSetDebugUtilsObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT)->VkResult;
    fn vkSetDebugUtilsObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT)->VkResult;
    fn vkQueueBeginDebugUtilsLabelEXT(queue: core::VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkQueueEndDebugUtilsLabelEXT(queue: core::VkQueue);
    fn vkQueueInsertDebugUtilsLabelEXT(queue: core::VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCmdBeginDebugUtilsLabelEXT(commandBuffer: core::VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: core::VkCommandBuffer);
    fn vkCmdInsertDebugUtilsLabelEXT(commandBuffer: core::VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);
    fn vkCreateDebugUtilsMessengerEXT(instance: VkInstance, pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT, pAllocator: *const core::VkAllocationCallbacks, pMessenger: *mut VkDebugUtilsMessengerEXT)->VkResult;
    fn vkDestroyDebugUtilsMessengerEXT(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const core::VkAllocationCallbacks);
    fn vkSubmitDebugUtilsMessageEXT(instance: VkInstance, messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT, messageTypes: VkDebugUtilsMessageTypeFlagsEXT, pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT);
}
