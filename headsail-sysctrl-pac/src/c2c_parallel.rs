#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    info: Info,
    status: Status,
    status_clr: StatusClr,
    irq_en: IrqEn,
    irq_config: IrqConfig,
    address_mapping_tx: AddressMappingTx,
    address_masking_tx: AddressMaskingTx,
    address_mapping_rx: AddressMappingRx,
    address_masking_rx: AddressMaskingRx,
    pad_c2c_config_1: PadC2cConfig1,
    pad_c2c_config_2: PadC2cConfig2,
}
impl RegisterBlock {
    #[doc = "0x00 - ID register. Constant ID value"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - Version number - AXI Address width - AXI Data width-Reserved"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x08 - Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Acknowledging status register. Clearing each respective one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clr(&self) -> &StatusClr {
        &self.status_clr
    }
    #[doc = "0x10 - Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IrqEn {
        &self.irq_en
    }
    #[doc = "0x14 - Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    pub const fn irq_config(&self) -> &IrqConfig {
        &self.irq_config
    }
    #[doc = "0x18 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub const fn address_mapping_tx(&self) -> &AddressMappingTx {
        &self.address_mapping_tx
    }
    #[doc = "0x1c - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub const fn address_masking_tx(&self) -> &AddressMaskingTx {
        &self.address_masking_tx
    }
    #[doc = "0x20 - Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub const fn address_mapping_rx(&self) -> &AddressMappingRx {
        &self.address_mapping_rx
    }
    #[doc = "0x24 - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub const fn address_masking_rx(&self) -> &AddressMaskingRx {
        &self.address_masking_rx
    }
    #[doc = "0x28 - PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub const fn pad_c2c_config_1(&self) -> &PadC2cConfig1 {
        &self.pad_c2c_config_1
    }
    #[doc = "0x2c - PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub const fn pad_c2c_config_2(&self) -> &PadC2cConfig2 {
        &self.pad_c2c_config_2
    }
}
#[doc = "ID (r) register accessor: ID register. Constant ID value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ID register. Constant ID value"]
pub mod id;
#[doc = "INFO (r) register accessor: Version number - AXI Address width - AXI Data width-Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Version number - AXI Address width - AXI Data width-Reserved"]
pub mod info;
#[doc = "STATUS (r) register accessor: Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
pub mod status;
#[doc = "STATUS_CLR (rw) register accessor: Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_clr`]
module"]
#[doc(alias = "STATUS_CLR")]
pub type StatusClr = crate::Reg<status_clr::StatusClrSpec>;
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit"]
pub mod status_clr;
#[doc = "IRQ_EN (rw) register accessor: Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
#[doc(alias = "IRQ_EN")]
pub type IrqEn = crate::Reg<irq_en::IrqEnSpec>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_en;
#[doc = "IRQ_CONFIG (rw) register accessor: Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_config`]
module"]
#[doc(alias = "IRQ_CONFIG")]
pub type IrqConfig = crate::Reg<irq_config::IrqConfigSpec>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub mod irq_config;
#[doc = "ADDRESS_MAPPING_TX (rw) register accessor: Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_tx`]
module"]
#[doc(alias = "ADDRESS_MAPPING_TX")]
pub type AddressMappingTx = crate::Reg<address_mapping_tx::AddressMappingTxSpec>;
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_tx;
#[doc = "PAD_C2C_CONFIG_2 (rw) register accessor: PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_c2c_config_2`]
module"]
#[doc(alias = "PAD_C2C_CONFIG_2")]
pub type PadC2cConfig2 = crate::Reg<pad_c2c_config_2::PadC2cConfig2Spec>;
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_2;
#[doc = "PAD_C2C_CONFIG_1 (rw) register accessor: PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_c2c_config_1`]
module"]
#[doc(alias = "PAD_C2C_CONFIG_1")]
pub type PadC2cConfig1 = crate::Reg<pad_c2c_config_1::PadC2cConfig1Spec>;
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_1;
#[doc = "ADDRESS_MASKING_RX (rw) register accessor: Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_masking_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_masking_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_masking_rx`]
module"]
#[doc(alias = "ADDRESS_MASKING_RX")]
pub type AddressMaskingRx = crate::Reg<address_masking_rx::AddressMaskingRxSpec>;
#[doc = "Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_rx;
#[doc = "ADDRESS_MAPPING_RX (rw) register accessor: Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_rx`]
module"]
#[doc(alias = "ADDRESS_MAPPING_RX")]
pub type AddressMappingRx = crate::Reg<address_mapping_rx::AddressMappingRxSpec>;
#[doc = "Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_rx;
#[doc = "ADDRESS_MASKING_TX (rw) register accessor: Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_masking_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_masking_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_masking_tx`]
module"]
#[doc(alias = "ADDRESS_MASKING_TX")]
pub type AddressMaskingTx = crate::Reg<address_masking_tx::AddressMaskingTxSpec>;
#[doc = "Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_tx;
