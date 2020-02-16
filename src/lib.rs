use std::fmt::{Display, Formatter, Error};
use std::ffi::CStr;

pub type VkSampleMask = u32;
pub type VkFlags = u32;
pub type VkDeviceSize = u64;
pub type VkDeviceAddress = u64;

pub type HINSTANCE = usize;
pub type HWND = usize;

macro_rules! handle {
    ($x:ident,$y:ty) => {
        #[repr(C)]
        #[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
        pub struct $x($y);
        impl Display for $x{
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                write!(f, "{}", self.0.to_string())
            }
        }
    }
}

macro_rules! enums{
    (
        $(
            enum $enum_name:ident{
                $($variant_name:ident = $value:literal),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub enum $enum_name{
                $($variant_name = $value),*
            }
            impl $enum_name{
                #[inline(always)]
                pub fn value(self,)->i32{
                    self as i32
                }
            }
        )*
    }
}

macro_rules! core_enums {
    (
        $(
            enum $enum_name:ident{
                $($variant_name:ident = $value:literal),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            #[repr(transparent)]
            #[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub struct $enum_name(i32);
            impl $enum_name{
                $(
                    pub const $variant_name: i32 = $value;
                )*
                #[inline(always)]
                pub fn value(&self,)->i32{
                    self.0
                }
            }
            impl From<i32> for $enum_name{
                #[inline(always)]
                fn from(n: i32)->Self{
                    $enum_name(n)
                }
            }
            impl PartialEq<i32> for $enum_name{
                #[inline(always)]
                fn eq(&self, other: &i32) -> bool {
                    self.value() == *other
                }
            }
        )*
    }
}

macro_rules! extend_enums {
    (
        $(
            enum $enum_name:ident{
                $($variant_name:ident = $value:literal),*$(,)?
            }
        ),*$(,)?
    )=>{
        pub mod extend_enums{
        $(
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub enum $enum_name{
                $($variant_name = $value),*
            }
            impl $enum_name{
                #[inline(always)]
                pub fn value(self,)->i32{
                    self as i32
                }
            }
            impl Into<crate::$enum_name> for $enum_name{
                #[inline(always)]
                fn into(self)->crate::$enum_name {
                    crate::$enum_name::from(self as i32)
                }
            }
            impl PartialEq<crate::$enum_name> for $enum_name{
                #[inline(always)]
                fn eq(&self, other: &crate::$enum_name) -> bool {
                    (*self as i32) == other.value()
                }
            }
//            impl PartialEq<$enum_name> for crate::$enum_name{
//                #[inline(always)]
//                fn eq(&self, other: &$enum_name) -> bool {
//                     self.value() == (*other as i32)
//                }
//            }
        )*
        }
    }
}

macro_rules! bitmasks {
    (
        $(
            {
                $flags_name:ident,
                enum $flag_bits_name:ident{
                    $($bit_name:ident = $value:literal),*$(,)?
                }$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            #[repr(transparent)]
            #[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub struct $flags_name(u32);
            impl $flags_name{
                #[inline(always)]
                pub fn value(&self,)->u32{
                    self.0
                }
            }
            impl From<u32> for $flags_name{
                #[inline(always)]
                fn from(n: u32)->Self{
                    $flags_name(n)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub enum $flag_bits_name{
                $(
                    $bit_name = $value
                ),*
            }
            impl $flag_bits_name{
                #[inline(always)]
                pub fn value(self,)->u32{
                    self as u32
                }
            }
            impl Into<$flags_name> for $flag_bits_name{
                #[inline(always)]
                fn into(self)->$flags_name {
                    $flags_name::from(self.value())
                }
            }
        )*
    }
}

macro_rules! _extend_bitmasks {
    (
        $(
            {
                $flags_name:ident,
                enum $flag_bits_name:ident{
                    $($bit_name:ident = $value:literal),*$(,)?
                }$(,)?
            }
        ),*$(,)?
    )=>{
        pub mod extend_bitmasks{
        $(
            #[repr(C)]
            #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
            pub enum $flag_bits_name{
                $($bit_name = $value),*
            }
            impl $flag_bits_name{
                #[inline(always)]
                pub fn value(self,)->u32{
                    self as u32
                }
            }
            impl Into<crate::$flags_name> for $flag_bits_name{
                #[inline(always)]
                fn into(self)->crate::$flags_name {
                    crate::$flags_name::from(self.value())
                }
            }
        )*
        }
    }
}

macro_rules! extension_functions {
    ( $(fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;)* )=>{
        pub struct Functions {
            $(
                $function_name: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)?,
            )*
        }
        impl Functions {
            pub fn load_from_instance(instance: crate::core::VkInstance)-> Functions {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_instance_proc_addr;
                unsafe {
                    Functions {
                        $(
                            $function_name: transmute(
                                get_instance_proc_addr(
                                    instance,
                                    CStr::from_bytes_with_nul(concat!(stringify!($function_name), '\0').as_bytes())
                                        .unwrap()
                                ).expect(concat!("Load \"", stringify!($function_name), "\" failed!"))
                            ),
                        )*
                    }
                }
            }
            pub fn load_from_device(device: crate::core::VkDevice)-> Functions {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_device_proc_addr;
                unsafe {
                    Functions {
                        $(
                            $function_name: transmute(
                                get_device_proc_addr(
                                    device,
                                    CStr::from_bytes_with_nul(concat!(stringify!($function_name), '\0').as_bytes())
                                        .unwrap()
                                ).expect(concat!("Load \"", stringify!($function_name), "\" failed!"))
                            ),
                        )*
                    }
                }
            }
            $(
                #[inline(always)]
                pub unsafe fn $function_name(&self, $($parameter_name:$parameter_type),*)$(->$return_type)?{
                (self.$function_name)($($parameter_name),*)
                }
            )*
        }
    }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct DispatchableHandle(usize);
impl Display for DispatchableHandle{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#x}", self.0)
    }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct NonDispatchableHandle(u64);
impl Display for NonDispatchableHandle{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000f32;
pub const VK_REMAINING_MIP_LEVELS: u32 = 0xFFFF_FFFF;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 0xFFFF_FFFF;
pub const VK_WHOLE_SIZE: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const VK_ATTACHMENT_UNUSED: u32 = 0xFFFF_FFFF;
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VkBool32{
    TRUE = 1,
    FALSE = 0,
}
impl Default for VkBool32{
    fn default() -> Self {
        VkBool32::FALSE
    }
}
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xFFFF_FFFF;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = VK_QUEUE_FAMILY_IGNORED-1;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xFFFF_FFFF;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;

fn get_instance_proc_addr(instance: VkInstance, name: &CStr)->Option<PFN_vkVoidFunction>{
    let function_pointer = unsafe {vkGetInstanceProcAddr(instance, name.as_ptr())};
    if function_pointer as usize == 0 {
        None
    }else {
        Some(function_pointer)
    }
}

fn get_device_proc_addr(device: VkDevice, name: &CStr)->Option<PFN_vkVoidFunction>{
    let function_pointer = unsafe {vkGetDeviceProcAddr(device, name.as_ptr())};
    if function_pointer as usize == 0 {
        None
    }else {
        Some(function_pointer)
    }
}



mod core;
pub use crate::core::*;

pub mod khr;
pub mod ext;
