#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "channel1"]
#[doc(alias = "channel1")]
pub struct CHANNEL1 {
    source_addr: SOURCE_ADDR,
    source_length: SOURCE_LENGTH,
    source_mode: SOURCE_MODE,
    source_mode_set: SOURCE_MODE_SET,
    source_mode_unset: SOURCE_MODE_UNSET,
    axi_read_config: AXI_READ_CONFIG,
    _reserved6: [u8; 0x04],
    destination_addr: DESTINATION_ADDR,
    destination_length: DESTINATION_LENGTH,
    destination_mode: DESTINATION_MODE,
    destination_mode_set: DESTINATION_MODE_SET,
    destination_mode_unset: DESTINATION_MODE_UNSET,
    axi_write_config: AXI_WRITE_CONFIG,
    _reserved12: [u8; 0x0c],
    d_source_addr: D_SOURCE_ADDR,
    d_source_length: D_SOURCE_LENGTH,
    d_source_mode: D_SOURCE_MODE,
    d_source_mode_set: D_SOURCE_MODE_SET,
    d_source_mode_unset: D_SOURCE_MODE_UNSET,
    d_source_axi_config: D_SOURCE_AXI_CONFIG,
    _reserved18: [u8; 0x04],
    d_destination_addr: D_DESTINATION_ADDR,
    d_destination_length: D_DESTINATION_LENGTH,
    d_destination_mode: D_DESTINATION_MODE,
    d_destination_mode_set: D_DESTINATION_MODE_SET,
    d_destination_mode_unset: D_DESTINATION_MODE_UNSET,
    d_destination_axi_config: D_DESTINATION_AXI_CONFIG,
}
impl CHANNEL1 {
    #[doc = "0x00 - Start address for read"]
    #[inline(always)]
    pub const fn source_addr(&self) -> &SOURCE_ADDR {
        &self.source_addr
    }
    #[doc = "0x04 - Length of read transfer in bytes"]
    #[inline(always)]
    pub const fn source_length(&self) -> &SOURCE_LENGTH {
        &self.source_length
    }
    #[doc = "0x08 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn source_mode(&self) -> &SOURCE_MODE {
        &self.source_mode
    }
    #[doc = "0x0c - Write to set source mode register"]
    #[inline(always)]
    pub const fn source_mode_set(&self) -> &SOURCE_MODE_SET {
        &self.source_mode_set
    }
    #[doc = "0x10 - Write to unset source mode register"]
    #[inline(always)]
    pub const fn source_mode_unset(&self) -> &SOURCE_MODE_UNSET {
        &self.source_mode_unset
    }
    #[doc = "0x14 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn axi_read_config(&self) -> &AXI_READ_CONFIG {
        &self.axi_read_config
    }
    #[doc = "0x1c - Start address for write"]
    #[inline(always)]
    pub const fn destination_addr(&self) -> &DESTINATION_ADDR {
        &self.destination_addr
    }
    #[doc = "0x20 - Length of write transfer in bytes"]
    #[inline(always)]
    pub const fn destination_length(&self) -> &DESTINATION_LENGTH {
        &self.destination_length
    }
    #[doc = "0x24 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn destination_mode(&self) -> &DESTINATION_MODE {
        &self.destination_mode
    }
    #[doc = "0x28 - Write to set destination mode register"]
    #[inline(always)]
    pub const fn destination_mode_set(&self) -> &DESTINATION_MODE_SET {
        &self.destination_mode_set
    }
    #[doc = "0x2c - Write to unset destination mode register"]
    #[inline(always)]
    pub const fn destination_mode_unset(&self) -> &DESTINATION_MODE_UNSET {
        &self.destination_mode_unset
    }
    #[doc = "0x30 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn axi_write_config(&self) -> &AXI_WRITE_CONFIG {
        &self.axi_write_config
    }
    #[doc = "0x40 - Start address for read"]
    #[inline(always)]
    pub const fn d_source_addr(&self) -> &D_SOURCE_ADDR {
        &self.d_source_addr
    }
    #[doc = "0x44 - Length of read transfer in bytes"]
    #[inline(always)]
    pub const fn d_source_length(&self) -> &D_SOURCE_LENGTH {
        &self.d_source_length
    }
    #[doc = "0x48 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn d_source_mode(&self) -> &D_SOURCE_MODE {
        &self.d_source_mode
    }
    #[doc = "0x4c - Write to set source mode register"]
    #[inline(always)]
    pub const fn d_source_mode_set(&self) -> &D_SOURCE_MODE_SET {
        &self.d_source_mode_set
    }
    #[doc = "0x50 - Write to unset source mode register"]
    #[inline(always)]
    pub const fn d_source_mode_unset(&self) -> &D_SOURCE_MODE_UNSET {
        &self.d_source_mode_unset
    }
    #[doc = "0x54 - AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn d_source_axi_config(&self) -> &D_SOURCE_AXI_CONFIG {
        &self.d_source_axi_config
    }
    #[doc = "0x5c - Start address for write"]
    #[inline(always)]
    pub const fn d_destination_addr(&self) -> &D_DESTINATION_ADDR {
        &self.d_destination_addr
    }
    #[doc = "0x60 - Length of write transfer in bytes"]
    #[inline(always)]
    pub const fn d_destination_length(&self) -> &D_DESTINATION_LENGTH {
        &self.d_destination_length
    }
    #[doc = "0x64 - ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
    #[inline(always)]
    pub const fn d_destination_mode(&self) -> &D_DESTINATION_MODE {
        &self.d_destination_mode
    }
    #[doc = "0x68 - Write to set destination mode register"]
    #[inline(always)]
    pub const fn d_destination_mode_set(&self) -> &D_DESTINATION_MODE_SET {
        &self.d_destination_mode_set
    }
    #[doc = "0x6c - Write to unset destination mode register"]
    #[inline(always)]
    pub const fn d_destination_mode_unset(&self) -> &D_DESTINATION_MODE_UNSET {
        &self.d_destination_mode_unset
    }
    #[doc = "0x70 - AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
    #[inline(always)]
    pub const fn d_destination_axi_config(&self) -> &D_DESTINATION_AXI_CONFIG {
        &self.d_destination_axi_config
    }
}
#[doc = "SOURCE_ADDR (rw) register accessor: Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_addr`]
module"]
pub type SOURCE_ADDR = crate::Reg<source_addr::SOURCE_ADDR_SPEC>;
#[doc = "Start address for read"]
pub mod source_addr;
#[doc = "SOURCE_LENGTH (rw) register accessor: Length of read transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_length`]
module"]
pub type SOURCE_LENGTH = crate::Reg<source_length::SOURCE_LENGTH_SPEC>;
#[doc = "Length of read transfer in bytes"]
pub mod source_length;
#[doc = "SOURCE_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode`]
module"]
pub type SOURCE_MODE = crate::Reg<source_mode::SOURCE_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod source_mode;
#[doc = "SOURCE_MODE_SET (w) register accessor: Write to set source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode_set`]
module"]
pub type SOURCE_MODE_SET = crate::Reg<source_mode_set::SOURCE_MODE_SET_SPEC>;
#[doc = "Write to set source mode register"]
pub mod source_mode_set;
#[doc = "SOURCE_MODE_UNSET (w) register accessor: Write to unset source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source_mode_unset`]
module"]
pub type SOURCE_MODE_UNSET = crate::Reg<source_mode_unset::SOURCE_MODE_UNSET_SPEC>;
#[doc = "Write to unset source mode register"]
pub mod source_mode_unset;
#[doc = "AXI_READ_CONFIG (rw) register accessor: AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_read_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_read_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_read_config`]
module"]
pub type AXI_READ_CONFIG = crate::Reg<axi_read_config::AXI_READ_CONFIG_SPEC>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_read_config;
#[doc = "DESTINATION_ADDR (rw) register accessor: Start address for write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_addr`]
module"]
pub type DESTINATION_ADDR = crate::Reg<destination_addr::DESTINATION_ADDR_SPEC>;
#[doc = "Start address for write"]
pub mod destination_addr;
#[doc = "D_SOURCE_ADDR (rw) register accessor: Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_addr`]
module"]
pub type D_SOURCE_ADDR = crate::Reg<d_source_addr::D_SOURCE_ADDR_SPEC>;
#[doc = "Start address for read"]
pub mod d_source_addr;
#[doc = "AXI_WRITE_CONFIG (rw) register accessor: AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_write_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_write_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_write_config`]
module"]
pub type AXI_WRITE_CONFIG = crate::Reg<axi_write_config::AXI_WRITE_CONFIG_SPEC>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod axi_write_config;
#[doc = "DESTINATION_MODE_UNSET (w) register accessor: Write to unset destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode_unset`]
module"]
pub type DESTINATION_MODE_UNSET = crate::Reg<destination_mode_unset::DESTINATION_MODE_UNSET_SPEC>;
#[doc = "Write to unset destination mode register"]
pub mod destination_mode_unset;
#[doc = "DESTINATION_MODE_SET (w) register accessor: Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode_set`]
module"]
pub type DESTINATION_MODE_SET = crate::Reg<destination_mode_set::DESTINATION_MODE_SET_SPEC>;
#[doc = "Write to set destination mode register"]
pub mod destination_mode_set;
#[doc = "DESTINATION_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_mode`]
module"]
pub type DESTINATION_MODE = crate::Reg<destination_mode::DESTINATION_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod destination_mode;
#[doc = "DESTINATION_LENGTH (rw) register accessor: Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination_length`]
module"]
pub type DESTINATION_LENGTH = crate::Reg<destination_length::DESTINATION_LENGTH_SPEC>;
#[doc = "Length of write transfer in bytes"]
pub mod destination_length;
#[doc = "D_SOURCE_LENGTH (rw) register accessor: Length of read transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_length`]
module"]
pub type D_SOURCE_LENGTH = crate::Reg<d_source_length::D_SOURCE_LENGTH_SPEC>;
#[doc = "Length of read transfer in bytes"]
pub mod d_source_length;
#[doc = "D_SOURCE_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode`]
module"]
pub type D_SOURCE_MODE = crate::Reg<d_source_mode::D_SOURCE_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_source_mode;
#[doc = "D_SOURCE_MODE_SET (w) register accessor: Write to set source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode_set`]
module"]
pub type D_SOURCE_MODE_SET = crate::Reg<d_source_mode_set::D_SOURCE_MODE_SET_SPEC>;
#[doc = "Write to set source mode register"]
pub mod d_source_mode_set;
#[doc = "D_SOURCE_MODE_UNSET (w) register accessor: Write to unset source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_mode_unset`]
module"]
pub type D_SOURCE_MODE_UNSET = crate::Reg<d_source_mode_unset::D_SOURCE_MODE_UNSET_SPEC>;
#[doc = "Write to unset source mode register"]
pub mod d_source_mode_unset;
#[doc = "D_SOURCE_AXI_CONFIG (rw) register accessor: AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_source_axi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_axi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_source_axi_config`]
module"]
pub type D_SOURCE_AXI_CONFIG = crate::Reg<d_source_axi_config::D_SOURCE_AXI_CONFIG_SPEC>;
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_source_axi_config;
#[doc = "D_DESTINATION_ADDR (rw) register accessor: Start address for write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_addr`]
module"]
pub type D_DESTINATION_ADDR = crate::Reg<d_destination_addr::D_DESTINATION_ADDR_SPEC>;
#[doc = "Start address for write"]
pub mod d_destination_addr;
#[doc = "D_DESTINATION_LENGTH (rw) register accessor: Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_length`]
module"]
pub type D_DESTINATION_LENGTH = crate::Reg<d_destination_length::D_DESTINATION_LENGTH_SPEC>;
#[doc = "Length of write transfer in bytes"]
pub mod d_destination_length;
#[doc = "D_DESTINATION_MODE (rw) register accessor: ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode`]
module"]
pub type D_DESTINATION_MODE = crate::Reg<d_destination_mode::D_DESTINATION_MODE_SPEC>;
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified."]
pub mod d_destination_mode;
#[doc = "D_DESTINATION_MODE_SET (w) register accessor: Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode_set`]
module"]
pub type D_DESTINATION_MODE_SET = crate::Reg<d_destination_mode_set::D_DESTINATION_MODE_SET_SPEC>;
#[doc = "Write to set destination mode register"]
pub mod d_destination_mode_set;
#[doc = "D_DESTINATION_MODE_UNSET (w) register accessor: Write to unset destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode_unset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_mode_unset`]
module"]
pub type D_DESTINATION_MODE_UNSET =
    crate::Reg<d_destination_mode_unset::D_DESTINATION_MODE_UNSET_SPEC>;
#[doc = "Write to unset destination mode register"]
pub mod d_destination_mode_unset;
#[doc = "D_DESTINATION_AXI_CONFIG (rw) register accessor: AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_axi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_axi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_destination_axi_config`]
module"]
pub type D_DESTINATION_AXI_CONFIG =
    crate::Reg<d_destination_axi_config::D_DESTINATION_AXI_CONFIG_SPEC>;
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1"]
pub mod d_destination_axi_config;
