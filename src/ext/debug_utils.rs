#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt::{Display, Error, Formatter};
use std::os::raw::c_char;
use std::ptr;

use crate::*;

// pub const SPEC_VERSION: u32 = 2;
// pub const EXTENSION_NAME: *const c_char = b"VK_EXT_debug_utils\0".as_ptr() as *const c_char;

handle!(
    VkDebugUtilsMessengerEXT,
    NonDispatchableHandle,
    VkObjectType::DEBUG_UTILS_MESSENGER_EXT
);

bitmasks! {
    VkDebugUtilsMessengerCreateFlagsEXT = enum VkDebugUtilsMessengerCreateFlagBitsEXT{
        _RESERVED = 0,
    },
    VkDebugUtilsMessengerCallbackDataFlagsEXT = enum VkDebugUtilsMessengerCallbackDataFlagBitsEXT{
        _RESERVED = 0,
    },
    VkDebugUtilsMessageSeverityFlagsEXT = enum VkDebugUtilsMessageSeverityFlagBitsEXT{
        VERBOSE_BIT_EXT = 0x00000001,
        INFO_BIT_EXT = 0x00000010,
        WARNING_BIT_EXT = 0x00000100,
        ERROR_BIT_EXT = 0x00001000,
    },
    VkDebugUtilsMessageTypeFlagsEXT = enum VkDebugUtilsMessageTypeFlagBitsEXT{
        GENERAL_BIT_EXT = 0x00000001,
        VALIDATION_BIT_EXT = 0x00000002,
        PERFORMANCE_BIT_EXT = 0x00000004,
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
            sType: VkStructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
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
    pub tagSize: usize,
    pub pTag: *const c_void,
}
impl Default for VkDebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        VkDebugUtilsObjectTagInfoEXT {
            sType: VkStructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
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
            sType: VkStructureType::DEBUG_UTILS_LABEL_EXT,
            pNext: ptr::null(),
            pLabelName: "\0".as_ptr() as *const c_char,
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
            sType: VkStructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            pMessageIdName: ptr::null(),
            messageIdNumber: Default::default(),
            pMessage: "\0".as_ptr() as *const c_char,
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
            sType: VkStructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: Default::default(),
            messageSeverity: Default::default(),
            messageType: Default::default(),
            pfnUserCallback: vkDebugUtilsMessengerCallbackEXT,
            pUserData: ptr::null_mut(),
        }
    }
}
