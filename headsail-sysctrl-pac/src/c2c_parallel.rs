#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    info: INFO,
    status: STATUS,
    status_clr: STATUS_CLR,
    irq_en: IRQ_EN,
    irq_config: IRQ_CONFIG,
    address_mapping_tx: ADDRESS_MAPPING_TX,
    address_masking_tx: ADDRESS_MASKING_TX,
    address_mapping_rx: ADDRESS_MAPPING_RX,
    address_masking_rx: ADDRESS_MASKING_RX,
    pad_c2c_config_1: PAD_C2C_CONFIG_1,
    pad_c2c_config_2: PAD_C2C_CONFIG_2,
}
impl RegisterBlock {
    #[doc = "0x00 - ID register. Constant ID value"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - Version number - AXI Address width - AXI Data width-Reserved"]
    #[inline(always)]
    pub const fn info(&self) -> &INFO {
        &self.info
    }
    #[doc = "0x08 - Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - Acknowledging status register. Clearing each respective one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clr(&self) -> &STATUS_CLR {
        &self.status_clr
    }
    #[doc = "0x10 - Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IRQ_EN {
        &self.irq_en
    }
    #[doc = "0x14 - Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    pub const fn irq_config(&self) -> &IRQ_CONFIG {
        &self.irq_config
    }
    #[doc = "0x18 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub const fn address_mapping_tx(&self) -> &ADDRESS_MAPPING_TX {
        &self.address_mapping_tx
    }
    #[doc = "0x1c - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub const fn address_masking_tx(&self) -> &ADDRESS_MASKING_TX {
        &self.address_masking_tx
    }
    #[doc = "0x20 - Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub const fn address_mapping_rx(&self) -> &ADDRESS_MAPPING_RX {
        &self.address_mapping_rx
    }
    #[doc = "0x24 - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub const fn address_masking_rx(&self) -> &ADDRESS_MASKING_RX {
        &self.address_masking_rx
    }
    #[doc = "0x28 - PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub const fn pad_c2c_config_1(&self) -> &PAD_C2C_CONFIG_1 {
        &self.pad_c2c_config_1
    }
    #[doc = "0x2c - PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub const fn pad_c2c_config_2(&self) -> &PAD_C2C_CONFIG_2 {
        &self.pad_c2c_config_2
    }
}
#[doc = "ID (r) register accessor: ID register. Constant ID value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID register. Constant ID value"]
pub mod id;
#[doc = "INFO (r) register accessor: Version number - AXI Address width - AXI Data width-Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Version number - AXI Address width - AXI Data width-Reserved"]
pub mod info;
#[doc = "STATUS (r) register accessor: Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
pub mod status;
#[doc = "STATUS_CLR (rw) register accessor: Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_clr`]
module"]
pub type STATUS_CLR = crate::Reg<status_clr::STATUS_CLR_SPEC>;
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit"]
pub mod status_clr;
#[doc = "IRQ_EN (rw) register accessor: Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_en;
#[doc = "IRQ_CONFIG (rw) register accessor: Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_config`]
module"]
pub type IRQ_CONFIG = crate::Reg<irq_config::IRQ_CONFIG_SPEC>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub mod irq_config;
#[doc = "ADDRESS_MAPPING_TX (rw) register accessor: Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_tx`]
module"]
pub type ADDRESS_MAPPING_TX = crate::Reg<address_mapping_tx::ADDRESS_MAPPING_TX_SPEC>;
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_tx;
#[doc = "PAD_C2C_CONFIG_2 (rw) register accessor: PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_c2c_config_2`]
module"]
pub type PAD_C2C_CONFIG_2 = crate::Reg<pad_c2c_config_2::PAD_C2C_CONFIG_2_SPEC>;
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_2;
#[doc = "PAD_C2C_CONFIG_1 (rw) register accessor: PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_c2c_config_1`]
module"]
pub type PAD_C2C_CONFIG_1 = crate::Reg<pad_c2c_config_1::PAD_C2C_CONFIG_1_SPEC>;
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_1;
#[doc = "ADDRESS_MASKING_RX (rw) register accessor: Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_masking_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_masking_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_masking_rx`]
module"]
pub type ADDRESS_MASKING_RX = crate::Reg<address_masking_rx::ADDRESS_MASKING_RX_SPEC>;
#[doc = "Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_rx;
#[doc = "ADDRESS_MAPPING_RX (rw) register accessor: Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_rx`]
module"]
pub type ADDRESS_MAPPING_RX = crate::Reg<address_mapping_rx::ADDRESS_MAPPING_RX_SPEC>;
#[doc = "Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_rx;
#[doc = "ADDRESS_MASKING_TX (rw) register accessor: Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_masking_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_masking_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_masking_tx`]
module"]
pub type ADDRESS_MASKING_TX = crate::Reg<address_masking_tx::ADDRESS_MASKING_TX_SPEC>;
#[doc = "Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_tx;
