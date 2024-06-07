#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "channel2"]
#[doc(alias = "channel2")]
pub struct Channel2 {
    source_addr: SourceAddr,
    source_length: SourceLength,
    source_mode: SourceMode,
    source_mode_set: SourceModeSet,
    source_mode_unset: SourceModeUnset,
    axi_read_config: AxiReadConfig,
    _reserved6: [u8; 0x04],
    destination_addr: DestinationAddr,
    destination_length: DestinationLength,
    destination_mode: DestinationMode,
    destination_mode_set: DestinationModeSet,
    destination_mode_unset: DestinationModeUnset,
    axi_write_config: AxiWriteConfig,
    _reserved12: [u8; 0x0c],
    d_source_addr: DSourceAddr,
    d_source_length: DSourceLength,
    d_source_mode: DSourceMode,
    d_source_mode_set: DSourceModeSet,
    d_source_mode_unset: DSourceModeUnset,
    d_source_axi_config: DSourceAxiConfig,
    _reserved18: [u8; 0x04],
    d_destination_addr: DDestinationAddr,
    d_destination_length: DDestinationLength,
    d_destination_mode: DDestinationMode,
    d_destination_mode_set: DDestinationModeSet,
    d_destination_mode_unset: DDestinationModeUnset,
    d_destination_axi_config: DDestinationAxiConfig,
}
impl Channel2 {
    #[doc = "0x00 - Start address for read"]
    #[inline(always)]
    pub const fn source_addr(&self) -> &SourceAddr {
        &self.source_addr
    }
    #[doc = "0x04 - Length of read transfer in bytes"]
    #[inline(always)]
    pub const fn source_length(&self) -> &SourceLength {
        &self.source_length
    }
    #[doc = "0x08 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn source_mode(&self) -> &SourceMode {
        &self.source_mode
    }
    #[doc = "0x0c - Write to set source mode register"]
    #[inline(always)]
    pub const fn source_mode_set(&self) -> &SourceModeSet {
        &self.source_mode_set
    }
    #[doc = "0x10 - Write to unset source mode register"]
    #[inline(always)]
    pub const fn source_mode_unset(&self) -> &SourceModeUnset {
        &self.source_mode_unset
    }
    #[doc = "0x14 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn axi_read_config(&self) -> &AxiReadConfig {
        &self.axi_read_config
    }
    #[doc = "0x1c - Start address for write"]
    #[inline(always)]
    pub const fn destination_addr(&self) -> &DestinationAddr {
        &self.destination_addr
    }
    #[doc = "0x20 - Length of write transfer in bytes"]
    #[inline(always)]
    pub const fn destination_length(&self) -> &DestinationLength {
        &self.destination_length
    }
    #[doc = "0x24 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn destination_mode(&self) -> &DestinationMode {
        &self.destination_mode
    }
    #[doc = "0x28 - Write to set destination mode register"]
    #[inline(always)]
    pub const fn destination_mode_set(&self) -> &DestinationModeSet {
        &self.destination_mode_set
    }
    #[doc = "0x2c - Write to unset destination mode register"]
    #[inline(always)]
    pub const fn destination_mode_unset(&self) -> &DestinationModeUnset {
        &self.destination_mode_unset
    }
    #[doc = "0x30 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn axi_write_config(&self) -> &AxiWriteConfig {
        &self.axi_write_config
    }
    #[doc = "0x40 - Start address for read"]
    #[inline(always)]
    pub const fn d_source_addr(&self) -> &DSourceAddr {
        &self.d_source_addr
    }
    #[doc = "0x44 - Length of read transfer in bytes"]
    #[inline(always)]
    pub const fn d_source_length(&self) -> &DSourceLength {
        &self.d_source_length
    }
    #[doc = "0x48 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn d_source_mode(&self) -> &DSourceMode {
        &self.d_source_mode
    }
    #[doc = "0x4c - Write to set source mode register"]
    #[inline(always)]
    pub const fn d_source_mode_set(&self) -> &DSourceModeSet {
        &self.d_source_mode_set
    }
    #[doc = "0x50 - Write to unset source mode register"]
    #[inline(always)]
    pub const fn d_source_mode_unset(&self) -> &DSourceModeUnset {
        &self.d_source_mode_unset
    }
    #[doc = "0x54 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn d_source_axi_config(&self) -> &DSourceAxiConfig {
        &self.d_source_axi_config
    }
    #[doc = "0x5c - Start address for write"]
    #[inline(always)]
    pub const fn d_destination_addr(&self) -> &DDestinationAddr {
        &self.d_destination_addr
    }
    #[doc = "0x60 - Length of write transfer in bytes"]
    #[inline(always)]
    pub const fn d_destination_length(&self) -> &DDestinationLength {
        &self.d_destination_length
    }
    #[doc = "0x64 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn d_destination_mode(&self) -> &DDestinationMode {
        &self.d_destination_mode
    }
    #[doc = "0x68 - Write to set destination mode register"]
    #[inline(always)]
    pub const fn d_destination_mode_set(&self) -> &DDestinationModeSet {
        &self.d_destination_mode_set
    }
    #[doc = "0x6c - Write to unset destination mode register"]
    #[inline(always)]
    pub const fn d_destination_mode_unset(&self) -> &DDestinationModeUnset {
        &self.d_destination_mode_unset
    }
    #[doc = "0x70 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn d_destination_axi_config(&self) -> &DDestinationAxiConfig {
        &self.d_destination_axi_config
    }
}
#[doc = "SOURCE_ADDR (rw) register accessor: Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_addr`]
module"]
#[doc(alias = "SOURCE_ADDR")]
pub type SourceAddr = crate::Reg<source_addr::SourceAddrSpec>;
#[doc = "Start address for read"]
pub mod source_addr;
#[doc = "SOURCE_LENGTH (rw) register accessor: Length of read transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_length`]
module"]
#[doc(alias = "SOURCE_LENGTH")]
pub type SourceLength = crate::Reg<source_length::SourceLengthSpec>;
#[doc = "Length of read transfer in bytes"]
pub mod source_length;
#[doc = "SOURCE_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode`]
module"]
#[doc(alias = "SOURCE_MODE")]
pub type SourceMode = crate::Reg<source_mode::SourceModeSpec>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod source_mode;
#[doc = "SOURCE_MODE_SET (w) register accessor: Write to set source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode_set`]
module"]
#[doc(alias = "SOURCE_MODE_SET")]
pub type SourceModeSet = crate::Reg<source_mode_set::SourceModeSetSpec>;
#[doc = "Write to set source mode register"]
pub mod source_mode_set;
#[doc = "SOURCE_MODE_UNSET (w) register accessor: Write to unset source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode_unset`]
module"]
#[doc(alias = "SOURCE_MODE_UNSET")]
pub type SourceModeUnset = crate::Reg<source_mode_unset::SourceModeUnsetSpec>;
#[doc = "Write to unset source mode register"]
pub mod source_mode_unset;
#[doc = "AXI_READ_CONFIG (rw) register accessor: AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_read_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_read_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_read_config`]
module"]
#[doc(alias = "AXI_READ_CONFIG")]
pub type AxiReadConfig = crate::Reg<axi_read_config::AxiReadConfigSpec>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_read_config;
#[doc = "DESTINATION_ADDR (rw) register accessor: Start address for write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_addr`]
module"]
#[doc(alias = "DESTINATION_ADDR")]
pub type DestinationAddr = crate::Reg<destination_addr::DestinationAddrSpec>;
#[doc = "Start address for write"]
pub mod destination_addr;
#[doc = "D_SOURCE_ADDR (rw) register accessor: Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_addr`]
module"]
#[doc(alias = "D_SOURCE_ADDR")]
pub type DSourceAddr = crate::Reg<d_source_addr::DSourceAddrSpec>;
#[doc = "Start address for read"]
pub mod d_source_addr;
#[doc = "AXI_WRITE_CONFIG (rw) register accessor: AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_write_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_write_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_write_config`]
module"]
#[doc(alias = "AXI_WRITE_CONFIG")]
pub type AxiWriteConfig = crate::Reg<axi_write_config::AxiWriteConfigSpec>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_write_config;
#[doc = "DESTINATION_MODE_UNSET (w) register accessor: Write to unset destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode_unset`]
module"]
#[doc(alias = "DESTINATION_MODE_UNSET")]
pub type DestinationModeUnset = crate::Reg<destination_mode_unset::DestinationModeUnsetSpec>;
#[doc = "Write to unset destination mode register"]
pub mod destination_mode_unset;
#[doc = "DESTINATION_MODE_SET (w) register accessor: Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode_set`]
module"]
#[doc(alias = "DESTINATION_MODE_SET")]
pub type DestinationModeSet = crate::Reg<destination_mode_set::DestinationModeSetSpec>;
#[doc = "Write to set destination mode register"]
pub mod destination_mode_set;
#[doc = "DESTINATION_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode`]
module"]
#[doc(alias = "DESTINATION_MODE")]
pub type DestinationMode = crate::Reg<destination_mode::DestinationModeSpec>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod destination_mode;
#[doc = "DESTINATION_LENGTH (rw) register accessor: Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_length`]
module"]
#[doc(alias = "DESTINATION_LENGTH")]
pub type DestinationLength = crate::Reg<destination_length::DestinationLengthSpec>;
#[doc = "Length of write transfer in bytes"]
pub mod destination_length;
#[doc = "D_SOURCE_LENGTH (rw) register accessor: Length of read transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_length`]
module"]
#[doc(alias = "D_SOURCE_LENGTH")]
pub type DSourceLength = crate::Reg<d_source_length::DSourceLengthSpec>;
#[doc = "Length of read transfer in bytes"]
pub mod d_source_length;
#[doc = "D_SOURCE_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode`]
module"]
#[doc(alias = "D_SOURCE_MODE")]
pub type DSourceMode = crate::Reg<d_source_mode::DSourceModeSpec>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_source_mode;
#[doc = "D_SOURCE_MODE_SET (w) register accessor: Write to set source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode_set`]
module"]
#[doc(alias = "D_SOURCE_MODE_SET")]
pub type DSourceModeSet = crate::Reg<d_source_mode_set::DSourceModeSetSpec>;
#[doc = "Write to set source mode register"]
pub mod d_source_mode_set;
#[doc = "D_SOURCE_MODE_UNSET (w) register accessor: Write to unset source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode_unset`]
module"]
#[doc(alias = "D_SOURCE_MODE_UNSET")]
pub type DSourceModeUnset = crate::Reg<d_source_mode_unset::DSourceModeUnsetSpec>;
#[doc = "Write to unset source mode register"]
pub mod d_source_mode_unset;
#[doc = "D_SOURCE_AXI_CONFIG (rw) register accessor: AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_axi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_axi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_axi_config`]
module"]
#[doc(alias = "D_SOURCE_AXI_CONFIG")]
pub type DSourceAxiConfig = crate::Reg<d_source_axi_config::DSourceAxiConfigSpec>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_source_axi_config;
#[doc = "D_DESTINATION_ADDR (rw) register accessor: Start address for write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_addr`]
module"]
#[doc(alias = "D_DESTINATION_ADDR")]
pub type DDestinationAddr = crate::Reg<d_destination_addr::DDestinationAddrSpec>;
#[doc = "Start address for write"]
pub mod d_destination_addr;
#[doc = "D_DESTINATION_LENGTH (rw) register accessor: Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_length`]
module"]
#[doc(alias = "D_DESTINATION_LENGTH")]
pub type DDestinationLength = crate::Reg<d_destination_length::DDestinationLengthSpec>;
#[doc = "Length of write transfer in bytes"]
pub mod d_destination_length;
#[doc = "D_DESTINATION_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode`]
module"]
#[doc(alias = "D_DESTINATION_MODE")]
pub type DDestinationMode = crate::Reg<d_destination_mode::DDestinationModeSpec>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_destination_mode;
#[doc = "D_DESTINATION_MODE_SET (w) register accessor: Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode_set`]
module"]
#[doc(alias = "D_DESTINATION_MODE_SET")]
pub type DDestinationModeSet = crate::Reg<d_destination_mode_set::DDestinationModeSetSpec>;
#[doc = "Write to set destination mode register"]
pub mod d_destination_mode_set;
#[doc = "D_DESTINATION_MODE_UNSET (w) register accessor: Write to unset destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode_unset`]
module"]
#[doc(alias = "D_DESTINATION_MODE_UNSET")]
pub type DDestinationModeUnset = crate::Reg<d_destination_mode_unset::DDestinationModeUnsetSpec>;
#[doc = "Write to unset destination mode register"]
pub mod d_destination_mode_unset;
#[doc = "D_DESTINATION_AXI_CONFIG (rw) register accessor: AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_axi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_axi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_axi_config`]
module"]
#[doc(alias = "D_DESTINATION_AXI_CONFIG")]
pub type DDestinationAxiConfig = crate::Reg<d_destination_axi_config::DDestinationAxiConfigSpec>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_destination_axi_config;
