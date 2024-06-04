#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "processor_info"]
#[doc(alias = "processor_info")]
pub struct PROCESSOR_INFO {
    device_class: DEVICE_CLASS,
    _reserved1: [u8; 0x04],
    interface_version: INTERFACE_VERSION,
    core_count: CORE_COUNT,
    ctrl_size: CTRL_SIZE,
    instr_mem_size: INSTR_MEM_SIZE,
    instr_mem_start_lo: INSTR_MEM_START_LO,
    instr_mem_start_hi: INSTR_MEM_START_HI,
    cq_mem_size_lo: CQ_MEM_SIZE_LO,
    cq_mem_size_hi: CQ_MEM_SIZE_HI,
    cq_mem_start_lo: CQ_MEM_START_LO,
    cq_mem_start_hi: CQ_MEM_START_HI,
    data_mem_size_lo: DATA_MEM_SIZE_LO,
    data_mem_size_hi: DATA_MEM_SIZE_HI,
    data_mem_start_lo: DATA_MEM_START_LO,
    data_mem_start_hi: DATA_MEM_START_HI,
    feature_flags_lo: FEATURE_FLAGS_LO,
    feature_flags_hi: FEATURE_FLAGS_HI,
    ptr_size: PTR_SIZE,
    debug_feature_support: DEBUG_FEATURE_SUPPORT,
    bp_count: BP_COUNT,
}
impl PROCESSOR_INFO {
    #[doc = "0x00 - Device class (0x774)"]
    #[inline(always)]
    pub const fn device_class(&self) -> &DEVICE_CLASS {
        &self.device_class
    }
    #[doc = "0x08 - Interface version (0x3)"]
    #[inline(always)]
    pub const fn interface_version(&self) -> &INTERFACE_VERSION {
        &self.interface_version
    }
    #[doc = "0x0c - Core count"]
    #[inline(always)]
    pub const fn core_count(&self) -> &CORE_COUNT {
        &self.core_count
    }
    #[doc = "0x10 - CTRL size, per core, in bytes"]
    #[inline(always)]
    pub const fn ctrl_size(&self) -> &CTRL_SIZE {
        &self.ctrl_size
    }
    #[doc = "0x14 - Instruction memory size, in bytes"]
    #[inline(always)]
    pub const fn instr_mem_size(&self) -> &INSTR_MEM_SIZE {
        &self.instr_mem_size
    }
    #[doc = "0x18 - Start of instruction memory, low 32 bits"]
    #[inline(always)]
    pub const fn instr_mem_start_lo(&self) -> &INSTR_MEM_START_LO {
        &self.instr_mem_start_lo
    }
    #[doc = "0x1c - Start of instruction memory, high 32-bits"]
    #[inline(always)]
    pub const fn instr_mem_start_hi(&self) -> &INSTR_MEM_START_HI {
        &self.instr_mem_start_hi
    }
    #[doc = "0x20 - CQ memory size, low 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_size_lo(&self) -> &CQ_MEM_SIZE_LO {
        &self.cq_mem_size_lo
    }
    #[doc = "0x24 - CQ memory size, high 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_size_hi(&self) -> &CQ_MEM_SIZE_HI {
        &self.cq_mem_size_hi
    }
    #[doc = "0x28 - CQ memory start, low 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_start_lo(&self) -> &CQ_MEM_START_LO {
        &self.cq_mem_start_lo
    }
    #[doc = "0x2c - CQ memory start, high 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_start_hi(&self) -> &CQ_MEM_START_HI {
        &self.cq_mem_start_hi
    }
    #[doc = "0x30 - Data memory size in bytes, low 32 bits"]
    #[inline(always)]
    pub const fn data_mem_size_lo(&self) -> &DATA_MEM_SIZE_LO {
        &self.data_mem_size_lo
    }
    #[doc = "0x34 - Data memory size in bytes, high 32 bits"]
    #[inline(always)]
    pub const fn data_mem_size_hi(&self) -> &DATA_MEM_SIZE_HI {
        &self.data_mem_size_hi
    }
    #[doc = "0x38 - Data memory start, low 32 bits"]
    #[inline(always)]
    pub const fn data_mem_start_lo(&self) -> &DATA_MEM_START_LO {
        &self.data_mem_start_lo
    }
    #[doc = "0x3c - Data memory start, high 32 bits"]
    #[inline(always)]
    pub const fn data_mem_start_hi(&self) -> &DATA_MEM_START_HI {
        &self.data_mem_start_hi
    }
    #[doc = "0x40 - Feature flags, low 32 bits"]
    #[inline(always)]
    pub const fn feature_flags_lo(&self) -> &FEATURE_FLAGS_LO {
        &self.feature_flags_lo
    }
    #[doc = "0x44 - Feature flags, high 32 bits"]
    #[inline(always)]
    pub const fn feature_flags_hi(&self) -> &FEATURE_FLAGS_HI {
        &self.feature_flags_hi
    }
    #[doc = "0x48 - Pointer size"]
    #[inline(always)]
    pub const fn ptr_size(&self) -> &PTR_SIZE {
        &self.ptr_size
    }
    #[doc = "0x4c - Debug feature support (0x1)"]
    #[inline(always)]
    pub const fn debug_feature_support(&self) -> &DEBUG_FEATURE_SUPPORT {
        &self.debug_feature_support
    }
    #[doc = "0x50 - Breakpoint count (0x2)"]
    #[inline(always)]
    pub const fn bp_count(&self) -> &BP_COUNT {
        &self.bp_count
    }
}
#[doc = "device_class (r) register accessor: Device class (0x774)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_class::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_class`]
module"]
#[doc(alias = "device_class")]
pub type DEVICE_CLASS = crate::Reg<device_class::DEVICE_CLASS_SPEC>;
#[doc = "Device class (0x774)"]
pub mod device_class;
#[doc = "interface_version (r) register accessor: Interface version (0x3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_version`]
module"]
#[doc(alias = "interface_version")]
pub type INTERFACE_VERSION = crate::Reg<interface_version::INTERFACE_VERSION_SPEC>;
#[doc = "Interface version (0x3)"]
pub mod interface_version;
#[doc = "core_count (r) register accessor: Core count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_count`]
module"]
#[doc(alias = "core_count")]
pub type CORE_COUNT = crate::Reg<core_count::CORE_COUNT_SPEC>;
#[doc = "Core count"]
pub mod core_count;
#[doc = "ctrl_size (r) register accessor: CTRL size, per core, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_size`]
module"]
#[doc(alias = "ctrl_size")]
pub type CTRL_SIZE = crate::Reg<ctrl_size::CTRL_SIZE_SPEC>;
#[doc = "CTRL size, per core, in bytes"]
pub mod ctrl_size;
#[doc = "ptr_size (r) register accessor: Pointer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr_size`]
module"]
#[doc(alias = "ptr_size")]
pub type PTR_SIZE = crate::Reg<ptr_size::PTR_SIZE_SPEC>;
#[doc = "Pointer size"]
pub mod ptr_size;
#[doc = "feature_flags_hi (r) register accessor: Feature flags, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_flags_hi`]
module"]
#[doc(alias = "feature_flags_hi")]
pub type FEATURE_FLAGS_HI = crate::Reg<feature_flags_hi::FEATURE_FLAGS_HI_SPEC>;
#[doc = "Feature flags, high 32 bits"]
pub mod feature_flags_hi;
#[doc = "feature_flags_lo (r) register accessor: Feature flags, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_flags_lo`]
module"]
#[doc(alias = "feature_flags_lo")]
pub type FEATURE_FLAGS_LO = crate::Reg<feature_flags_lo::FEATURE_FLAGS_LO_SPEC>;
#[doc = "Feature flags, low 32 bits"]
pub mod feature_flags_lo;
#[doc = "data_mem_start_hi (r) register accessor: Data memory start, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_start_hi`]
module"]
#[doc(alias = "data_mem_start_hi")]
pub type DATA_MEM_START_HI = crate::Reg<data_mem_start_hi::DATA_MEM_START_HI_SPEC>;
#[doc = "Data memory start, high 32 bits"]
pub mod data_mem_start_hi;
#[doc = "data_mem_start_lo (r) register accessor: Data memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_start_lo`]
module"]
#[doc(alias = "data_mem_start_lo")]
pub type DATA_MEM_START_LO = crate::Reg<data_mem_start_lo::DATA_MEM_START_LO_SPEC>;
#[doc = "Data memory start, low 32 bits"]
pub mod data_mem_start_lo;
#[doc = "data_mem_size_hi (r) register accessor: Data memory size in bytes, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_size_hi`]
module"]
#[doc(alias = "data_mem_size_hi")]
pub type DATA_MEM_SIZE_HI = crate::Reg<data_mem_size_hi::DATA_MEM_SIZE_HI_SPEC>;
#[doc = "Data memory size in bytes, high 32 bits"]
pub mod data_mem_size_hi;
#[doc = "data_mem_size_lo (r) register accessor: Data memory size in bytes, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_size_lo`]
module"]
#[doc(alias = "data_mem_size_lo")]
pub type DATA_MEM_SIZE_LO = crate::Reg<data_mem_size_lo::DATA_MEM_SIZE_LO_SPEC>;
#[doc = "Data memory size in bytes, low 32 bits"]
pub mod data_mem_size_lo;
#[doc = "cq_mem_start_hi (r) register accessor: CQ memory start, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_start_hi`]
module"]
#[doc(alias = "cq_mem_start_hi")]
pub type CQ_MEM_START_HI = crate::Reg<cq_mem_start_hi::CQ_MEM_START_HI_SPEC>;
#[doc = "CQ memory start, high 32 bits"]
pub mod cq_mem_start_hi;
#[doc = "cq_mem_start_lo (r) register accessor: CQ memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_start_lo`]
module"]
#[doc(alias = "cq_mem_start_lo")]
pub type CQ_MEM_START_LO = crate::Reg<cq_mem_start_lo::CQ_MEM_START_LO_SPEC>;
#[doc = "CQ memory start, low 32 bits"]
pub mod cq_mem_start_lo;
#[doc = "cq_mem_size_hi (r) register accessor: CQ memory size, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_size_hi`]
module"]
#[doc(alias = "cq_mem_size_hi")]
pub type CQ_MEM_SIZE_HI = crate::Reg<cq_mem_size_hi::CQ_MEM_SIZE_HI_SPEC>;
#[doc = "CQ memory size, high 32 bits"]
pub mod cq_mem_size_hi;
#[doc = "cq_mem_size_lo (r) register accessor: CQ memory size, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_size_lo`]
module"]
#[doc(alias = "cq_mem_size_lo")]
pub type CQ_MEM_SIZE_LO = crate::Reg<cq_mem_size_lo::CQ_MEM_SIZE_LO_SPEC>;
#[doc = "CQ memory size, low 32 bits"]
pub mod cq_mem_size_lo;
#[doc = "instr_mem_start_hi (r) register accessor: Start of instruction memory, high 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_start_hi`]
module"]
#[doc(alias = "instr_mem_start_hi")]
pub type INSTR_MEM_START_HI = crate::Reg<instr_mem_start_hi::INSTR_MEM_START_HI_SPEC>;
#[doc = "Start of instruction memory, high 32-bits"]
pub mod instr_mem_start_hi;
#[doc = "instr_mem_start_lo (r) register accessor: Start of instruction memory, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_start_lo`]
module"]
#[doc(alias = "instr_mem_start_lo")]
pub type INSTR_MEM_START_LO = crate::Reg<instr_mem_start_lo::INSTR_MEM_START_LO_SPEC>;
#[doc = "Start of instruction memory, low 32 bits"]
pub mod instr_mem_start_lo;
#[doc = "instr_mem_size (r) register accessor: Instruction memory size, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_size`]
module"]
#[doc(alias = "instr_mem_size")]
pub type INSTR_MEM_SIZE = crate::Reg<instr_mem_size::INSTR_MEM_SIZE_SPEC>;
#[doc = "Instruction memory size, in bytes"]
pub mod instr_mem_size;
#[doc = "bp_count (r) register accessor: Breakpoint count (0x2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_count`]
module"]
#[doc(alias = "bp_count")]
pub type BP_COUNT = crate::Reg<bp_count::BP_COUNT_SPEC>;
#[doc = "Breakpoint count (0x2)"]
pub mod bp_count;
#[doc = "debug_feature_support (r) register accessor: Debug feature support (0x1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_feature_support::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_feature_support`]
module"]
#[doc(alias = "debug_feature_support")]
pub type DEBUG_FEATURE_SUPPORT = crate::Reg<debug_feature_support::DEBUG_FEATURE_SUPPORT_SPEC>;
#[doc = "Debug feature support (0x1)"]
pub mod debug_feature_support;
