#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::os::raw::c_void;
use std::ptr;

use crate::*;

handle!(
    VkAccelerationStructureKHR,
    NonDispatchableHandle,
    VkObjectType::ACCELERATION_STRUCTURE_KHR
);

enums! {
    enum VkAccelerationStructureBuildTypeKHR {
        HOST_KHR = 0,
        DEVICE_KHR = 1,
        HOST_OR_DEVICE_KHR = 2,
    },
    enum VkAccelerationStructureCompatibilityKHR{
        COMPATIBLE_KHR = 0,
        INCOMPATIBLE_KHR = 1,
    },
    enum VkAccelerationStructureTypeKHR {
        TOP_LEVEL_KHR = 0,
        BOTTOM_LEVEL_KHR = 1,
        GENERIC_KHR = 2,
    },
    enum VkBuildAccelerationStructureModeKHR{
        BUILD_KHR = 0,
        UPDATE_KHR = 1,
    },
    enum VkCopyAccelerationStructureModeKHR {
        CLONE_KHR = 0,
        COMPACT_KHR = 1,
        SERIALIZE_KHR = 2,
        DESERIALIZE_KHR = 3,
    },
    enum VkGeometryTypeKHR {
        TRIANGLES_KHR = 0,
        AABBS_KHR = 1,
        INSTANCES_KHR = 2,
    },
}

bitmasks! {
    VkAccelerationStructureCreateFlagsKHR = enum VkAccelerationStructureCreateFlagBitsKHR{
        DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 0x00000001,
    },
    VkBuildAccelerationStructureFlagsKHR = enum VkBuildAccelerationStructureFlagBitsKHR {
        ALLOW_UPDATE_BIT_KHR = 0x00000001,
        ALLOW_COMPACTION_BIT_KHR = 0x00000002,
        PREFER_FAST_TRACE_BIT_KHR = 0x00000004,
        PREFER_FAST_BUILD_BIT_KHR = 0x00000008,
        LOW_MEMORY_BIT_KHR = 0x00000010,
    },
    VkGeometryFlagsKHR = enum VkGeometryFlagBitsKHR {
        OPAQUE_BIT_KHR = 0x00000001,
        NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
    },
    VkGeometryInstanceFlagsKHR = enum VkGeometryInstanceFlagBitsKHR {
        TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
        TRIANGLE_FLIP_FACING_BIT_KHR = 0x00000002,
        FORCE_OPAQUE_BIT_KHR = 0x00000004,
        FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
    },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct VkAabbPositionsKHR {
    pub minX: f32,
    pub minY: f32,
    pub minZ: f32,
    pub maxX: f32,
    pub maxY: f32,
    pub maxZ: f32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub mode: VkBuildAccelerationStructureModeKHR,
    pub srcAccelerationStructure: VkAccelerationStructureKHR,
    pub dstAccelerationStructure: VkAccelerationStructureKHR,
    pub geometryCount: u32,
    pub pGeometries: *const VkAccelerationStructureGeometryKHR,
    pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
    pub scratchData: VkDeviceOrHostAddressKHR,
}
impl Default for VkAccelerationStructureBuildGeometryInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            pNext: ptr::null(),
            r#type: VkAccelerationStructureTypeKHR::TOP_LEVEL_KHR,
            flags: Default::default(),
            mode: VkBuildAccelerationStructureModeKHR::BUILD_KHR,
            srcAccelerationStructure: Default::default(),
            dstAccelerationStructure: Default::default(),
            geometryCount: Default::default(),
            pGeometries: ptr::null(),
            ppGeometries: ptr::null(),
            scratchData: VkDeviceOrHostAddressKHR {
                hostAddress: ptr::null_mut(),
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
    pub primitiveCount: u32,
    pub primitiveOffset: u32,
    pub firstVertex: u32,
    pub transformOffset: u32,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructureSize: VkDeviceSize,
    pub updateScratchSize: VkDeviceSize,
    pub buildScratchSize: VkDeviceSize,
}
impl Default for VkAccelerationStructureBuildSizesInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
            pNext: ptr::null(),
            accelerationStructureSize: Default::default(),
            updateScratchSize: Default::default(),
            buildScratchSize: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAccelerationStructureCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub createFlags: VkAccelerationStructureCreateFlagsKHR,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub deviceAddress: VkDeviceAddress,
}
impl Default for VkAccelerationStructureCreateInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            createFlags: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
            r#type: VkAccelerationStructureTypeKHR::TOP_LEVEL_KHR,
            deviceAddress: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
}
impl Default for VkAccelerationStructureDeviceAddressInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
            pNext: ptr::null(),
            accelerationStructure: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)] // FIXME: Because of unions with non-`Copy` fields are unstable, so VkAccelerationStructureGeometryAabbsDataKHR need to have Copy trait currently.
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub data: VkDeviceOrHostAddressConstKHR,
    pub stride: VkDeviceSize,
}
impl Default for VkAccelerationStructureGeometryAabbsDataKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR,
            pNext: ptr::null(),
            data: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
            stride: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)] // FIXME: Because of unions with non-`Copy` fields are unstable, so VkAccelerationStructureGeometryInstancesDataKHR need to have Copy trait currently.
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub arrayOfPointers: VkBool32,
    pub data: VkDeviceOrHostAddressConstKHR,
}
impl Default for VkAccelerationStructureGeometryInstancesDataKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
            pNext: ptr::null(),
            arrayOfPointers: Default::default(),
            data: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkAccelerationStructureGeometryKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeKHR,
    pub geometry: VkAccelerationStructureGeometryDataKHR,
    pub flags: VkGeometryFlagsKHR,
}
impl Default for VkAccelerationStructureGeometryKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::TRIANGLES_KHR,
            geometry: VkAccelerationStructureGeometryDataKHR {
                triangles: Default::default(),
            },
            flags: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)] // FIXME: Because of unions with non-`Copy` fields are unstable, so VkAccelerationStructureGeometryTrianglesDataKHR need to have Copy trait currently.
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
    pub vertexStride: VkDeviceSize,
    pub maxVertex: u32,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceOrHostAddressConstKHR,
    pub transformData: VkDeviceOrHostAddressConstKHR,
}
impl Default for VkAccelerationStructureGeometryTrianglesDataKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
            pNext: ptr::null(),
            vertexFormat: VkFormat::UNDEFINED,
            vertexData: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
            vertexStride: Default::default(),
            indexType: VkIndexType::NONE_KHR,
            maxVertex: 0,
            indexData: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
            transformData: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct VkAccelerationStructureInstanceKHR {
    pub transform: VkTransformMatrixKHR,
    instanceCustomIndex_mask: u32,
    instanceShaderBindingTableRecordOffset_flags: u32,
    pub accelerationStructureReference: u64,
}
impl VkAccelerationStructureInstanceKHR {
    pub fn new() -> Self {
        Self {
            transform: Default::default(),
            instanceCustomIndex_mask: 0,
            instanceShaderBindingTableRecordOffset_flags: 0,
            accelerationStructureReference: Default::default(),
        }
    }

    pub fn set_instance_custom_index(&mut self, index: u32) {
        if cfg!(target_endian = "little") {
            self.instanceCustomIndex_mask =
                (0xFF000000u32 & self.instanceCustomIndex_mask) | (0x00FFFFFFu32 & index);
        } else {
            self.instanceCustomIndex_mask =
                (0x000000FFu32 & self.instanceCustomIndex_mask) | (index << 8);
        }
    }

    pub fn get_instance_custom_index(&self) -> u32 {
        if cfg!(target_endian = "little") {
            0x00FFFFFFu32 & self.instanceCustomIndex_mask
        } else {
            self.instanceCustomIndex_mask >> 8
        }
    }

    pub fn set_mask(&mut self, mask: u32) {
        if cfg!(target_endian = "little") {
            self.instanceCustomIndex_mask =
                (self.instanceCustomIndex_mask & 0x00FFFFFFu32) | (mask << 24);
        } else {
            self.instanceCustomIndex_mask =
                (self.instanceCustomIndex_mask & 0xFFFFFF00u32) | (0x000000FFu32 & mask);
        }
    }

    pub fn get_mask(&self) -> u32 {
        if cfg!(target_endian = "little") {
            self.instanceCustomIndex_mask >> 24
        } else {
            0x000000FFu32 & self.instanceCustomIndex_mask
        }
    }

    pub fn set_instance_shader_binding_table_record_offset(&mut self, index: u32) {
        if cfg!(target_endian = "little") {
            self.instanceShaderBindingTableRecordOffset_flags = (0xFF000000u32
                & self.instanceShaderBindingTableRecordOffset_flags)
                | (0x00FFFFFFu32 & index);
        } else {
            self.instanceShaderBindingTableRecordOffset_flags =
                (0x000000FFu32 & self.instanceShaderBindingTableRecordOffset_flags) | (index << 8);
        }
    }

    pub fn get_instance_shader_binding_table_record_offset(&self) -> u32 {
        if cfg!(target_endian = "little") {
            0x00FFFFFFu32 & self.instanceShaderBindingTableRecordOffset_flags
        } else {
            self.instanceShaderBindingTableRecordOffset_flags >> 8
        }
    }

    pub fn set_flags(&mut self, flags: VkGeometryInstanceFlagsKHR) {
        let flags: u32 = flags.bits();
        if cfg!(target_endian = "little") {
            self.instanceShaderBindingTableRecordOffset_flags =
                (self.instanceShaderBindingTableRecordOffset_flags & 0x00FFFFFFu32) | (flags << 24);
        } else {
            self.instanceShaderBindingTableRecordOffset_flags =
                (self.instanceShaderBindingTableRecordOffset_flags & 0xFFFFFF00u32)
                    | (0x000000FFu32 & flags);
        }
    }

    pub fn get_flags(&self) -> VkGeometryInstanceFlagsKHR {
        if cfg!(target_endian = "little") {
            unsafe {
                VkGeometryInstanceFlagsKHR::from_bits_unchecked(
                    self.instanceShaderBindingTableRecordOffset_flags >> 24,
                )
            }
        } else {
            unsafe {
                VkGeometryInstanceFlagsKHR::from_bits_unchecked(
                    0x000000FFu32 & self.instanceShaderBindingTableRecordOffset_flags,
                )
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkAccelerationStructureVersionInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pVersionData: *const u8,
}
impl Default for VkAccelerationStructureVersionInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR,
            pNext: ptr::null(),
            pVersionData: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkCopyAccelerationStructureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkAccelerationStructureKHR,
    pub dst: VkAccelerationStructureKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR,
            pNext: ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: VkCopyAccelerationStructureModeKHR::CLONE_KHR,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkAccelerationStructureKHR,
    pub dst: VkDeviceOrHostAddressKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyAccelerationStructureToMemoryInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR,
            pNext: ptr::null(),
            src: Default::default(),
            dst: VkDeviceOrHostAddressKHR {
                hostAddress: ptr::null_mut(),
            },
            mode: VkCopyAccelerationStructureModeKHR::CLONE_KHR,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub src: VkDeviceOrHostAddressConstKHR,
    pub dst: VkAccelerationStructureKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}
impl Default for VkCopyMemoryToAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR,
            pNext: ptr::null(),
            src: VkDeviceOrHostAddressConstKHR {
                hostAddress: ptr::null(),
            },
            dst: Default::default(),
            mode: VkCopyAccelerationStructureModeKHR::CLONE_KHR,
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct VkTransformMatrixKHR {
    pub matrix: [[f32; 4]; 3],
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub accelerationStructure: VkBool32,
    pub accelerationStructureCaptureReplay: VkBool32,
    pub accelerationStructureIndirectBuild: VkBool32,
    pub accelerationStructureHostCommands: VkBool32,
    pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
}
impl Default for VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
            pNext: ptr::null_mut(),
            accelerationStructure: VkBool32::FALSE,
            accelerationStructureCaptureReplay: VkBool32::FALSE,
            accelerationStructureIndirectBuild: VkBool32::FALSE,
            accelerationStructureHostCommands: VkBool32::FALSE,
            descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32::FALSE,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxGeometryCount: u64,
    pub maxInstanceCount: u64,
    pub maxPrimitiveCount: u64,
    pub maxPerStageDescriptorAccelerationStructures: u32,
    pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures: u32,
    pub maxDescriptorSetAccelerationStructures: u32,
    pub maxDescriptorSetUpdateAfterBindAccelerationStructures: u32,
    pub minAccelerationStructureScratchOffsetAlignment: u32,
}
impl Default for VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR,
            pNext: ptr::null(),
            maxGeometryCount: 0,
            maxInstanceCount: 0,
            maxPrimitiveCount: 0,
            maxPerStageDescriptorAccelerationStructures: 0,
            maxPerStageDescriptorUpdateAfterBindAccelerationStructures: 0,
            maxDescriptorSetAccelerationStructures: 0,
            maxDescriptorSetUpdateAfterBindAccelerationStructures: 0,
            minAccelerationStructureScratchOffsetAlignment: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}
impl Default for VkWriteDescriptorSetAccelerationStructureKHR {
    fn default() -> Self {
        Self {
            sType: VkStructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
            pNext: ptr::null(),
            accelerationStructureCount: 0,
            pAccelerationStructures: ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkAccelerationStructureGeometryDataKHR {
    pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
    pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}
impl Debug for VkAccelerationStructureGeometryDataKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe {
            if self.triangles.sType
                == VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR
            {
                write!(
                    f,
                    "VkAccelerationStructureGeometryDataKHR {{ triangles: {:?} }}",
                    self.triangles
                )
            } else if self.aabbs.sType
                == VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR
            {
                write!(
                    f,
                    "VkAccelerationStructureGeometryDataKHR {{ aabbs: {:?} }}",
                    self.aabbs
                )
            } else if self.instances.sType
                == VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR
            {
                write!(
                    f,
                    "VkAccelerationStructureGeometryDataKHR {{ instances: {:?} }}",
                    self.instances
                )
            } else {
                unreachable!()
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkDeviceOrHostAddressConstKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *const c_void,
}
impl Debug for VkDeviceOrHostAddressConstKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe {
            write!(
                f,
                "VkDeviceOrHostAddressConstKHR {{ deviceAddress: {:?}, hostAddress: {:?} }}",
                self.deviceAddress, self.hostAddress
            )
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkDeviceOrHostAddressKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *mut c_void,
}
impl Debug for VkDeviceOrHostAddressKHR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe {
            write!(
                f,
                "VkDeviceOrHostAddressKHR {{ deviceAddress: {:?}, hostAddress: {:?} }}",
                self.deviceAddress, self.hostAddress
            )
        }
    }
}
