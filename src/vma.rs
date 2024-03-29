#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::*;
use std::ffi::c_void;
use std::fmt::{Debug, Display, Error, Formatter};
use std::os::raw::c_char;
use std::ptr;

handle!(VmaAllocator, DispatchableHandle);
handle!(VmaPool, DispatchableHandle);
handle!(VmaAllocation, DispatchableHandle);
handle!(VmaDefragmentationContext, DispatchableHandle);
handle!(VmaVirtualAllocation, NonDispatchableHandle);
handle!(VmaVirtualBlock, DispatchableHandle);

bitmasks! {
    VmaAllocationCreateFlags = enum VmaAllocationCreateFlagBits{
        DEDICATED_MEMORY_BIT = 0x00000001,
        NEVER_ALLOCATE_BIT = 0x00000002,
        MAPPED_BIT = 0x00000004,
        USER_DATA_COPY_STRING_BIT = 0x00000020,
        UPPER_ADDRESS_BIT = 0x00000040,
        DONT_BIND_BIT = 0x00000080,
        WITHIN_BUDGET_BIT = 0x00000100,
        CAN_ALIAS_BIT = 0x00000200,
        HOST_ACCESS_SEQUENTIAL_WRITE_BIT = 0x00000400,
        HOST_ACCESS_RANDOM_BIT = 0x00000800,
        HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT = 0x00001000,
        STRATEGY_MIN_MEMORY_BIT = 0x00010000,
        STRATEGY_MIN_TIME_BIT = 0x00020000,
        STRATEGY_MIN_OFFSET_BIT  = 0x00040000,
    },
    VmaAllocatorCreateFlags = enum VmaAllocatorCreateFlagBits{
        EXTERNALLY_SYNCHRONIZED_BIT = 0x00000001,
        KHR_DEDICATED_ALLOCATION_BIT = 0x00000002,
        KHR_BIND_MEMORY2_BIT = 0x00000004,
        EXT_MEMORY_BUDGET_BIT = 0x00000008,
        AMD_DEVICE_COHERENT_MEMORY_BIT = 0x00000010,
        BUFFER_DEVICE_ADDRESS_BIT = 0x00000020,
        EXT_MEMORY_PRIORITY_BIT = 0x00000040,
    },
    VmaPoolCreateFlags = enum VmaPoolCreateFlagBits{
        IGNORE_BUFFER_IMAGE_GRANULARITY_BIT = 0x00000002,
        LINEAR_ALGORITHM_BIT = 0x00000004,
    },
    VmaDefragmentationFlags = enum VmaDefragmentationFlagBits{
        ALGORITHM_FAST_BIT = 0x1,
        ALGORITHM_BALANCED_BIT = 0x2,
        ALGORITHM_FULL_BIT = 0x4,
        ALGORITHM_EXTENSIVE_BIT = 0x8,
    },
    VmaVirtualAllocationCreateFlags = enum VmaVirtualAllocationCreateFlagBits{
        UPPER_ADDRESS_BIT = 0x00000040,
        STRATEGY_MIN_MEMORY_BIT = 0x00010000,
        STRATEGY_MIN_TIME_BIT = 0x00020000,
        STRATEGY_MIN_OFFSET_BIT = 0x00040000,
    },
    VmaVirtualBlockCreateFlags = enum VmaVirtualBlockCreateFlagBits{
        LINEAR_ALGORITHM_BIT = 0x00000001,
    }
}

enums! {
    enum VmaMemoryUsage{
        UNKNOWN = 0,
        GPU_LAZILY_ALLOCATED = 6,
        AUTO = 7,
        AUTO_PREFER_DEVICE = 8,
        AUTO_PREFER_HOST = 9,
    },
    enum VmaDefragmentationMoveOperation{
        COPY = 0,
        IGNORE = 1,
        DESTROY = 2,
    },
}

pub type PFN_vmaAllocateDeviceMemoryFunction = extern "C" fn(
    allocator: VmaAllocator,
    memoryType: u32,
    memory: VkDeviceMemory,
    size: VkDeviceSize,
    pUserData: *mut c_void,
);
pub type PFN_vmaFreeDeviceMemoryFunction = extern "C" fn(
    allocator: VmaAllocator,
    memoryType: u32,
    memory: VkDeviceMemory,
    size: VkDeviceSize,
    pUserData: *mut c_void,
);
pub type PFN_vkGetInstanceProcAddr =
    extern "C" fn(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetDeviceProcAddr =
    extern "C" fn(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetPhysicalDeviceProperties =
    extern "C" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
);
pub type PFN_vkAllocateMemory = extern "C" fn(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult;
pub type PFN_vkFreeMemory = extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
);
pub type PFN_vkMapMemory = extern "C" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut *mut c_void,
) -> VkResult;
pub type PFN_vkUnmapMemory = extern "C" fn(device: VkDevice, memory: VkDeviceMemory);
pub type PFN_vkFlushMappedMemoryRanges = extern "C" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
pub type PFN_vkInvalidateMappedMemoryRanges = extern "C" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
pub type PFN_vkBindBufferMemory = extern "C" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
pub type PFN_vkBindImageMemory = extern "C" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
pub type PFN_vkGetBufferMemoryRequirements = extern "C" fn(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *const VkMemoryRequirements,
);
pub type PFN_vkGetImageMemoryRequirements =
    extern "C" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
pub type PFN_vkCreateBuffer = extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult;
pub type PFN_vkDestroyBuffer =
    extern "C" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateImage = extern "C" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult;
pub type PFN_vkDestroyImage =
    extern "C" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCmdCopyBuffer = extern "C" fn(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferCopy,
);
pub type PFN_vkGetBufferMemoryRequirements2KHR = extern "C" fn(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkGetImageMemoryRequirements2KHR = extern "C" fn(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
pub type PFN_vkBindBufferMemory2KHR = extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult;
pub type PFN_vkBindImageMemory2KHR = extern "C" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult;
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = extern "C" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
);

#[cfg(feature = "VK_VERSION_1_3")]
pub type PFN_vkGetDeviceBufferMemoryRequirements = extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);

#[cfg(feature = "VK_VERSION_1_3")]
pub type PFN_vkGetDeviceImageMemoryRequirements = extern "C" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaDeviceMemoryCallbacks {
    pub pfnAllocate: PFN_vmaAllocateDeviceMemoryFunction,
    pub pfnFree: PFN_vmaFreeDeviceMemoryFunction,
    pub pUserData: *mut c_void,
}
impl Default for VmaDeviceMemoryCallbacks {
    fn default() -> Self {
        extern "C" fn allocate(
            _allocator: VmaAllocator,
            _memoryType: u32,
            _memory: VkDeviceMemory,
            _size: VkDeviceSize,
            _pUserData: *mut c_void,
        ) {
            unimplemented!()
        }
        extern "C" fn free(
            _allocator: VmaAllocator,
            _memoryType: u32,
            _memory: VkDeviceMemory,
            _size: VkDeviceSize,
            _pUserData: *mut c_void,
        ) {
            unimplemented!()
        }
        Self {
            pfnAllocate: allocate,
            pfnFree: free,
            pUserData: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaVulkanFunctions {
    pub vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr,
    pub vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    pub vkGetPhysicalDeviceProperties: PFN_vkGetPhysicalDeviceProperties,
    pub vkGetPhysicalDeviceMemoryProperties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pub vkAllocateMemory: PFN_vkAllocateMemory,
    pub vkFreeMemory: PFN_vkFreeMemory,
    pub vkMapMemory: PFN_vkMapMemory,
    pub vkUnmapMemory: PFN_vkUnmapMemory,
    pub vkFlushMappedMemoryRanges: PFN_vkFlushMappedMemoryRanges,
    pub vkInvalidateMappedMemoryRanges: PFN_vkInvalidateMappedMemoryRanges,
    pub vkBindBufferMemory: PFN_vkBindBufferMemory,
    pub vkBindImageMemory: PFN_vkBindImageMemory,
    pub vkGetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements,
    pub vkGetImageMemoryRequirements: PFN_vkGetImageMemoryRequirements,
    pub vkCreateBuffer: PFN_vkCreateBuffer,
    pub vkDestroyBuffer: PFN_vkDestroyBuffer,
    pub vkCreateImage: PFN_vkCreateImage,
    pub vkDestroyImage: PFN_vkDestroyImage,
    pub vkCmdCopyBuffer: PFN_vkCmdCopyBuffer,
    // vulkan 1.1
    pub vkGetBufferMemoryRequirements2KHR: PFN_vkGetBufferMemoryRequirements2KHR,
    pub vkGetImageMemoryRequirements2KHR: PFN_vkGetImageMemoryRequirements2KHR,
    // vulkan 1.1
    pub vkBindBufferMemory2KHR: PFN_vkBindBufferMemory2KHR,
    pub vkBindImageMemory2KHR: PFN_vkBindImageMemory2KHR,
    // vulkan 1.1
    pub vkGetPhysicalDeviceMemoryProperties2KHR: PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
    // vulkan 1.3
    #[cfg(feature = "VK_VERSION_1_3")]
    pub vkGetDeviceBufferMemoryRequirements: PFN_vkGetDeviceBufferMemoryRequirements,
    #[cfg(feature = "VK_VERSION_1_3")]
    pub vkGetDeviceImageMemoryRequirements: PFN_vkGetDeviceImageMemoryRequirements,
}
impl Default for VmaVulkanFunctions {
    fn default() -> Self {
        extern "C" fn vkGetInstanceProcAddr(
            _instance: VkInstance,
            _pName: *const c_char,
        ) -> PFN_vkVoidFunction {
            unimplemented!()
        }
        extern "C" fn vkGetDeviceProcAddr(
            _device: VkDevice,
            _pName: *const c_char,
        ) -> PFN_vkVoidFunction {
            unimplemented!()
        }
        extern "C" fn vkGetPhysicalDeviceProperties(
            _physicalDevice: VkPhysicalDevice,
            _pProperties: *mut VkPhysicalDeviceProperties,
        ) {
            unimplemented!()
        }
        extern "C" fn vkGetPhysicalDeviceMemoryProperties(
            _physicalDevice: VkPhysicalDevice,
            _pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
        ) {
            unimplemented!()
        }
        extern "C" fn vkAllocateMemory(
            _device: VkDevice,
            _pAllocateInfo: *const VkMemoryAllocateInfo,
            _pAllocator: *const VkAllocationCallbacks,
            _pMemory: *mut VkDeviceMemory,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkFreeMemory(
            _device: VkDevice,
            _memory: VkDeviceMemory,
            _pAllocator: *const VkAllocationCallbacks,
        ) {
            unimplemented!()
        }
        extern "C" fn vkMapMemory(
            _device: VkDevice,
            _memory: VkDeviceMemory,
            _offset: VkDeviceSize,
            _size: VkDeviceSize,
            _flags: VkMemoryMapFlags,
            _ppData: *mut *mut c_void,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkUnmapMemory(_device: VkDevice, _memory: VkDeviceMemory) {
            unimplemented!()
        }
        extern "C" fn vkFlushMappedMemoryRanges(
            _device: VkDevice,
            _memoryRangeCount: u32,
            _pMemoryRanges: *const VkMappedMemoryRange,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkInvalidateMappedMemoryRanges(
            _device: VkDevice,
            _memoryRangeCount: u32,
            _pMemoryRanges: *const VkMappedMemoryRange,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkBindBufferMemory(
            _device: VkDevice,
            _buffer: VkBuffer,
            _memory: VkDeviceMemory,
            _memoryOffset: VkDeviceSize,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkBindImageMemory(
            _device: VkDevice,
            _image: VkImage,
            _memory: VkDeviceMemory,
            _memoryOffset: VkDeviceSize,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkGetBufferMemoryRequirements(
            _device: VkDevice,
            _buffer: VkBuffer,
            _pMemoryRequirements: *const VkMemoryRequirements,
        ) {
            unimplemented!()
        }
        extern "C" fn vkGetImageMemoryRequirements(
            _device: VkDevice,
            _image: VkImage,
            _pMemoryRequirements: *mut VkMemoryRequirements,
        ) {
            unimplemented!()
        }
        extern "C" fn vkCreateBuffer(
            _device: VkDevice,
            _pCreateInfo: *const VkBufferCreateInfo,
            _pAllocator: *const VkAllocationCallbacks,
            _pBuffer: *mut VkBuffer,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkDestroyBuffer(
            _device: VkDevice,
            _buffer: VkBuffer,
            _pAllocator: *const VkAllocationCallbacks,
        ) {
            unimplemented!()
        }
        extern "C" fn vkCreateImage(
            _device: VkDevice,
            _pCreateInfo: *const VkImageCreateInfo,
            _pAllocator: *const VkAllocationCallbacks,
            _pImage: *mut VkImage,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkDestroyImage(
            _device: VkDevice,
            _image: VkImage,
            _pAllocator: *const VkAllocationCallbacks,
        ) {
            unimplemented!()
        }
        extern "C" fn vkCmdCopyBuffer(
            _commandBuffer: VkCommandBuffer,
            _srcBuffer: VkBuffer,
            _dstBuffer: VkBuffer,
            _regionCount: u32,
            _pRegions: *const VkBufferCopy,
        ) {
            unimplemented!()
        }
        extern "C" fn vkGetBufferMemoryRequirements2(
            _device: VkDevice,
            _pInfo: *const VkBufferMemoryRequirementsInfo2,
            _pMemoryRequirements: *mut VkMemoryRequirements2,
        ) {
            unimplemented!()
        }
        extern "C" fn vkGetImageMemoryRequirements2(
            _device: VkDevice,
            _pInfo: *const VkImageMemoryRequirementsInfo2,
            _pMemoryRequirements: *mut VkMemoryRequirements2,
        ) {
            unimplemented!()
        }
        extern "C" fn vkBindBufferMemory2(
            _device: VkDevice,
            _bindInfoCount: u32,
            _pBindInfos: *const VkBindBufferMemoryInfo,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkBindImageMemory2(
            _device: VkDevice,
            _bindInfoCount: u32,
            _pBindInfos: *const VkBindImageMemoryInfo,
        ) -> VkResult {
            unimplemented!()
        }
        extern "C" fn vkGetPhysicalDeviceMemoryProperties2(
            _physicalDevice: VkPhysicalDevice,
            _pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
        ) {
            unimplemented!()
        }
        #[cfg(feature = "VK_VERSION_1_3")]
        extern "C" fn vkGetDeviceBufferMemoryRequirements(
            _device: VkDevice,
            _pInfo: *const VkDeviceBufferMemoryRequirements,
            _pMemoryRequirements: *mut VkMemoryRequirements2,
        ) {
            unimplemented!()
        }
        #[cfg(feature = "VK_VERSION_1_3")]
        extern "C" fn vkGetDeviceImageMemoryRequirements(
            _device: VkDevice,
            _pInfo: *const VkDeviceImageMemoryRequirements,
            _pMemoryRequirements: *mut VkMemoryRequirements2,
        ) {
            unimplemented!()
        }
        Self {
            vkGetInstanceProcAddr,
            vkGetDeviceProcAddr,
            vkGetPhysicalDeviceProperties,
            vkGetPhysicalDeviceMemoryProperties,
            vkAllocateMemory,
            vkFreeMemory,
            vkMapMemory,
            vkUnmapMemory,
            vkFlushMappedMemoryRanges,
            vkInvalidateMappedMemoryRanges,
            vkBindBufferMemory,
            vkBindImageMemory,
            vkGetBufferMemoryRequirements,
            vkGetImageMemoryRequirements,
            vkCreateBuffer,
            vkDestroyBuffer,
            vkCreateImage,
            vkDestroyImage,
            vkCmdCopyBuffer,
            vkGetBufferMemoryRequirements2KHR: vkGetBufferMemoryRequirements2,
            vkGetImageMemoryRequirements2KHR: vkGetImageMemoryRequirements2,
            vkBindBufferMemory2KHR: vkBindBufferMemory2,
            vkBindImageMemory2KHR: vkBindImageMemory2,
            vkGetPhysicalDeviceMemoryProperties2KHR: vkGetPhysicalDeviceMemoryProperties2,
            #[cfg(feature = "VK_VERSION_1_3")]
            vkGetDeviceBufferMemoryRequirements,
            #[cfg(feature = "VK_VERSION_1_3")]
            vkGetDeviceImageMemoryRequirements,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaAllocatorCreateInfo {
    pub flags: VmaAllocatorCreateFlags,
    pub physicalDevice: VkPhysicalDevice,
    pub device: VkDevice,
    pub preferredLargeHeapBlockSize: VkDeviceSize,
    pub pAllocationCallbacks: *const VkAllocationCallbacks,
    pub pDeviceMemoryCallbacks: *const VmaDeviceMemoryCallbacks,
    pub pHeapSizeLimit: *const VkDeviceSize,
    pub pVulkanFunctions: *const VmaVulkanFunctions,
    pub instance: VkInstance,
    pub vulkanApiVersion: u32,
    pub pTypeExternalMemoryHandleTypes: *const VkExternalMemoryHandleTypeFlags,
}
impl Default for VmaAllocatorCreateInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            physicalDevice: Default::default(),
            device: Default::default(),
            preferredLargeHeapBlockSize: Default::default(),
            pAllocationCallbacks: ptr::null(),
            pDeviceMemoryCallbacks: ptr::null(),
            pHeapSizeLimit: ptr::null(),
            pVulkanFunctions: ptr::null(),
            instance: Default::default(),
            vulkanApiVersion: 0,
            pTypeExternalMemoryHandleTypes: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaAllocatorInfo {
    pub instance: VkInstance,
    pub physicalDevice: VkPhysicalDevice,
    pub device: VkDevice,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaStatistics {
    pub blockCount: u32,
    pub allocationCount: u32,
    pub blockBytes: VkDeviceSize,
    pub allocationBytes: VkDeviceSize,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaDetailedStatistics {
    pub statistics: VmaStatistics,
    pub unusedRangeCount: u32,
    pub allocationSizeMin: VkDeviceSize,
    pub allocationSizeMax: VkDeviceSize,
    pub unusedRangeSizeMin: VkDeviceSize,
    pub unusedRangeSizeMax: VkDeviceSize,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaTotalStatistics {
    pub memoryType: [VmaDetailedStatistics; VK_MAX_MEMORY_TYPES],
    pub memoryHeap: [VmaDetailedStatistics; VK_MAX_MEMORY_HEAPS],
    pub total: VmaDetailedStatistics,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaBudget {
    pub statistics: VmaStatistics,
    pub usage: VkDeviceSize,
    pub budget: VkDeviceSize,
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct VmaAllocationCreateInfo {
    pub flags: VmaAllocationCreateFlags,
    pub usage: VmaMemoryUsage,
    pub requiredFlags: VkMemoryPropertyFlags,
    pub preferredFlags: VkMemoryPropertyFlags,
    pub memoryTypeBits: u32,
    pub pool: VmaPool,
    pub pUserData: *mut c_void,
    pub priority: f32,
}
impl Default for VmaAllocationCreateInfo {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            usage: VmaMemoryUsage::UNKNOWN,
            requiredFlags: Default::default(),
            preferredFlags: Default::default(),
            memoryTypeBits: 0,
            pool: Default::default(),
            pUserData: ptr::null_mut(),
            priority: 0f32,
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct VmaPoolCreateInfo {
    pub memoryTypeIndex: u32,
    pub flags: VmaPoolCreateFlags,
    pub blockSize: VkDeviceSize,
    pub minBlockCount: usize,
    pub maxBlockCount: usize,
    pub priority: f32,
    pub minAllocationAlignment: VkDeviceSize,
    pub pMemoryAllocateNext: *mut c_void,
}
impl Default for VmaPoolCreateInfo {
    fn default() -> Self {
        VmaPoolCreateInfo {
            memoryTypeIndex: 0,
            flags: VmaPoolCreateFlags::default(),
            blockSize: 0,
            minBlockCount: 0,
            maxBlockCount: 0,
            priority: 0f32,
            minAllocationAlignment: 0,
            pMemoryAllocateNext: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaAllocationInfo {
    pub memoryType: u32,
    pub deviceMemory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub pMappedData: *mut c_void,
    pub pUserData: *mut c_void,
    pub pName: *const c_char,
}
impl Default for VmaAllocationInfo {
    fn default() -> Self {
        Self {
            memoryType: 0,
            deviceMemory: Default::default(),
            offset: Default::default(),
            size: Default::default(),
            pMappedData: ptr::null_mut(),
            pUserData: ptr::null_mut(),
            pName: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaDefragmentationInfo {
    pub flags: VmaDefragmentationFlags,
    pub pool: VmaPool,
    pub maxBytesPerPass: VkDeviceSize,
    pub maxAllocationsPerPass: u32,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaDefragmentationMove {
    pub operation: VmaDefragmentationMoveOperation,
    pub srcAllocation: VmaAllocation,
    pub dstTmpAllocation: VmaAllocation,
}
impl Default for VmaDefragmentationMove {
    fn default() -> Self {
        Self {
            operation: VmaDefragmentationMoveOperation::COPY,
            srcAllocation: Default::default(),
            dstTmpAllocation: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaDefragmentationPassMoveInfo {
    pub moveCount: u32,
    pub pMoves: *mut VmaDefragmentationMove,
}
impl Default for VmaDefragmentationPassMoveInfo {
    fn default() -> Self {
        Self {
            moveCount: 0,
            pMoves: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaDefragmentationStats {
    pub bytesMoved: VkDeviceSize,
    pub bytesFreed: VkDeviceSize,
    pub allocationsMoved: u32,
    pub deviceMemoryBlocksFreed: u32,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaVirtualBlockCreateInfo {
    pub size: VkDeviceSize,
    pub flags: VmaVirtualBlockCreateFlags,
    pub pAllocationCallbacks: *const VkAllocationCallbacks,
}
impl Default for VmaVirtualBlockCreateInfo {
    fn default() -> Self {
        Self {
            size: Default::default(),
            flags: Default::default(),
            pAllocationCallbacks: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaVirtualAllocationCreateInfo {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub flags: VmaVirtualAllocationCreateFlags,
    pub pUserData: *mut c_void,
}
impl Default for VmaVirtualAllocationCreateInfo {
    fn default() -> Self {
        Self {
            size: Default::default(),
            alignment: Default::default(),
            flags: Default::default(),
            pUserData: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaVirtualAllocationInfo {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub pUserData: *mut c_void,
}
impl Default for VmaVirtualAllocationInfo {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            size: Default::default(),
            pUserData: ptr::null_mut(),
        }
    }
}

extern "C" {
    pub fn vmaCreateAllocator(
        pCreateInfo: *const VmaAllocatorCreateInfo,
        pAllocator: *mut VmaAllocator,
    ) -> VkResult;
    pub fn vmaDestroyAllocator(allocator: VmaAllocator);
    pub fn vmaGetAllocatorInfo(allocator: VmaAllocator, pAllocatorInfo: *mut VmaAllocatorInfo);
    pub fn vmaGetPhysicalDeviceProperties(
        allocator: VmaAllocator,
        ppPhysicalDeviceProperties: *mut *const VkPhysicalDeviceProperties,
    );
    pub fn vmaGetMemoryProperties(
        allocator: VmaAllocator,
        ppPhysicalDeviceMemoryProperties: *mut *const VkPhysicalDeviceMemoryProperties,
    );
    pub fn vmaGetMemoryTypeProperties(
        allocator: VmaAllocator,
        memoryTypeIndex: u32,
        pFlags: *mut VkMemoryPropertyFlags,
    );
    pub fn vmaSetCurrentFrameIndex(allocator: VmaAllocator, frameIndex: u32);
    pub fn vmaCalculateStatistics(allocator: VmaAllocator, pStats: *mut VmaTotalStatistics);
    pub fn vmaGetHeapBudgets(allocator: VmaAllocator, pBudgets: *mut VmaBudget);
    pub fn vmaFindMemoryTypeIndex(
        allocator: VmaAllocator,
        memoryTypeBits: u32,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> VkResult;
    pub fn vmaFindMemoryTypeIndexForBufferInfo(
        allocator: VmaAllocator,
        pBufferCreateInfo: *const VkBufferCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> VkResult;
    pub fn vmaFindMemoryTypeIndexForImageInfo(
        allocator: VmaAllocator,
        pImageCreateInfo: *const VkImageCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> VkResult;
    pub fn vmaCreatePool(
        allocator: VmaAllocator,
        pCreateInfo: *const VmaPoolCreateInfo,
        pPool: *mut VmaPool,
    ) -> VkResult;
    pub fn vmaDestroyPool(allocator: VmaAllocator, pool: VmaPool);
    pub fn vmaGetPoolStatistics(
        allocator: VmaAllocator,
        pool: VmaPool,
        pPoolStats: *mut VmaStatistics,
    );
    pub fn vmaCalculatePoolStatistics(
        allocator: VmaAllocator,
        pool: VmaPool,
        pPoolStats: *mut VmaDetailedStatistics,
    );
    pub fn vmaCheckPoolCorruption(allocator: VmaAllocator, pool: VmaPool) -> VkResult;
    pub fn vmaGetPoolName(allocator: VmaAllocator, pool: VmaPool, ppName: *mut *const c_char);
    pub fn vmaSetPoolName(allocator: VmaAllocator, pool: VmaPool, pName: *const c_char);
    pub fn vmaAllocateMemory(
        allocator: VmaAllocator,
        pVkMemoryRequirements: *const VkMemoryRequirements,
        pCreateInfo: *const VmaAllocationCreateInfo,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaAllocateMemoryPages(
        allocator: VmaAllocator,
        pVkMemoryRequirements: *const VkMemoryRequirements,
        pCreateInfo: *const VmaAllocationCreateInfo,
        allocationCount: usize,
        pAllocations: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaAllocateMemoryForBuffer(
        allocator: VmaAllocator,
        buffer: VkBuffer,
        pCreateInfo: *const VmaAllocationCreateInfo,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaAllocateMemoryForImage(
        allocator: VmaAllocator,
        image: VkImage,
        pCreateInfo: *const VmaAllocationCreateInfo,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaFreeMemory(allocator: VmaAllocator, allocation: VmaAllocation);
    pub fn vmaFreeMemoryPages(
        allocator: VmaAllocator,
        allocationCount: usize,
        pAllocations: *const VmaAllocation,
    );
    pub fn vmaGetAllocationInfo(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    );
    pub fn vmaSetAllocationUserData(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pUserData: *mut c_void,
    );
    pub fn vmaSetAllocationName(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pName: *const c_char,
    );
    pub fn vmaGetAllocationMemoryProperties(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pFlags: *mut VkMemoryPropertyFlags,
    );
    pub fn vmaMapMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        ppData: *mut *mut c_void,
    ) -> VkResult;
    pub fn vmaUnmapMemory(allocator: VmaAllocator, allocation: VmaAllocation);
    pub fn vmaFlushAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        offset: VkDeviceSize,
        size: VkDeviceSize,
    ) -> VkResult;
    pub fn vmaInvalidateAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        offset: VkDeviceSize,
        size: VkDeviceSize,
    ) -> VkResult;
    pub fn vmaFlushAllocations(
        allocator: VmaAllocator,
        allocationCount: u32,
        allocations: *const VmaAllocation,
        offsets: *const VkDeviceSize,
        sizes: *const VkDeviceSize,
    ) -> VkResult;
    pub fn vmaInvalidateAllocations(
        allocator: VmaAllocator,
        allocationCount: u32,
        allocations: *const VmaAllocation,
        offsets: *const VkDeviceSize,
        sizes: *const VkDeviceSize,
    ) -> VkResult;
    pub fn vmaCheckCorruption(allocator: VmaAllocator, memoryTypeBits: u32) -> VkResult;
    pub fn vmaBeginDefragmentation(
        allocator: VmaAllocator,
        pInfo: *const VmaDefragmentationInfo,
        pContext: *mut VmaDefragmentationContext,
    ) -> VkResult;
    pub fn vmaEndDefragmentation(
        allocator: VmaAllocator,
        context: VmaDefragmentationContext,
        pStats: *mut VmaDefragmentationStats,
    ) -> VkResult;
    pub fn vmaBeginDefragmentationPass(
        allocator: VmaAllocator,
        context: VmaDefragmentationContext,
        pPassInfo: *mut VmaDefragmentationPassMoveInfo,
    ) -> VkResult;
    pub fn vmaEndDefragmentationPass(
        allocator: VmaAllocator,
        context: VmaDefragmentationContext,
        pPassInfo: *mut VmaDefragmentationPassMoveInfo,
    ) -> VkResult;
    pub fn vmaBindBufferMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        buffer: VkBuffer,
    ) -> VkResult;
    pub fn vmaBindBufferMemory2(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        allocationLocalOffset: VkDeviceSize,
        buffer: VkBuffer,
        pNext: *const c_void,
    ) -> VkResult;
    pub fn vmaBindImageMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        image: VkImage,
    ) -> VkResult;
    pub fn vmaBindImageMemory2(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        allocationLocalOffset: VkDeviceSize,
        image: VkImage,
        pNext: *const c_void,
    ) -> VkResult;
    pub fn vmaCreateBuffer(
        allocator: VmaAllocator,
        pBufferCreateInfo: *const VkBufferCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pBuffer: *mut VkBuffer,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaCreateBufferWithAlignment(
        allocator: VmaAllocator,
        pBufferCreateInfo: *const VkBufferCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        minAlignment: VkDeviceSize,
        pBuffer: *mut VkBuffer,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaCreateAliasingBuffer(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pBufferCreateInfo: *const VkBufferCreateInfo,
        pBuffer: *mut VkBuffer,
    ) -> VkResult;
    pub fn vmaDestroyBuffer(allocator: VmaAllocator, buffer: VkBuffer, allocation: VmaAllocation);
    pub fn vmaCreateImage(
        allocator: VmaAllocator,
        pImageCreateInfo: *const VkImageCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pImage: *mut VkImage,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaCreateAliasingImage(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pImageCreateInfo: *const VkImageCreateInfo,
        pImage: *mut VkImage,
    ) -> VkResult;
    pub fn vmaDestroyImage(allocator: VmaAllocator, image: VkImage, allocation: VmaAllocation);
    pub fn vmaCreateVirtualBlock(
        pCreateInfo: *const VmaVirtualBlockCreateInfo,
        pVirtualBlock: *mut VmaVirtualBlock,
    ) -> VkResult;
    pub fn vmaDestroyVirtualBlock(virtualBlock: VmaVirtualBlock);
    pub fn vmaIsVirtualBlockEmpty(virtualBlock: VmaVirtualBlock) -> VkBool32;
    pub fn vmaGetVirtualAllocationInfo(
        virtualBlock: VmaVirtualBlock,
        allocation: VmaVirtualAllocation,
        pVirtualAllocInfo: *mut VmaVirtualAllocationInfo,
    );
    pub fn vmaVirtualAllocate(
        virtualBlock: VmaVirtualBlock,
        pCreateInfo: *const VmaVirtualAllocationCreateInfo,
        pAllocation: *mut VmaVirtualAllocation,
        pOffset: *mut VkDeviceSize,
    ) -> VkResult;
    pub fn vmaVirtualFree(virtualBlock: VmaVirtualBlock, allocation: VmaVirtualAllocation);
    pub fn vmaClearVirtualBlock(virtualBlock: VmaVirtualBlock);
    pub fn vmaSetVirtualAllocationUserData(
        virtualBlock: VmaVirtualBlock,
        allocation: VmaVirtualAllocation,
        pUserData: *mut c_void,
    );
    pub fn vmaGetVirtualBlockStatistics(virtualBlock: VmaVirtualBlock, pStats: *mut VmaStatistics);
    pub fn vmaCalculateVirtualBlockStatistics(
        virtualBlock: VmaVirtualBlock,
        pStats: *mut VmaDetailedStatistics,
    );
    pub fn vmaBuildVirtualBlockStatsString(
        virtualBlock: VmaVirtualBlock,
        ppStatsString: *mut *mut c_char,
        detailedMap: VkBool32,
    );
    pub fn vmaFreeVirtualBlockStatsString(virtualBlock: VmaVirtualBlock, pStatsString: *mut c_char);
    pub fn vmaBuildStatsString(
        allocator: VmaAllocator,
        ppStatsString: *mut *mut c_char,
        detailedMap: VkBool32,
    );
    pub fn vmaFreeStatsString(allocator: VmaAllocator, pStatsString: *mut c_char);
}
