#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL3 {
    #[doc = "0x00 - Start address for read"]
    pub source_addr: SOURCE_ADDR,
    #[doc = "0x04 - Length of read transfer in bytes"]
    pub source_length: SOURCE_LENGTH,
    #[doc = "0x08 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    pub source_mode: SOURCE_MODE,
    #[doc = "0x0c - Write to set source mode register"]
    pub source_mode_set: SOURCE_MODE_SET,
    #[doc = "0x10 - Write to unset source mode register"]
    pub source_mode_unset: SOURCE_MODE_UNSET,
    #[doc = "0x14 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    pub axi_read_config: AXI_READ_CONFIG,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Start address for write"]
    pub destination_addr: DESTINATION_ADDR,
    #[doc = "0x20 - Length of write transfer in bytes"]
    pub destination_length: DESTINATION_LENGTH,
    #[doc = "0x24 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    pub destination_mode: DESTINATION_MODE,
    #[doc = "0x28 - Write to set destination mode register"]
    pub destination_mode_set: DESTINATION_MODE_SET,
    #[doc = "0x2c - Write to unset destination mode register"]
    pub destination_mode_unset: DESTINATION_MODE_UNSET,
    #[doc = "0x30 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    pub axi_write_config: AXI_WRITE_CONFIG,
    _reserved12: [u8; 0x0c],
    #[doc = "0x40 - Start address for read"]
    pub d_source_addr: D_SOURCE_ADDR,
    #[doc = "0x44 - Length of read transfer in bytes"]
    pub d_source_length: D_SOURCE_LENGTH,
    #[doc = "0x48 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    pub d_source_mode: D_SOURCE_MODE,
    #[doc = "0x4c - Write to set source mode register"]
    pub d_source_mode_set: D_SOURCE_MODE_SET,
    #[doc = "0x50 - Write to unset source mode register"]
    pub d_source_mode_unset: D_SOURCE_MODE_UNSET,
    #[doc = "0x54 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    pub d_source_axi_config: D_SOURCE_AXI_CONFIG,
    _reserved18: [u8; 0x04],
    #[doc = "0x5c - Start address for write"]
    pub d_destination_addr: D_DESTINATION_ADDR,
    #[doc = "0x60 - Length of write transfer in bytes"]
    pub d_destination_length: D_DESTINATION_LENGTH,
    #[doc = "0x64 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    pub d_destination_mode: D_DESTINATION_MODE,
    #[doc = "0x68 - Write to set destination mode register"]
    pub d_destination_mode_set: D_DESTINATION_MODE_SET,
    #[doc = "0x6c - Write to unset destination mode register"]
    pub d_destination_mode_unset: D_DESTINATION_MODE_UNSET,
    #[doc = "0x70 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    pub d_destination_axi_config: D_DESTINATION_AXI_CONFIG,
}
#[doc = "SOURCE_ADDR (rw) register accessor: an alias for `Reg<SOURCE_ADDR_SPEC>`"]
pub type SOURCE_ADDR = crate::Reg<source_addr::SOURCE_ADDR_SPEC>;
#[doc = "Start address for read"]
pub mod source_addr;
#[doc = "SOURCE_LENGTH (rw) register accessor: an alias for `Reg<SOURCE_LENGTH_SPEC>`"]
pub type SOURCE_LENGTH = crate::Reg<source_length::SOURCE_LENGTH_SPEC>;
#[doc = "Length of read transfer in bytes"]
pub mod source_length;
#[doc = "SOURCE_MODE (rw) register accessor: an alias for `Reg<SOURCE_MODE_SPEC>`"]
pub type SOURCE_MODE = crate::Reg<source_mode::SOURCE_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod source_mode;
#[doc = "SOURCE_MODE_SET (w) register accessor: an alias for `Reg<SOURCE_MODE_SET_SPEC>`"]
pub type SOURCE_MODE_SET = crate::Reg<source_mode_set::SOURCE_MODE_SET_SPEC>;
#[doc = "Write to set source mode register"]
pub mod source_mode_set;
#[doc = "SOURCE_MODE_UNSET (w) register accessor: an alias for `Reg<SOURCE_MODE_UNSET_SPEC>`"]
pub type SOURCE_MODE_UNSET = crate::Reg<source_mode_unset::SOURCE_MODE_UNSET_SPEC>;
#[doc = "Write to unset source mode register"]
pub mod source_mode_unset;
#[doc = "AXI_READ_CONFIG (rw) register accessor: an alias for `Reg<AXI_READ_CONFIG_SPEC>`"]
pub type AXI_READ_CONFIG = crate::Reg<axi_read_config::AXI_READ_CONFIG_SPEC>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_read_config;
#[doc = "DESTINATION_ADDR (rw) register accessor: an alias for `Reg<DESTINATION_ADDR_SPEC>`"]
pub type DESTINATION_ADDR = crate::Reg<destination_addr::DESTINATION_ADDR_SPEC>;
#[doc = "Start address for write"]
pub mod destination_addr;
#[doc = "D_SOURCE_ADDR (rw) register accessor: an alias for `Reg<D_SOURCE_ADDR_SPEC>`"]
pub type D_SOURCE_ADDR = crate::Reg<d_source_addr::D_SOURCE_ADDR_SPEC>;
#[doc = "Start address for read"]
pub mod d_source_addr;
#[doc = "AXI_WRITE_CONFIG (rw) register accessor: an alias for `Reg<AXI_WRITE_CONFIG_SPEC>`"]
pub type AXI_WRITE_CONFIG = crate::Reg<axi_write_config::AXI_WRITE_CONFIG_SPEC>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_write_config;
#[doc = "DESTINATION_MODE_UNSET (w) register accessor: an alias for `Reg<DESTINATION_MODE_UNSET_SPEC>`"]
pub type DESTINATION_MODE_UNSET = crate::Reg<destination_mode_unset::DESTINATION_MODE_UNSET_SPEC>;
#[doc = "Write to unset destination mode register"]
pub mod destination_mode_unset;
#[doc = "DESTINATION_MODE_SET (w) register accessor: an alias for `Reg<DESTINATION_MODE_SET_SPEC>`"]
pub type DESTINATION_MODE_SET = crate::Reg<destination_mode_set::DESTINATION_MODE_SET_SPEC>;
#[doc = "Write to set destination mode register"]
pub mod destination_mode_set;
#[doc = "DESTINATION_MODE (rw) register accessor: an alias for `Reg<DESTINATION_MODE_SPEC>`"]
pub type DESTINATION_MODE = crate::Reg<destination_mode::DESTINATION_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod destination_mode;
#[doc = "DESTINATION_LENGTH (rw) register accessor: an alias for `Reg<DESTINATION_LENGTH_SPEC>`"]
pub type DESTINATION_LENGTH = crate::Reg<destination_length::DESTINATION_LENGTH_SPEC>;
#[doc = "Length of write transfer in bytes"]
pub mod destination_length;
#[doc = "D_SOURCE_LENGTH (rw) register accessor: an alias for `Reg<D_SOURCE_LENGTH_SPEC>`"]
pub type D_SOURCE_LENGTH = crate::Reg<d_source_length::D_SOURCE_LENGTH_SPEC>;
#[doc = "Length of read transfer in bytes"]
pub mod d_source_length;
#[doc = "D_SOURCE_MODE (rw) register accessor: an alias for `Reg<D_SOURCE_MODE_SPEC>`"]
pub type D_SOURCE_MODE = crate::Reg<d_source_mode::D_SOURCE_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_source_mode;
#[doc = "D_SOURCE_MODE_SET (w) register accessor: an alias for `Reg<D_SOURCE_MODE_SET_SPEC>`"]
pub type D_SOURCE_MODE_SET = crate::Reg<d_source_mode_set::D_SOURCE_MODE_SET_SPEC>;
#[doc = "Write to set source mode register"]
pub mod d_source_mode_set;
#[doc = "D_SOURCE_MODE_UNSET (w) register accessor: an alias for `Reg<D_SOURCE_MODE_UNSET_SPEC>`"]
pub type D_SOURCE_MODE_UNSET = crate::Reg<d_source_mode_unset::D_SOURCE_MODE_UNSET_SPEC>;
#[doc = "Write to unset source mode register"]
pub mod d_source_mode_unset;
#[doc = "D_SOURCE_AXI_CONFIG (rw) register accessor: an alias for `Reg<D_SOURCE_AXI_CONFIG_SPEC>`"]
pub type D_SOURCE_AXI_CONFIG = crate::Reg<d_source_axi_config::D_SOURCE_AXI_CONFIG_SPEC>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_source_axi_config;
#[doc = "D_DESTINATION_ADDR (rw) register accessor: an alias for `Reg<D_DESTINATION_ADDR_SPEC>`"]
pub type D_DESTINATION_ADDR = crate::Reg<d_destination_addr::D_DESTINATION_ADDR_SPEC>;
#[doc = "Start address for write"]
pub mod d_destination_addr;
#[doc = "D_DESTINATION_LENGTH (rw) register accessor: an alias for `Reg<D_DESTINATION_LENGTH_SPEC>`"]
pub type D_DESTINATION_LENGTH = crate::Reg<d_destination_length::D_DESTINATION_LENGTH_SPEC>;
#[doc = "Length of write transfer in bytes"]
pub mod d_destination_length;
#[doc = "D_DESTINATION_MODE (rw) register accessor: an alias for `Reg<D_DESTINATION_MODE_SPEC>`"]
pub type D_DESTINATION_MODE = crate::Reg<d_destination_mode::D_DESTINATION_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_destination_mode;
#[doc = "D_DESTINATION_MODE_SET (w) register accessor: an alias for `Reg<D_DESTINATION_MODE_SET_SPEC>`"]
pub type D_DESTINATION_MODE_SET = crate::Reg<d_destination_mode_set::D_DESTINATION_MODE_SET_SPEC>;
#[doc = "Write to set destination mode register"]
pub mod d_destination_mode_set;
#[doc = "D_DESTINATION_MODE_UNSET (w) register accessor: an alias for `Reg<D_DESTINATION_MODE_UNSET_SPEC>`"]
pub type D_DESTINATION_MODE_UNSET =
    crate::Reg<d_destination_mode_unset::D_DESTINATION_MODE_UNSET_SPEC>;
#[doc = "Write to unset destination mode register"]
pub mod d_destination_mode_unset;
#[doc = "D_DESTINATION_AXI_CONFIG (rw) register accessor: an alias for `Reg<D_DESTINATION_AXI_CONFIG_SPEC>`"]
pub type D_DESTINATION_AXI_CONFIG =
    crate::Reg<d_destination_axi_config::D_DESTINATION_AXI_CONFIG_SPEC>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_destination_axi_config;
