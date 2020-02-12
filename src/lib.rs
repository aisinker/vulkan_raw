use std::fmt::{Display, Formatter, Error};
use std::ffi::CStr;

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

macro_rules! reserved_flags {
    ($($x:ident),*) => {
        $(
            bitflags! {
                #[repr(C)]
	            #[derive(Default)]
                pub struct $x: u32 {
                    const NONE = 0x00000000;
                }
            }
        )*
    };
}

macro_rules! extern_c_functions {
    ( $(fn $function_name:ident($($parameter_name:ident:$parameter_type:ty),*)$(->$return_type:ty)?;)* )=>{
        pub struct Functions {
            $(
                $function_name: extern "C" fn($($parameter_name:$parameter_type),*)$(->$return_type)?,
            )*
        }
        impl Functions {
            pub fn load_from_instance(instance: VkInstance)-> Functions {
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
            pub fn load_from_device(device: VkDevice)-> Functions {
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

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct DispatchableHandle(usize);
impl Display for DispatchableHandle{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#x}", self.0)
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct NonDispatchableHandle(u64);
impl Display for NonDispatchableHandle{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

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

pub mod khr;
pub mod ext;

pub use self::core::*;