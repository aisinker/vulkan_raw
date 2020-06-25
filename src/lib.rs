use std::ffi::CStr;
use std::fmt::{Debug, Display, Error, Formatter};

#[macro_use]
extern crate lazy_static;

pub type VkSampleMask = u32;
pub type VkFlags = u32;
pub type VkDeviceSize = u64;
pub type VkDeviceAddress = u64;

#[cfg(target_family = "windows")]
pub type HANDLE = usize;
#[cfg(target_family = "windows")]
pub type HINSTANCE = usize;
#[cfg(target_family = "windows")]
pub type HWND = usize;
#[cfg(target_family = "windows")]
pub type LPCWSTR = *const u16;
#[cfg(target_family = "windows")]
pub type DWORD = u32;
#[cfg(target_family = "windows")]
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = c_void; // TODO

macro_rules! handle {
    ($x:ident,$y:ty) => {
        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct $x($y);
        impl $x {
            pub fn none() -> Self {
                $x(<$y>::none())
            }
        }
        impl Default for $x {
            fn default() -> Self {
                $x::none()
            }
        }
        impl Display for $x {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                write!(f, "{}", self.0.to_string())
            }
        }
        impl From<$x> for u64 {
            fn from(handle: $x) -> Self {
                handle.0.into()
            }
        }
    };
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
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            pub struct $enum_name(pub i32);
            impl $enum_name{
                $(
                    pub const $variant_name: $enum_name = $enum_name($value);
                )*
            }
        )*
    }
}

macro_rules! extension_enums{
    (
        $(
            enum $enum_name:ident{
                $($variant_name:ident = $value:literal),*$(,)?
            }
        ),*$(,)?
    )=>{
        $(
            #[repr(transparent)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            pub struct $enum_name(pub i32);
            impl $enum_name{
                $(
                    pub const $variant_name: $enum_name = $enum_name($value);
                )*
            }
        )*
    }
}

macro_rules! extend_core_enums {
    (
        $(
            enum $enum_name:ident{
                $($variant_name:ident = $value:literal),*$(,)?
            }
        ),*$(,)?
    )=>{
        pub mod extend_core_enums{
        $(
            pub enum $enum_name{
            }
            impl $enum_name{
                $(
                    pub const $variant_name: crate::$enum_name = crate::$enum_name($value);
                )*
            }
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
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
            pub struct $flags_name(pub u32);
            pub type $flag_bits_name = $flags_name;
            impl $flag_bits_name {
                $(
                    pub const $bit_name: $flag_bits_name = $flags_name($value);
                )*
            }

            impl std::ops::Not for $flags_name {
                type Output = $flags_name;
                #[inline(always)]
                fn not(self) -> Self::Output {
                    $flags_name(!self.0)
                }
            }

            impl std::ops::BitAnd for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitand(self, rhs: Self) -> Self::Output {
                    $flags_name(self.0&rhs.0)
                }
            }
            impl std::ops::BitAnd<u32> for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitand(self, rhs: u32) -> Self::Output {
                    $flags_name(self.0&rhs)
                }
            }
            impl std::ops::BitAnd<$flags_name> for u32 {
                type Output = $flags_name;
                #[inline(always)]
                fn bitand(self, rhs: $flags_name) -> Self::Output {
                    $flags_name(self&rhs.0)
                }
            }

            impl std::ops::BitAndAssign for $flags_name {
                #[inline(always)]
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0&=rhs.0;
                }
            }
            impl std::ops::BitAndAssign<u32> for $flags_name {
                #[inline(always)]
                fn bitand_assign(&mut self, rhs: u32) {
                    self.0&=rhs;
                }
            }
            impl std::ops::BitAndAssign<$flags_name> for u32 {
                #[inline(always)]
                fn bitand_assign(&mut self, rhs: $flags_name) {
                    *self&=rhs.0;
                }
            }

            impl std::ops::BitOr for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitor(self, rhs: Self) -> Self::Output {
                    $flags_name(self.0|rhs.0)
                }
            }
            impl std::ops::BitOr<u32> for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitor(self, rhs: u32) -> Self::Output {
                    $flags_name(self.0|rhs)
                }
            }
            impl std::ops::BitOr<$flags_name> for u32 {
                type Output = $flags_name;
                #[inline(always)]
                fn bitor(self, rhs: $flags_name) -> Self::Output {
                    $flags_name(self|rhs.0)
                }
            }

            impl std::ops::BitOrAssign for $flags_name {
                #[inline(always)]
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0|=rhs.0;
                }
            }
            impl std::ops::BitOrAssign<u32> for $flags_name {
                #[inline(always)]
                fn bitor_assign(&mut self, rhs: u32) {
                    self.0|=rhs;
                }
            }
            impl std::ops::BitOrAssign<$flags_name> for u32 {
                #[inline(always)]
                fn bitor_assign(&mut self, rhs: $flags_name) {
                    *self|=rhs.0;
                }
            }

            impl std::ops::BitXor for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitxor(self, rhs: Self) -> Self::Output {
                    $flags_name(self.0^rhs.0)
                }
            }
            impl std::ops::BitXor<u32> for $flags_name {
                type Output = Self;
                #[inline(always)]
                fn bitxor(self, rhs: u32) -> Self::Output {
                    $flags_name(self.0^rhs)
                }
            }
            impl std::ops::BitXor<$flags_name> for u32 {
                type Output = $flags_name;
                #[inline(always)]
                fn bitxor(self, rhs: $flags_name) -> Self::Output {
                    $flags_name(self^rhs.0)
                }
            }

            impl std::ops::BitXorAssign for $flags_name {
                #[inline(always)]
                fn bitxor_assign(&mut self, rhs: Self) {
                    self.0^=rhs.0;
                }
            }
            impl std::ops::BitXorAssign<u32> for $flags_name {
                #[inline(always)]
                fn bitxor_assign(&mut self, rhs: u32) {
                    self.0^=rhs;
                }
            }
            impl std::ops::BitXorAssign<$flags_name> for u32 {
                #[inline(always)]
                fn bitxor_assign(&mut self, rhs: $flags_name) {
                    *self^=rhs.0;
                }
            }

            impl PartialEq<u32> for $flags_name {
                fn eq(&self, other: &u32) -> bool {
                    self.0 == *other
                }
            }
            impl PartialEq<$flags_name> for u32{
                fn eq(&self, other: &$flags_name) -> bool {
                    *self == other.0
                }
            }
        )*
    };
}

macro_rules! _extend_core_bitmasks {
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
        pub mod extend_core_bitmasks{
        $(
            pub struct $flag_bits_name {
            }
            impl $flag_bits_name {
                $(
                    pub const $bit_name: crate::$flag_bits_name = crate::$flags_name($value);
                )*
            }
        )*
        }
    }
}

macro_rules! instance_level_functions {
    ( $(fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;)* )=>{
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct InstanceLevelFunctions {
            $(
                $function_name: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)?,
            )*
        }
        impl InstanceLevelFunctions {
            pub fn load_from_instance(instance: crate::core::VkInstance)->InstanceLevelFunctions {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_instance_proc_addr;
                unsafe {
                    let functions = InstanceLevelFunctions {
                        $(
                            $function_name: match get_instance_proc_addr(
                                instance,
                                CStr::from_bytes_with_nul_unchecked(concat!(stringify!($function_name), '\0').as_bytes()))
                            {
                                Ok(proc_addr) => transmute(proc_addr),
                                Err(_) => {
                                    extern "C" fn $function_name($(_:$parameter_type),*)$(->$return_type)?{
                                        unimplemented!()
                                    }
                                    $function_name
                                },
                            },
                        )*
                    };
                    functions
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

macro_rules! device_level_functions {
    ( $(fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;)* )=>{
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct DeviceLevelFunctions {
            $(
                $function_name: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)?,
            )*
        }
        impl DeviceLevelFunctions {
            pub fn load_from_instance(instance: crate::core::VkInstance)->DeviceLevelFunctions {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_instance_proc_addr;
                unsafe {
                    let functions = DeviceLevelFunctions {
                        $(
                            $function_name: match get_instance_proc_addr(
                                instance,
                                CStr::from_bytes_with_nul_unchecked(concat!(stringify!($function_name), '\0').as_bytes()))
                            {
                                Ok(proc_addr) => transmute(proc_addr),
                                Err(_) => {
                                    extern "C" fn $function_name($(_:$parameter_type),*)$(->$return_type)?{
                                        unimplemented!()
                                    }
                                    $function_name
                                },
                            },
                        )*
                    };
                    functions
                }
            }
            pub fn load_from_device(core_functions: &crate::core::InstanceLevelFunctions, device: crate::core::VkDevice)->DeviceLevelFunctions {
                use std::ffi::CStr;
                use std::mem::transmute;
                use crate::get_device_proc_addr;
                unsafe {
                    let functions = DeviceLevelFunctions {
                        $(
                            $function_name: match get_device_proc_addr(
                                core_functions,
                                device,
                                CStr::from_bytes_with_nul_unchecked(concat!(stringify!($function_name), '\0').as_bytes()))
                            {
                                Ok(proc_addr) => transmute(proc_addr),
                                Err(_) => {
                                    extern "C" fn $function_name($(_:$parameter_type),*)$(->$return_type)?{
                                        unimplemented!()
                                    }
                                    $function_name
                                },
                            },
                        )*
                    };
                    functions
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct DispatchableHandle(usize);
impl DispatchableHandle {
    pub fn none() -> Self {
        DispatchableHandle(0)
    }
}
impl Display for DispatchableHandle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#x}", self.0)
    }
}
impl From<DispatchableHandle> for u64 {
    fn from(handle: DispatchableHandle) -> Self {
        handle.0 as u64
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct NonDispatchableHandle(u64);
impl NonDispatchableHandle {
    pub fn none() -> Self {
        NonDispatchableHandle(0)
    }
}
impl Display for NonDispatchableHandle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
impl From<NonDispatchableHandle> for u64 {
    fn from(handle: NonDispatchableHandle) -> Self {
        handle.0
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VkBool32(u32);
impl VkBool32 {
    pub const TRUE: VkBool32 = VkBool32(1);
    pub const FALSE: VkBool32 = VkBool32(0);
}
impl Default for VkBool32 {
    #[inline(always)]
    fn default() -> Self {
        VkBool32::FALSE
    }
}
impl From<VkBool32> for bool {
    #[inline(always)]
    fn from(bool: VkBool32) -> Self {
        bool == VkBool32::TRUE
    }
}
impl From<bool> for VkBool32 {
    #[inline(always)]
    fn from(bool: bool) -> Self {
        VkBool32(bool as u32)
    }
}
impl Debug for VkBool32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            if *self == VkBool32::TRUE {
                "VkBool32::TRUE"
            } else {
                "VkBool32::FALSE"
            }
        )
    }
}
impl Display for VkBool32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", *self == VkBool32::TRUE)
    }
}

pub const VK_QUEUE_FAMILY_IGNORED: u32 = 0xFFFF_FFFF;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = VK_QUEUE_FAMILY_IGNORED - 1;
pub const VK_SUBPASS_EXTERNAL: u32 = 0xFFFF_FFFF;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;

#[derive(Debug)]
pub struct LoadingError(String);

fn get_instance_proc_addr(
    instance: VkInstance,
    name: &CStr,
) -> Result<PFN_vkVoidFunction, LoadingError> {
    let function_pointer = unsafe { vkGetInstanceProcAddr(instance, name.as_ptr()) };
    match function_pointer as usize {
        0 => Err(LoadingError(format!(
            "Load function \"{}\"  failed!",
            name.to_str().unwrap()
        ))),
        _ => Ok(function_pointer),
    }
}
fn get_device_proc_addr(
    core_functions: &InstanceLevelFunctions,
    device: VkDevice,
    name: &CStr,
) -> Result<PFN_vkVoidFunction, LoadingError> {
    let function_pointer = unsafe { core_functions.vkGetDeviceProcAddr(device, name.as_ptr()) };
    match function_pointer as usize {
        0 => Err(LoadingError(format!(
            "Load function '{}'  failed!",
            name.to_str().unwrap()
        ))),
        _ => Ok(function_pointer),
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ApiVersion(u32);

impl ApiVersion {
    #[inline(always)]
    pub const fn new(major: u32, minor: u32, patch: u32) -> ApiVersion {
        ApiVersion((major << 22) | ((minor & 0x0000_03FF) << 12) | (patch & 0x0000_0FFF))
    }

    #[inline(always)]
    pub fn major(&self) -> u32 {
        (self.0 & 0xFFC0_0000) >> 22
    }

    #[inline(always)]
    pub fn minor(&self) -> u32 {
        (self.0 & 0x003F_F000) >> 12
    }

    #[inline(always)]
    pub fn patch(&self) -> u32 {
        self.0 & 0x0000_0FFF
    }
}

impl From<u32> for ApiVersion {
    #[inline(always)]
    fn from(api_version: u32) -> Self {
        ApiVersion(api_version)
    }
}

impl Into<u32> for ApiVersion {
    #[inline(always)]
    fn into(self) -> u32 {
        self.0
    }
}

impl Display for ApiVersion {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

mod core;
pub use crate::core::*;
use std::os::raw::c_void;

pub mod ext;
pub mod khr;
