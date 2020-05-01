#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt::{Display, Error, Formatter};
use std::os::raw::c_char;
use std::ptr;

use crate::*;

handle!(VkDebugUtilsMessengerEXT, NonDispatchableHandle);

bitmasks! {
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

extend_core_enums! {
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

pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "C" fn(
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> VkBool32;

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}
impl Default for VkDebugUtilsObjectNameInfoEXT {
    fn default() -> Self {
        VkDebugUtilsObjectNameInfoEXT {
            sType: extend_core_enums::VkStructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            pNext: ptr::null(),
            objectType: VkObjectType::UNKNOWN,
            objectHandle: Default::default(),
            pObjectName: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub tagName: u64,
    pub tagSize: isize,
    pub pTag: *const c_void,
}
impl Default for VkDebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        VkDebugUtilsObjectTagInfoEXT {
            sType: extend_core_enums::VkStructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            pNext: ptr::null(),
            objectType: VkObjectType::UNKNOWN,
            objectHandle: Default::default(),
            tagName: Default::default(),
            tagSize: Default::default(),
            pTag: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [f32; 4],
}
impl Default for VkDebugUtilsLabelEXT {
    fn default() -> Self {
        VkDebugUtilsLabelEXT {
            sType: extend_core_enums::VkStructureType::DEBUG_UTILS_LABEL_EXT,
            pNext: ptr::null(),
            pLabelName: ptr::null(),
            color: [0f32; 4],
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
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
impl Default for VkDebugUtilsMessengerCallbackDataEXT {
    fn default() -> Self {
        VkDebugUtilsMessengerCallbackDataEXT {
            sType: extend_core_enums::VkStructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            pMessageIdName: ptr::null(),
            messageIdNumber: Default::default(),
            pMessage: ptr::null(),
            queueLabelCount: Default::default(),
            pQueueLabels: ptr::null(),
            cmdBufLabelCount: Default::default(),
            pCmdBufLabels: ptr::null(),
            objectCount: Default::default(),
            pObjects: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}
impl Default for VkDebugUtilsMessengerCreateInfoEXT {
    fn default() -> Self {
        extern "C" fn vkDebugUtilsMessengerCallbackEXT(
            _messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
            _messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
            _pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
            _pUserData: *mut c_void,
        ) -> VkBool32 {
            unimplemented!()
        }
        VkDebugUtilsMessengerCreateInfoEXT {
            sType: extend_core_enums::VkStructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            messageSeverity: Default::default(),
            messageType: Default::default(),
            pfnUserCallback: vkDebugUtilsMessengerCallbackEXT,
            pUserData: ptr::null_mut(),
        }
    }
}

instance_level_functions! {
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
