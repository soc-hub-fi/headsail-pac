#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "processor_info"]
#[doc(alias = "processor_info")]
pub struct ProcessorInfo {
    device_class: DeviceClass,
    _reserved1: [u8; 0x04],
    interface_version: InterfaceVersion,
    core_count: CoreCount,
    ctrl_size: CtrlSize,
    instr_mem_size: InstrMemSize,
    instr_mem_start_lo: InstrMemStartLo,
    instr_mem_start_hi: InstrMemStartHi,
    cq_mem_size_lo: CqMemSizeLo,
    cq_mem_size_hi: CqMemSizeHi,
    cq_mem_start_lo: CqMemStartLo,
    cq_mem_start_hi: CqMemStartHi,
    data_mem_size_lo: DataMemSizeLo,
    data_mem_size_hi: DataMemSizeHi,
    data_mem_start_lo: DataMemStartLo,
    data_mem_start_hi: DataMemStartHi,
    feature_flags_lo: FeatureFlagsLo,
    feature_flags_hi: FeatureFlagsHi,
    ptr_size: PtrSize,
    debug_feature_support: DebugFeatureSupport,
    bp_count: BpCount,
}
impl ProcessorInfo {
    #[doc = "0x00 - Device class (0x774)"]
    #[inline(always)]
    pub const fn device_class(&self) -> &DeviceClass {
        &self.device_class
    }
    #[doc = "0x08 - Interface version (0x3)"]
    #[inline(always)]
    pub const fn interface_version(&self) -> &InterfaceVersion {
        &self.interface_version
    }
    #[doc = "0x0c - Core count"]
    #[inline(always)]
    pub const fn core_count(&self) -> &CoreCount {
        &self.core_count
    }
    #[doc = "0x10 - CTRL size, per core, in bytes"]
    #[inline(always)]
    pub const fn ctrl_size(&self) -> &CtrlSize {
        &self.ctrl_size
    }
    #[doc = "0x14 - Instruction memory size, in bytes"]
    #[inline(always)]
    pub const fn instr_mem_size(&self) -> &InstrMemSize {
        &self.instr_mem_size
    }
    #[doc = "0x18 - Start of instruction memory, low 32 bits"]
    #[inline(always)]
    pub const fn instr_mem_start_lo(&self) -> &InstrMemStartLo {
        &self.instr_mem_start_lo
    }
    #[doc = "0x1c - Start of instruction memory, high 32-bits"]
    #[inline(always)]
    pub const fn instr_mem_start_hi(&self) -> &InstrMemStartHi {
        &self.instr_mem_start_hi
    }
    #[doc = "0x20 - CQ memory size, low 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_size_lo(&self) -> &CqMemSizeLo {
        &self.cq_mem_size_lo
    }
    #[doc = "0x24 - CQ memory size, high 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_size_hi(&self) -> &CqMemSizeHi {
        &self.cq_mem_size_hi
    }
    #[doc = "0x28 - CQ memory start, low 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_start_lo(&self) -> &CqMemStartLo {
        &self.cq_mem_start_lo
    }
    #[doc = "0x2c - CQ memory start, high 32 bits"]
    #[inline(always)]
    pub const fn cq_mem_start_hi(&self) -> &CqMemStartHi {
        &self.cq_mem_start_hi
    }
    #[doc = "0x30 - Data memory size in bytes, low 32 bits"]
    #[inline(always)]
    pub const fn data_mem_size_lo(&self) -> &DataMemSizeLo {
        &self.data_mem_size_lo
    }
    #[doc = "0x34 - Data memory size in bytes, high 32 bits"]
    #[inline(always)]
    pub const fn data_mem_size_hi(&self) -> &DataMemSizeHi {
        &self.data_mem_size_hi
    }
    #[doc = "0x38 - Data memory start, low 32 bits"]
    #[inline(always)]
    pub const fn data_mem_start_lo(&self) -> &DataMemStartLo {
        &self.data_mem_start_lo
    }
    #[doc = "0x3c - Data memory start, high 32 bits"]
    #[inline(always)]
    pub const fn data_mem_start_hi(&self) -> &DataMemStartHi {
        &self.data_mem_start_hi
    }
    #[doc = "0x40 - Feature flags, low 32 bits"]
    #[inline(always)]
    pub const fn feature_flags_lo(&self) -> &FeatureFlagsLo {
        &self.feature_flags_lo
    }
    #[doc = "0x44 - Feature flags, high 32 bits"]
    #[inline(always)]
    pub const fn feature_flags_hi(&self) -> &FeatureFlagsHi {
        &self.feature_flags_hi
    }
    #[doc = "0x48 - Pointer size"]
    #[inline(always)]
    pub const fn ptr_size(&self) -> &PtrSize {
        &self.ptr_size
    }
    #[doc = "0x4c - Debug feature support (0x1)"]
    #[inline(always)]
    pub const fn debug_feature_support(&self) -> &DebugFeatureSupport {
        &self.debug_feature_support
    }
    #[doc = "0x50 - Breakpoint count (0x2)"]
    #[inline(always)]
    pub const fn bp_count(&self) -> &BpCount {
        &self.bp_count
    }
}
#[doc = "device_class (r) register accessor: Device class (0x774)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_class::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_class`]
module"]
#[doc(alias = "device_class")]
pub type DeviceClass = crate::Reg<device_class::DeviceClassSpec>;
#[doc = "Device class (0x774)"]
pub mod device_class;
#[doc = "interface_version (r) register accessor: Interface version (0x3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_version`]
module"]
#[doc(alias = "interface_version")]
pub type InterfaceVersion = crate::Reg<interface_version::InterfaceVersionSpec>;
#[doc = "Interface version (0x3)"]
pub mod interface_version;
#[doc = "core_count (r) register accessor: Core count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_count`]
module"]
#[doc(alias = "core_count")]
pub type CoreCount = crate::Reg<core_count::CoreCountSpec>;
#[doc = "Core count"]
pub mod core_count;
#[doc = "ctrl_size (r) register accessor: CTRL size, per core, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_size`]
module"]
#[doc(alias = "ctrl_size")]
pub type CtrlSize = crate::Reg<ctrl_size::CtrlSizeSpec>;
#[doc = "CTRL size, per core, in bytes"]
pub mod ctrl_size;
#[doc = "ptr_size (r) register accessor: Pointer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr_size`]
module"]
#[doc(alias = "ptr_size")]
pub type PtrSize = crate::Reg<ptr_size::PtrSizeSpec>;
#[doc = "Pointer size"]
pub mod ptr_size;
#[doc = "feature_flags_hi (r) register accessor: Feature flags, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_flags_hi`]
module"]
#[doc(alias = "feature_flags_hi")]
pub type FeatureFlagsHi = crate::Reg<feature_flags_hi::FeatureFlagsHiSpec>;
#[doc = "Feature flags, high 32 bits"]
pub mod feature_flags_hi;
#[doc = "feature_flags_lo (r) register accessor: Feature flags, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feature_flags_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature_flags_lo`]
module"]
#[doc(alias = "feature_flags_lo")]
pub type FeatureFlagsLo = crate::Reg<feature_flags_lo::FeatureFlagsLoSpec>;
#[doc = "Feature flags, low 32 bits"]
pub mod feature_flags_lo;
#[doc = "data_mem_start_hi (r) register accessor: Data memory start, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_start_hi`]
module"]
#[doc(alias = "data_mem_start_hi")]
pub type DataMemStartHi = crate::Reg<data_mem_start_hi::DataMemStartHiSpec>;
#[doc = "Data memory start, high 32 bits"]
pub mod data_mem_start_hi;
#[doc = "data_mem_start_lo (r) register accessor: Data memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_start_lo`]
module"]
#[doc(alias = "data_mem_start_lo")]
pub type DataMemStartLo = crate::Reg<data_mem_start_lo::DataMemStartLoSpec>;
#[doc = "Data memory start, low 32 bits"]
pub mod data_mem_start_lo;
#[doc = "data_mem_size_hi (r) register accessor: Data memory size in bytes, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_size_hi`]
module"]
#[doc(alias = "data_mem_size_hi")]
pub type DataMemSizeHi = crate::Reg<data_mem_size_hi::DataMemSizeHiSpec>;
#[doc = "Data memory size in bytes, high 32 bits"]
pub mod data_mem_size_hi;
#[doc = "data_mem_size_lo (r) register accessor: Data memory size in bytes, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mem_size_lo`]
module"]
#[doc(alias = "data_mem_size_lo")]
pub type DataMemSizeLo = crate::Reg<data_mem_size_lo::DataMemSizeLoSpec>;
#[doc = "Data memory size in bytes, low 32 bits"]
pub mod data_mem_size_lo;
#[doc = "cq_mem_start_hi (r) register accessor: CQ memory start, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_start_hi`]
module"]
#[doc(alias = "cq_mem_start_hi")]
pub type CqMemStartHi = crate::Reg<cq_mem_start_hi::CqMemStartHiSpec>;
#[doc = "CQ memory start, high 32 bits"]
pub mod cq_mem_start_hi;
#[doc = "cq_mem_start_lo (r) register accessor: CQ memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_start_lo`]
module"]
#[doc(alias = "cq_mem_start_lo")]
pub type CqMemStartLo = crate::Reg<cq_mem_start_lo::CqMemStartLoSpec>;
#[doc = "CQ memory start, low 32 bits"]
pub mod cq_mem_start_lo;
#[doc = "cq_mem_size_hi (r) register accessor: CQ memory size, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_size_hi`]
module"]
#[doc(alias = "cq_mem_size_hi")]
pub type CqMemSizeHi = crate::Reg<cq_mem_size_hi::CqMemSizeHiSpec>;
#[doc = "CQ memory size, high 32 bits"]
pub mod cq_mem_size_hi;
#[doc = "cq_mem_size_lo (r) register accessor: CQ memory size, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cq_mem_size_lo`]
module"]
#[doc(alias = "cq_mem_size_lo")]
pub type CqMemSizeLo = crate::Reg<cq_mem_size_lo::CqMemSizeLoSpec>;
#[doc = "CQ memory size, low 32 bits"]
pub mod cq_mem_size_lo;
#[doc = "instr_mem_start_hi (r) register accessor: Start of instruction memory, high 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_start_hi`]
module"]
#[doc(alias = "instr_mem_start_hi")]
pub type InstrMemStartHi = crate::Reg<instr_mem_start_hi::InstrMemStartHiSpec>;
#[doc = "Start of instruction memory, high 32-bits"]
pub mod instr_mem_start_hi;
#[doc = "instr_mem_start_lo (r) register accessor: Start of instruction memory, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_start_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_start_lo`]
module"]
#[doc(alias = "instr_mem_start_lo")]
pub type InstrMemStartLo = crate::Reg<instr_mem_start_lo::InstrMemStartLoSpec>;
#[doc = "Start of instruction memory, low 32 bits"]
pub mod instr_mem_start_lo;
#[doc = "instr_mem_size (r) register accessor: Instruction memory size, in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`instr_mem_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_mem_size`]
module"]
#[doc(alias = "instr_mem_size")]
pub type InstrMemSize = crate::Reg<instr_mem_size::InstrMemSizeSpec>;
#[doc = "Instruction memory size, in bytes"]
pub mod instr_mem_size;
#[doc = "bp_count (r) register accessor: Breakpoint count (0x2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_count`]
module"]
#[doc(alias = "bp_count")]
pub type BpCount = crate::Reg<bp_count::BpCountSpec>;
#[doc = "Breakpoint count (0x2)"]
pub mod bp_count;
#[doc = "debug_feature_support (r) register accessor: Debug feature support (0x1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_feature_support::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_feature_support`]
module"]
#[doc(alias = "debug_feature_support")]
pub type DebugFeatureSupport = crate::Reg<debug_feature_support::DebugFeatureSupportSpec>;
#[doc = "Debug feature support (0x1)"]
pub mod debug_feature_support;
