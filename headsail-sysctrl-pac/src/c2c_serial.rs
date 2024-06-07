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
    pad1_config: Pad1Config,
    pad2_config: Pad2Config,
    _reserved12: [u8; 0x10],
    clk_divider_conf: ClkDividerConf,
    lane_config: [LaneConfig; 4],
    test_diagnosis_config: [TestDiagnosisConfig; 4],
    lane_status: [LaneStatus; 4],
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
    pub const fn pad1_config(&self) -> &Pad1Config {
        &self.pad1_config
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pad2_config(&self) -> &Pad2Config {
        &self.pad2_config
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn clk_divider_conf(&self) -> &ClkDividerConf {
        &self.clk_divider_conf
    }
    #[doc = "0x44..0x54 - "]
    #[inline(always)]
    pub const fn lane_config(&self, n: usize) -> &LaneConfig {
        &self.lane_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - "]
    #[inline(always)]
    pub fn lane_config_iter(&self) -> impl Iterator<Item = &LaneConfig> {
        self.lane_config.iter()
    }
    #[doc = "0x54..0x64 - "]
    #[inline(always)]
    pub const fn test_diagnosis_config(&self, n: usize) -> &TestDiagnosisConfig {
        &self.test_diagnosis_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x64 - "]
    #[inline(always)]
    pub fn test_diagnosis_config_iter(&self) -> impl Iterator<Item = &TestDiagnosisConfig> {
        self.test_diagnosis_config.iter()
    }
    #[doc = "0x64..0x74 - "]
    #[inline(always)]
    pub const fn lane_status(&self, n: usize) -> &LaneStatus {
        &self.lane_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x74 - "]
    #[inline(always)]
    pub fn lane_status_iter(&self) -> impl Iterator<Item = &LaneStatus> {
        self.lane_status.iter()
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
#[doc = "CLK_DIVIDER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_divider_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_divider_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_divider_conf`]
module"]
#[doc(alias = "CLK_DIVIDER_CONF")]
pub type ClkDividerConf = crate::Reg<clk_divider_conf::ClkDividerConfSpec>;
#[doc = ""]
pub mod clk_divider_conf;
#[doc = "LANE_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_status`]
module"]
#[doc(alias = "LANE_STATUS")]
pub type LaneStatus = crate::Reg<lane_status::LaneStatusSpec>;
#[doc = ""]
pub mod lane_status;
#[doc = "TEST_DIAGNOSIS_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_diagnosis_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_diagnosis_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_diagnosis_config`]
module"]
#[doc(alias = "TEST_DIAGNOSIS_CONFIG")]
pub type TestDiagnosisConfig = crate::Reg<test_diagnosis_config::TestDiagnosisConfigSpec>;
#[doc = ""]
pub mod test_diagnosis_config;
#[doc = "LANE_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_config`]
module"]
#[doc(alias = "LANE_CONFIG")]
pub type LaneConfig = crate::Reg<lane_config::LaneConfigSpec>;
#[doc = ""]
pub mod lane_config;
#[doc = "PAD2_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad2_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad2_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_config`]
module"]
#[doc(alias = "PAD2_CONFIG")]
pub type Pad2Config = crate::Reg<pad2_config::Pad2ConfigSpec>;
#[doc = ""]
pub mod pad2_config;
#[doc = "PAD1_CONFIG (rw) register accessor: PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad1_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad1_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_config`]
module"]
#[doc(alias = "PAD1_CONFIG")]
pub type Pad1Config = crate::Reg<pad1_config::Pad1ConfigSpec>;
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad1_config;
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
#[doc = "ADDRESS_MAPPING_TX (rw) register accessor: Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_tx`]
module"]
#[doc(alias = "ADDRESS_MAPPING_TX")]
pub type AddressMappingTx = crate::Reg<address_mapping_tx::AddressMappingTxSpec>;
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_tx;
#[doc = "IRQ_CONFIG (rw) register accessor: Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_config`]
module"]
#[doc(alias = "IRQ_CONFIG")]
pub type IrqConfig = crate::Reg<irq_config::IrqConfigSpec>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub mod irq_config;
#[doc = "IRQ_EN (rw) register accessor: Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
#[doc(alias = "IRQ_EN")]
pub type IrqEn = crate::Reg<irq_en::IrqEnSpec>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_en;
#[doc = "STATUS_CLR (rw) register accessor: Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_clr`]
module"]
#[doc(alias = "STATUS_CLR")]
pub type StatusClr = crate::Reg<status_clr::StatusClrSpec>;
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit"]
pub mod status_clr;
#[doc = "STATUS (r) register accessor: Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
pub mod status;
