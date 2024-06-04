#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Configuration"]
#[doc(alias = "Configuration")]
pub struct CONFIGURATION {
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
    pad1_config: PAD1_CONFIG,
    pad2_config: PAD2_CONFIG,
    _reserved12: [u8; 0x10],
    clk_divider_conf: CLK_DIVIDER_CONF,
    lane_config: [LANE_CONFIG; 4],
    test_diagnosis_config: [TEST_DIAGNOSIS_CONFIG; 4],
    lane_status: [LANE_STATUS; 4],
}
impl CONFIGURATION {
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
    pub const fn pad1_config(&self) -> &PAD1_CONFIG {
        &self.pad1_config
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pad2_config(&self) -> &PAD2_CONFIG {
        &self.pad2_config
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn clk_divider_conf(&self) -> &CLK_DIVIDER_CONF {
        &self.clk_divider_conf
    }
    #[doc = "0x44..0x54 - "]
    #[inline(always)]
    pub const fn lane_config(&self, n: usize) -> &LANE_CONFIG {
        &self.lane_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - "]
    #[inline(always)]
    pub fn lane_config_iter(&self) -> impl Iterator<Item = &LANE_CONFIG> {
        self.lane_config.iter()
    }
    #[doc = "0x54..0x64 - "]
    #[inline(always)]
    pub const fn test_diagnosis_config(&self, n: usize) -> &TEST_DIAGNOSIS_CONFIG {
        &self.test_diagnosis_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x64 - "]
    #[inline(always)]
    pub fn test_diagnosis_config_iter(&self) -> impl Iterator<Item = &TEST_DIAGNOSIS_CONFIG> {
        self.test_diagnosis_config.iter()
    }
    #[doc = "0x64..0x74 - "]
    #[inline(always)]
    pub const fn lane_status(&self, n: usize) -> &LANE_STATUS {
        &self.lane_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x74 - "]
    #[inline(always)]
    pub fn lane_status_iter(&self) -> impl Iterator<Item = &LANE_STATUS> {
        self.lane_status.iter()
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
#[doc = "CLK_DIVIDER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_divider_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_divider_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_divider_conf`]
module"]
pub type CLK_DIVIDER_CONF = crate::Reg<clk_divider_conf::CLK_DIVIDER_CONF_SPEC>;
#[doc = ""]
pub mod clk_divider_conf;
#[doc = "LANE_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_status`]
module"]
pub type LANE_STATUS = crate::Reg<lane_status::LANE_STATUS_SPEC>;
#[doc = ""]
pub mod lane_status;
#[doc = "TEST_DIAGNOSIS_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_diagnosis_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_diagnosis_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_diagnosis_config`]
module"]
pub type TEST_DIAGNOSIS_CONFIG = crate::Reg<test_diagnosis_config::TEST_DIAGNOSIS_CONFIG_SPEC>;
#[doc = ""]
pub mod test_diagnosis_config;
#[doc = "LANE_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lane_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lane_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lane_config`]
module"]
pub type LANE_CONFIG = crate::Reg<lane_config::LANE_CONFIG_SPEC>;
#[doc = ""]
pub mod lane_config;
#[doc = "PAD2_CONFIG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad2_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad2_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_config`]
module"]
pub type PAD2_CONFIG = crate::Reg<pad2_config::PAD2_CONFIG_SPEC>;
#[doc = ""]
pub mod pad2_config;
#[doc = "PAD1_CONFIG (rw) register accessor: PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad1_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad1_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_config`]
module"]
pub type PAD1_CONFIG = crate::Reg<pad1_config::PAD1_CONFIG_SPEC>;
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad1_config;
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
#[doc = "ADDRESS_MAPPING_TX (rw) register accessor: Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address_mapping_tx`]
module"]
pub type ADDRESS_MAPPING_TX = crate::Reg<address_mapping_tx::ADDRESS_MAPPING_TX_SPEC>;
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_tx;
#[doc = "IRQ_CONFIG (rw) register accessor: Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_config`]
module"]
pub type IRQ_CONFIG = crate::Reg<irq_config::IRQ_CONFIG_SPEC>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub mod irq_config;
#[doc = "IRQ_EN (rw) register accessor: Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_en;
#[doc = "STATUS_CLR (rw) register accessor: Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_clr`]
module"]
pub type STATUS_CLR = crate::Reg<status_clr::STATUS_CLR_SPEC>;
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit"]
pub mod status_clr;
#[doc = "STATUS (r) register accessor: Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
pub mod status;
