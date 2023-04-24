#[doc = r"Register block"]
#[repr(C)]
pub struct PROCESSOR_INFO {
    #[doc = "0x00 - Device class (0x774)"]
    pub device_class: DEVICE_CLASS,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interface version (0x3)"]
    pub interface_version: INTERFACE_VERSION,
    #[doc = "0x0c - Core count"]
    pub core_count: CORE_COUNT,
    #[doc = "0x10 - CTRL size, per core, in bytes"]
    pub ctrl_size: CTRL_SIZE,
    #[doc = "0x14 - Instruction memory size, in bytes"]
    pub instr_mem_size: INSTR_MEM_SIZE,
    #[doc = "0x18 - Start of instruction memory, low 32 bits"]
    pub instr_mem_start_lo: INSTR_MEM_START_LO,
    #[doc = "0x1c - Start of instruction memory, high 32-bits"]
    pub instr_mem_start_hi: INSTR_MEM_START_HI,
    #[doc = "0x20 - CQ memory size, low 32 bits"]
    pub cq_mem_size_lo: CQ_MEM_SIZE_LO,
    #[doc = "0x24 - CQ memory size, high 32 bits"]
    pub cq_mem_size_hi: CQ_MEM_SIZE_HI,
    #[doc = "0x28 - CQ memory start, low 32 bits"]
    pub cq_mem_start_lo: CQ_MEM_START_LO,
    #[doc = "0x2c - CQ memory start, high 32 bits"]
    pub cq_mem_start_hi: CQ_MEM_START_HI,
    #[doc = "0x30 - Data memory size in bytes, low 32 bits"]
    pub data_mem_size_lo: DATA_MEM_SIZE_LO,
    #[doc = "0x34 - Data memory size in bytes, high 32 bits"]
    pub data_mem_size_hi: DATA_MEM_SIZE_HI,
    #[doc = "0x38 - Data memory start, low 32 bits"]
    pub data_mem_start_lo: DATA_MEM_START_LO,
    #[doc = "0x3c - Data memory start, high 32 bits"]
    pub data_mem_start_hi: DATA_MEM_START_HI,
    #[doc = "0x40 - Feature flags, low 32 bits"]
    pub feature_flags_lo: FEATURE_FLAGS_LO,
    #[doc = "0x44 - Feature flags, high 32 bits"]
    pub feature_flags_hi: FEATURE_FLAGS_HI,
    #[doc = "0x48 - Pointer size"]
    pub ptr_size: PTR_SIZE,
    #[doc = "0x4c - Debug feature support (0x1)"]
    pub debug_feature_support: DEBUG_FEATURE_SUPPORT,
    #[doc = "0x50 - Breakpoint count (0x2)"]
    pub bp_count: BP_COUNT,
}
#[doc = "device_class (r) register accessor: an alias for `Reg<DEVICE_CLASS_SPEC>`"]
pub type DEVICE_CLASS = crate::Reg<device_class::DEVICE_CLASS_SPEC>;
#[doc = "Device class (0x774)"]
pub mod device_class;
#[doc = "interface_version (r) register accessor: an alias for `Reg<INTERFACE_VERSION_SPEC>`"]
pub type INTERFACE_VERSION = crate::Reg<interface_version::INTERFACE_VERSION_SPEC>;
#[doc = "Interface version (0x3)"]
pub mod interface_version;
#[doc = "core_count (r) register accessor: an alias for `Reg<CORE_COUNT_SPEC>`"]
pub type CORE_COUNT = crate::Reg<core_count::CORE_COUNT_SPEC>;
#[doc = "Core count"]
pub mod core_count;
#[doc = "ctrl_size (r) register accessor: an alias for `Reg<CTRL_SIZE_SPEC>`"]
pub type CTRL_SIZE = crate::Reg<ctrl_size::CTRL_SIZE_SPEC>;
#[doc = "CTRL size, per core, in bytes"]
pub mod ctrl_size;
#[doc = "ptr_size (r) register accessor: an alias for `Reg<PTR_SIZE_SPEC>`"]
pub type PTR_SIZE = crate::Reg<ptr_size::PTR_SIZE_SPEC>;
#[doc = "Pointer size"]
pub mod ptr_size;
#[doc = "feature_flags_hi (r) register accessor: an alias for `Reg<FEATURE_FLAGS_HI_SPEC>`"]
pub type FEATURE_FLAGS_HI = crate::Reg<feature_flags_hi::FEATURE_FLAGS_HI_SPEC>;
#[doc = "Feature flags, high 32 bits"]
pub mod feature_flags_hi;
#[doc = "feature_flags_lo (r) register accessor: an alias for `Reg<FEATURE_FLAGS_LO_SPEC>`"]
pub type FEATURE_FLAGS_LO = crate::Reg<feature_flags_lo::FEATURE_FLAGS_LO_SPEC>;
#[doc = "Feature flags, low 32 bits"]
pub mod feature_flags_lo;
#[doc = "data_mem_start_hi (r) register accessor: an alias for `Reg<DATA_MEM_START_HI_SPEC>`"]
pub type DATA_MEM_START_HI = crate::Reg<data_mem_start_hi::DATA_MEM_START_HI_SPEC>;
#[doc = "Data memory start, high 32 bits"]
pub mod data_mem_start_hi;
#[doc = "data_mem_start_lo (r) register accessor: an alias for `Reg<DATA_MEM_START_LO_SPEC>`"]
pub type DATA_MEM_START_LO = crate::Reg<data_mem_start_lo::DATA_MEM_START_LO_SPEC>;
#[doc = "Data memory start, low 32 bits"]
pub mod data_mem_start_lo;
#[doc = "data_mem_size_hi (r) register accessor: an alias for `Reg<DATA_MEM_SIZE_HI_SPEC>`"]
pub type DATA_MEM_SIZE_HI = crate::Reg<data_mem_size_hi::DATA_MEM_SIZE_HI_SPEC>;
#[doc = "Data memory size in bytes, high 32 bits"]
pub mod data_mem_size_hi;
#[doc = "data_mem_size_lo (r) register accessor: an alias for `Reg<DATA_MEM_SIZE_LO_SPEC>`"]
pub type DATA_MEM_SIZE_LO = crate::Reg<data_mem_size_lo::DATA_MEM_SIZE_LO_SPEC>;
#[doc = "Data memory size in bytes, low 32 bits"]
pub mod data_mem_size_lo;
#[doc = "cq_mem_start_hi (r) register accessor: an alias for `Reg<CQ_MEM_START_HI_SPEC>`"]
pub type CQ_MEM_START_HI = crate::Reg<cq_mem_start_hi::CQ_MEM_START_HI_SPEC>;
#[doc = "CQ memory start, high 32 bits"]
pub mod cq_mem_start_hi;
#[doc = "cq_mem_start_lo (r) register accessor: an alias for `Reg<CQ_MEM_START_LO_SPEC>`"]
pub type CQ_MEM_START_LO = crate::Reg<cq_mem_start_lo::CQ_MEM_START_LO_SPEC>;
#[doc = "CQ memory start, low 32 bits"]
pub mod cq_mem_start_lo;
#[doc = "cq_mem_size_hi (r) register accessor: an alias for `Reg<CQ_MEM_SIZE_HI_SPEC>`"]
pub type CQ_MEM_SIZE_HI = crate::Reg<cq_mem_size_hi::CQ_MEM_SIZE_HI_SPEC>;
#[doc = "CQ memory size, high 32 bits"]
pub mod cq_mem_size_hi;
#[doc = "cq_mem_size_lo (r) register accessor: an alias for `Reg<CQ_MEM_SIZE_LO_SPEC>`"]
pub type CQ_MEM_SIZE_LO = crate::Reg<cq_mem_size_lo::CQ_MEM_SIZE_LO_SPEC>;
#[doc = "CQ memory size, low 32 bits"]
pub mod cq_mem_size_lo;
#[doc = "instr_mem_start_hi (r) register accessor: an alias for `Reg<INSTR_MEM_START_HI_SPEC>`"]
pub type INSTR_MEM_START_HI = crate::Reg<instr_mem_start_hi::INSTR_MEM_START_HI_SPEC>;
#[doc = "Start of instruction memory, high 32-bits"]
pub mod instr_mem_start_hi;
#[doc = "instr_mem_start_lo (r) register accessor: an alias for `Reg<INSTR_MEM_START_LO_SPEC>`"]
pub type INSTR_MEM_START_LO = crate::Reg<instr_mem_start_lo::INSTR_MEM_START_LO_SPEC>;
#[doc = "Start of instruction memory, low 32 bits"]
pub mod instr_mem_start_lo;
#[doc = "instr_mem_size (r) register accessor: an alias for `Reg<INSTR_MEM_SIZE_SPEC>`"]
pub type INSTR_MEM_SIZE = crate::Reg<instr_mem_size::INSTR_MEM_SIZE_SPEC>;
#[doc = "Instruction memory size, in bytes"]
pub mod instr_mem_size;
#[doc = "bp_count (r) register accessor: an alias for `Reg<BP_COUNT_SPEC>`"]
pub type BP_COUNT = crate::Reg<bp_count::BP_COUNT_SPEC>;
#[doc = "Breakpoint count (0x2)"]
pub mod bp_count;
#[doc = "debug_feature_support (r) register accessor: an alias for `Reg<DEBUG_FEATURE_SUPPORT_SPEC>`"]
pub type DEBUG_FEATURE_SUPPORT = crate::Reg<debug_feature_support::DEBUG_FEATURE_SUPPORT_SPEC>;
#[doc = "Debug feature support (0x1)"]
pub mod debug_feature_support;
