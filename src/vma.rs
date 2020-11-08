#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::ffi::c_void;
use std::fmt::{Debug, Display, Error, Formatter};
use std::os::raw::c_char;
use std::ptr;
use crate::*;

handle!(VmaAllocator, DispatchableHandle);
handle!(VmaPool, DispatchableHandle);
handle!(VmaAllocation, DispatchableHandle);
handle!(VmaDefragmentationContext, DispatchableHandle);

bitmasks! {
    {
        VmaAllocationCreateFlags,
        enum VmaAllocationCreateFlagBits{
            DEDICATED_MEMORY_BIT = 0x00000001,
            NEVER_ALLOCATE_BIT = 0x00000002,
            MAPPED_BIT = 0x00000004,
            CAN_BECOME_LOST_BIT = 0x00000008,
            CAN_MAKE_OTHER_LOST_BIT = 0x00000010,
            USER_DATA_COPY_STRING_BIT = 0x00000020,
            UPPER_ADDRESS_BIT = 0x00000040,
            DONT_BIND_BIT = 0x00000080,
            WITHIN_BUDGET_BIT = 0x00000100,
            STRATEGY_BEST_FIT_BIT  = 0x00010000,
            STRATEGY_WORST_FIT_BIT = 0x00020000,
            STRATEGY_FIRST_FIT_BIT = 0x00040000,
            STRATEGY_MIN_MEMORY_BIT =  0x00010000, // STRATEGY_BEST_FIT_BIT,
            STRATEGY_MIN_TIME_BIT = 0x00040000, // STRATEGY_FIRST_FIT_BIT,
            STRATEGY_MIN_FRAGMENTATION_BIT = 0x00020000, // STRATEGY_WORST_FIT_BIT,
            STRATEGY_MASK = 0x00070000, // STRATEGY_BEST_FIT_BIT|STRATEGY_WORST_FIT_BIT|STRATEGY_FIRST_FIT_BIT,
        }
    },
    {
        VmaAllocatorCreateFlags,
        enum VmaAllocatorCreateFlagBits{
            EXTERNALLY_SYNCHRONIZED_BIT = 0x00000001,
            KHR_DEDICATED_ALLOCATION_BIT = 0x00000002,
            KHR_BIND_MEMORY2_BIT = 0x00000004,
            EXT_MEMORY_BUDGET_BIT = 0x00000008,
        }
    },
    {
        VmaRecordFlags,
        enum VmaRecordFlagBits{
            FLUSH_AFTER_CALL_BIT = 0x00000001,
        }
    },
    {
        VmaPoolCreateFlags,
        enum VmaPoolCreateFlagBits{
            IGNORE_BUFFER_IMAGE_GRANULARITY_BIT = 0x00000002,
            LINEAR_ALGORITHM_BIT = 0x00000004,
            BUDDY_ALGORITHM_BIT = 0x00000008,
            ALGORITHM_MASK = 0x0000000c, // LINEAR_ALGORITHM_BIT|BUDDY_ALGORITHM_BIT,
        }
    },
    {
        VmaDefragmentationFlags,
        enum VmaDefragmentationFlagBits{
            _RESERVED = 0,
        }
    }
}

enums! {
    enum VmaMemoryUsage{
        UNKNOWN = 0,
        GPU_ONLY = 1,
        CPU_ONLY = 2,
        CPU_TO_GPU = 3,
        GPU_TO_CPU = 4,
        CPU_COPY = 5,
        GPU_LAZILY_ALLOCATED = 6,
    },
}

pub type PFN_vmaAllocateDeviceMemoryFunction = extern "C" fn(
    allocator: VmaAllocator,
    memoryType: u32,
    memory: VkDeviceMemory,
    size: VkDeviceSize,
);
pub type PFN_vmaFreeDeviceMemoryFunction = extern "C" fn(
    allocator: VmaAllocator,
    memoryType: u32,
    memory: VkDeviceMemory,
    size: VkDeviceSize,
);
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

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaAllocationCreateInfo {
    pub flags: VmaAllocationCreateFlags,
    pub usage: VmaMemoryUsage,
    pub requiredFlags: VkMemoryPropertyFlags,
    pub preferredFlags: VkMemoryPropertyFlags,
    pub memoryTypeBits: u32,
    pub pool: VmaPool,
    pub pUserData: *mut c_void,
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
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaStatInfo {
    pub blockCount: u32,
    pub allocationCount: u32,
    pub unusedRangeCount: u32,
    pub usedBytes: VkDeviceSize,
    pub unusedBytes: VkDeviceSize,
    pub allocationSizeMin: VkDeviceSize,
    pub allocationSizeAvg: VkDeviceSize,
    pub allocationSizeMax: VkDeviceSize,
    pub unusedRangeSizeMin: VkDeviceSize,
    pub unusedRangeSizeAvg: VkDeviceSize,
    pub unusedRangeSizeMax: VkDeviceSize,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaStats {
    pub memoryType: [VmaStatInfo; VK_MAX_MEMORY_TYPES],
    pub memoryHeap: [VmaStatInfo; VK_MAX_MEMORY_HEAPS],
    pub total: VmaStatInfo,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaDeviceMemoryCallbacks {
    pub pfnAllocate: PFN_vmaAllocateDeviceMemoryFunction,
    pub pfnFree: PFN_vmaFreeDeviceMemoryFunction,
}
impl Default for VmaDeviceMemoryCallbacks {
    fn default() -> Self {
        extern "C" fn allocate(
            _allocator: VmaAllocator,
            _memoryType: u32,
            _memory: VkDeviceMemory,
            _size: VkDeviceSize,
        ) {
            unimplemented!()
        }
        extern "C" fn free(
            _allocator: VmaAllocator,
            _memoryType: u32,
            _memory: VkDeviceMemory,
            _size: VkDeviceSize,
        ) {
            unimplemented!()
        }
        Self {
            pfnAllocate: allocate,
            pfnFree: free,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaVulkanFunctions {
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
    pub vkGetBufferMemoryRequirements2KHR: PFN_vkGetBufferMemoryRequirements2KHR,
    pub vkGetImageMemoryRequirements2KHR: PFN_vkGetImageMemoryRequirements2KHR,
    pub vkBindBufferMemory2KHR: PFN_vkBindBufferMemory2KHR,
    pub vkBindImageMemory2KHR: PFN_vkBindImageMemory2KHR,
    pub vkGetPhysicalDeviceMemoryProperties2KHR: PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
}
impl Default for VmaVulkanFunctions {
    fn default() -> Self {
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
        Self {
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
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VmaRecordSettings {
    pub flags: VmaRecordFlags,
    pub pFilePath: *const c_char,
}
impl Default for VmaRecordSettings {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            pFilePath: ptr::null(),
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
    pub frameInUseCount: u32,
    pub pHeapSizeLimit: *const VkDeviceSize,
    pub pVulkanFunctions: *const VmaVulkanFunctions,
    pub pRecordSettings: *const VmaRecordSettings,
    pub instance: VkInstance,
    pub vulkanApiVersion: u32,
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
            frameInUseCount: 0,
            pHeapSizeLimit: ptr::null(),
            pVulkanFunctions: ptr::null(),
            pRecordSettings: ptr::null(),
            instance: Default::default(),
            vulkanApiVersion: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaPoolCreateInfo {
    pub memoryTypeIndex: u32,
    pub flags: VmaPoolCreateFlags,
    pub blockSize: VkDeviceSize,
    pub minBlockCount: usize,
    pub maxBlockCount: usize,
    pub frameInUseCount: u32,
}

#[deprecated(
note = "This is a part of the old interface. It is recommended to use structure VmaDefragmentationInfo2 and function vmaDefragmentationBegin() instead."
)]
#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaDefragmentationInfo {
    pub maxBytesToMove: VkDeviceSize,
    pub maxAllocationsToMove: u32,
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
pub struct VmaDefragmentationInfo2 {
    pub flags: VmaDefragmentationFlags,
    pub allocationCount: u32,
    pub pAllocations: *mut VmaAllocation,
    pub pAllocationsChanged: *mut VkBool32,
    pub poolCount: u32,
    pub pPools: *mut VmaPool,
    pub maxCpuBytesToMove: VkDeviceSize,
    pub maxCpuAllocationsToMove: u32,
    pub maxGpuBytesToMove: VkDeviceSize,
    pub maxGpuAllocationsToMove: u32,
    pub commandBuffer: VkCommandBuffer,
}
impl Default for VmaDefragmentationInfo2 {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            allocationCount: 0,
            pAllocations: ptr::null_mut(),
            pAllocationsChanged: ptr::null_mut(),
            poolCount: 0,
            pPools: ptr::null_mut(),
            maxCpuBytesToMove: Default::default(),
            maxCpuAllocationsToMove: 0,
            maxGpuBytesToMove: Default::default(),
            maxGpuAllocationsToMove: 0,
            commandBuffer: Default::default(),
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
pub struct VmaBudget {
    pub blockBytes: VkDeviceSize,
    pub allocationBytes: VkDeviceSize,
    pub usage: VkDeviceSize,
    pub budget: VkDeviceSize,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VmaPoolStats {
    pub size: VkDeviceSize,
    pub unusedSize: VkDeviceSize,
    pub allocationCount: usize,
    pub unusedRangeCount: usize,
    pub unusedRangeSizeMax: VkDeviceSize,
    pub blockCount: usize,
}

extern "C" {
    pub fn vmaAllocateMemory(
        allocator: VmaAllocator,
        pVkMemoryRequirements: *const VkMemoryRequirements,
        pCreateInfo: *const VmaAllocationCreateInfo,
        pAllocation: *mut VmaAllocation,
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
    pub fn vmaAllocateMemoryPages(
        allocator: VmaAllocator,
        pVkMemoryRequirements: *const VkMemoryRequirements,
        pCreateInfo: *const VmaAllocationCreateInfo,
        allocationCount: usize,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    // pub fn vmaBeginDefragmentationPass(
    //     allocator: VmaAllocator,
    //     context: VmaDefragmentationContext,
    //     pInfo: *mut VmaDefragmentationPassInfo,
    // )->VkResult;
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
    pub fn vmaBuildStatsString(
        allocator: VmaAllocator,
        ppStatsString: *mut *mut c_char,
        detailedMap: VkBool32,
    );
    pub fn vmaCalculateStats(allocator: VmaAllocator, pStats: *mut VmaStats);
    pub fn vmaCheckCorruption(allocator: VmaAllocator, memoryTypeBits: u32) -> VkResult;
    pub fn vmaCheckPoolCorruption(allocator: VmaAllocator, pool: VmaPool) -> VkResult;
    pub fn vmaCreateAllocator(
        pCreateInfo: *const VmaAllocatorCreateInfo,
        pAllocator: *mut VmaAllocator,
    ) -> VkResult;
    pub fn vmaCreateBuffer(
        allocator: VmaAllocator,
        pBufferCreateInfo: *const VkBufferCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pBuffer: *mut VkBuffer,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaCreateImage(
        allocator: VmaAllocator,
        pImageCreateInfo: *const VkImageCreateInfo,
        pAllocationCreateInfo: *const VmaAllocationCreateInfo,
        pImage: *mut VkImage,
        pAllocation: *mut VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    ) -> VkResult;
    pub fn vmaCreateLostAllocation(allocator: VmaAllocator, pAllocation: *mut VmaAllocation);
    pub fn vmaCreatePool(
        allocator: VmaAllocator,
        pCreateInfo: *const VmaPoolCreateInfo,
        pPool: *mut VmaPool,
    ) -> VkResult;
    #[deprecated(
    note = "This is a part of the old interface. It is recommended to use structure VmaDefragmentationInfo2 and function vmaDefragmentationBegin() instead."
    )]
    #[allow(deprecated)]
    pub fn vmaDefragment(
        allocator: VmaAllocator,
        pAllocations: *const VmaAllocation,
        allocationCount: usize,
        pAllocationsChanged: *mut VkBool32,
        pDefragmentationInfo: *const VmaDefragmentationInfo,
        pDefragmentationStats: *mut VmaDefragmentationStats,
    ) -> VkResult;
    pub fn vmaDefragmentationBegin(
        allocator: VmaAllocator,
        pInfo: *const VmaDefragmentationInfo2,
        pStats: *mut VmaDefragmentationStats,
        pContext: *mut VmaDefragmentationContext,
    ) -> VkResult;
    pub fn vmaDefragmentationEnd(
        allocator: VmaAllocator,
        context: VmaDefragmentationContext,
    ) -> VkResult;
    pub fn vmaDestroyAllocator(allocator: VmaAllocator);
    pub fn vmaDestroyBuffer(allocator: VmaAllocator, buffer: VkBuffer, allocation: VmaAllocation);
    pub fn vmaDestroyImage(allocator: VmaAllocator, image: VkImage, allocation: VmaAllocation);
    pub fn vmaDestroyPool(allocator: VmaAllocator, pool: VmaPool);
    // pub fn vmaEndDefragmentationPass(
    //     allocator: VmaAllocator,
    //     context: VmaDefragmentationContext,
    // )->VkResult;
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

    pub fn vmaFlushAllocation(
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

    pub fn vmaFreeMemory(allocator: VmaAllocator, allocation: VmaAllocation);

    pub fn vmaFreeMemoryPages(
        allocator: VmaAllocator,
        allocationCount: usize,
        pAllocations: *const VmaAllocation,
    );

    pub fn vmaFreeStatsString(allocator: VmaAllocator, pStatsString: *mut c_char);

    pub fn vmaGetAllocationInfo(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pAllocationInfo: *mut VmaAllocationInfo,
    );

    pub fn vmaGetAllocatorInfo(allocator: VmaAllocator, pAllocatorInfo: *mut VmaAllocatorInfo);

    pub fn vmaGetBudget(allocator: VmaAllocator, pBudget: *mut VmaBudget);

    pub fn vmaGetMemoryProperties(
        allocator: VmaAllocator,
        ppPhysicalDeviceMemoryProperties: *mut *const VkPhysicalDeviceMemoryProperties,
    );

    pub fn vmaGetMemoryTypeProperties(
        allocator: VmaAllocator,
        memoryTypeIndex: u32,
        pFlags: *mut VkMemoryPropertyFlags,
    );

    pub fn vmaGetPhysicalDeviceProperties(
        allocator: VmaAllocator,
        ppPhysicalDeviceProperties: *mut *const VkPhysicalDeviceProperties,
    );

    pub fn vmaGetPoolName(allocator: VmaAllocator, pool: VmaPool, ppName: *mut *const c_char);

    pub fn vmaGetPoolStats(allocator: VmaAllocator, pool: VmaPool, pPoolStats: *mut VmaPoolStats);

    pub fn vmaInvalidateAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        offset: VkDeviceSize,
        size: VkDeviceSize,
    ) -> VkResult;

    pub fn vmaInvalidateAllocations(
        allocator: VmaAllocator,
        allocationCount: u32,
        allocations: *const VmaAllocation,
        offsets: *const VkDeviceSize,
        sizes: *const VkDeviceSize,
    ) -> VkResult;

    pub fn vmaMakePoolAllocationsLost(
        allocator: VmaAllocator,
        pool: VmaPool,
        pLostAllocationCount: *mut usize,
    );

    pub fn vmaMapMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        ppData: *mut *mut c_void,
    ) -> VkResult;

    pub fn vmaResizeAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        newSize: VkDeviceSize,
    ) -> VkResult;

    pub fn vmaSetAllocationUserData(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pUserData: *mut c_void,
    );

    pub fn vmaSetCurrentFrameIndex(allocator: VmaAllocator, frameIndex: u32);

    pub fn vmaSetPoolName(allocator: VmaAllocator, pool: VmaPool, pName: *const c_char);

    pub fn vmaTouchAllocation(allocator: VmaAllocator, allocation: VmaAllocation) -> VkBool32;

    pub fn vmaUnmapMemory(allocator: VmaAllocator, allocation: VmaAllocation);

}