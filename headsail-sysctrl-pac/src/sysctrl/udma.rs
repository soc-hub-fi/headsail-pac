#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "UDMA"]
pub struct UDMA {
    ctrl_cfg_cg: CTRL_CFG_CG,
    ctrl_cfg_event: CTRL_CFG_EVENT,
    ctrl_cfg_rst: CTRL_CFG_RST,
    _reserved3: [u8; 0x74],
    uart_rx_saddr: UART_RX_SADDR,
    uart_rx_size: UART_RX_SIZE,
    uart_rx_cfg: UART_RX_CFG,
    _reserved6: [u8; 0x04],
    uart_tx_saddr: UART_TX_SADDR,
    uart_tx_size: UART_TX_SIZE,
    uart_tx_cfg: UART_TX_CFG,
    _reserved9: [u8; 0x04],
    uart_status: UART_STATUS,
    uart_setup: UART_SETUP,
    uart_error: UART_ERROR,
    uart_irq_en: UART_IRQ_EN,
    uart_valid: UART_VALID,
    uart_data: UART_DATA,
    _reserved15: [u8; 0x48],
    spim_rx_saddr: SPIM_RX_SADDR,
    spim_rx_size: SPIM_RX_SIZE,
    spim_rx_cfg: SPIM_RX_CFG,
    _reserved18: [u8; 0x04],
    spim_tx_saddr: SPIM_TX_SADDR,
    spim_tx_size: SPIM_TX_SIZE,
    spim_tx_cfg: SPIM_TX_CFG,
    _reserved21: [u8; 0x04],
    spim_cmd_saddr: SPIM_CMD_SADDR,
    spim_cmd_size: SPIM_CMD_SIZE,
    spim_cmd_cfg: SPIM_CMD_CFG,
}
impl UDMA {
    #[doc = "0x00 - uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
    #[inline(always)]
    pub const fn ctrl_cfg_cg(&self) -> &CTRL_CFG_CG {
        &self.ctrl_cfg_cg
    }
    #[doc = "0x04 - uDMA peripherals external event configuration"]
    #[inline(always)]
    pub const fn ctrl_cfg_event(&self) -> &CTRL_CFG_EVENT {
        &self.ctrl_cfg_event
    }
    #[doc = "0x08 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub const fn ctrl_cfg_rst(&self) -> &CTRL_CFG_RST {
        &self.ctrl_cfg_rst
    }
    #[doc = "0x80 - uDMA RX UART buffer base address configuration register"]
    #[inline(always)]
    pub const fn uart_rx_saddr(&self) -> &UART_RX_SADDR {
        &self.uart_rx_saddr
    }
    #[doc = "0x84 - uDMA RX UART buffer size configuration register"]
    #[inline(always)]
    pub const fn uart_rx_size(&self) -> &UART_RX_SIZE {
        &self.uart_rx_size
    }
    #[doc = "0x88 - uDMA RX UART stream configuration register"]
    #[inline(always)]
    pub const fn uart_rx_cfg(&self) -> &UART_RX_CFG {
        &self.uart_rx_cfg
    }
    #[doc = "0x90 - uDMA TX UART buffer base address configuration register."]
    #[inline(always)]
    pub const fn uart_tx_saddr(&self) -> &UART_TX_SADDR {
        &self.uart_tx_saddr
    }
    #[doc = "0x94 - uDMA TX UART buffer size configuration register"]
    #[inline(always)]
    pub const fn uart_tx_size(&self) -> &UART_TX_SIZE {
        &self.uart_tx_size
    }
    #[doc = "0x98 - uDMA TX UART stream configuration register."]
    #[inline(always)]
    pub const fn uart_tx_cfg(&self) -> &UART_TX_CFG {
        &self.uart_tx_cfg
    }
    #[doc = "0xa0 - uDMA UART status register"]
    #[inline(always)]
    pub const fn uart_status(&self) -> &UART_STATUS {
        &self.uart_status
    }
    #[doc = "0xa4 - UDMA UART configuration register."]
    #[inline(always)]
    pub const fn uart_setup(&self) -> &UART_SETUP {
        &self.uart_setup
    }
    #[doc = "0xa8 - uDMA UART Error status"]
    #[inline(always)]
    pub const fn uart_error(&self) -> &UART_ERROR {
        &self.uart_error
    }
    #[doc = "0xac - uDMA UART Read or Error interrupt enable register"]
    #[inline(always)]
    pub const fn uart_irq_en(&self) -> &UART_IRQ_EN {
        &self.uart_irq_en
    }
    #[doc = "0xb0 - uDMA UART Read polling data valid flag register"]
    #[inline(always)]
    pub const fn uart_valid(&self) -> &UART_VALID {
        &self.uart_valid
    }
    #[doc = "0xb4 - RX read data for polling or interrupt"]
    #[inline(always)]
    pub const fn uart_data(&self) -> &UART_DATA {
        &self.uart_data
    }
    #[doc = "0x100 - RX SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_rx_saddr(&self) -> &SPIM_RX_SADDR {
        &self.spim_rx_saddr
    }
    #[doc = "0x104 - RX SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_rx_size(&self) -> &SPIM_RX_SIZE {
        &self.spim_rx_size
    }
    #[doc = "0x108 - RX SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_rx_cfg(&self) -> &SPIM_RX_CFG {
        &self.spim_rx_cfg
    }
    #[doc = "0x110 - TX SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_tx_saddr(&self) -> &SPIM_TX_SADDR {
        &self.spim_tx_saddr
    }
    #[doc = "0x114 - TX SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_tx_size(&self) -> &SPIM_TX_SIZE {
        &self.spim_tx_size
    }
    #[doc = "0x118 - TX SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_tx_cfg(&self) -> &SPIM_TX_CFG {
        &self.spim_tx_cfg
    }
    #[doc = "0x120 - CMD SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_cmd_saddr(&self) -> &SPIM_CMD_SADDR {
        &self.spim_cmd_saddr
    }
    #[doc = "0x124 - CMD SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_cmd_size(&self) -> &SPIM_CMD_SIZE {
        &self.spim_cmd_size
    }
    #[doc = "0x128 - CMD SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_cmd_cfg(&self) -> &SPIM_CMD_CFG {
        &self.spim_cmd_cfg
    }
}
#[doc = "CTRL_CFG_CG (rw) register accessor: uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_cg`]
module"]
pub type CTRL_CFG_CG = crate::Reg<ctrl_cfg_cg::CTRL_CFG_CG_SPEC>;
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
pub mod ctrl_cfg_cg;
#[doc = "CTRL_CFG_EVENT (rw) register accessor: uDMA peripherals external event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_event`]
module"]
pub type CTRL_CFG_EVENT = crate::Reg<ctrl_cfg_event::CTRL_CFG_EVENT_SPEC>;
#[doc = "uDMA peripherals external event configuration"]
pub mod ctrl_cfg_event;
#[doc = "CTRL_CFG_RST (rw) register accessor: uDMA peripherals reset trigger (unimplemented)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_rst`]
module"]
pub type CTRL_CFG_RST = crate::Reg<ctrl_cfg_rst::CTRL_CFG_RST_SPEC>;
#[doc = "uDMA peripherals reset trigger (unimplemented)"]
pub mod ctrl_cfg_rst;
#[doc = "UART_RX_SADDR (rw) register accessor: uDMA RX UART buffer base address configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_saddr`]
module"]
pub type UART_RX_SADDR = crate::Reg<uart_rx_saddr::UART_RX_SADDR_SPEC>;
#[doc = "uDMA RX UART buffer base address configuration register"]
pub mod uart_rx_saddr;
#[doc = "UART_RX_SIZE (rw) register accessor: uDMA RX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_size`]
module"]
pub type UART_RX_SIZE = crate::Reg<uart_rx_size::UART_RX_SIZE_SPEC>;
#[doc = "uDMA RX UART buffer size configuration register"]
pub mod uart_rx_size;
#[doc = "UART_RX_CFG (rw) register accessor: uDMA RX UART stream configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_cfg`]
module"]
pub type UART_RX_CFG = crate::Reg<uart_rx_cfg::UART_RX_CFG_SPEC>;
#[doc = "uDMA RX UART stream configuration register"]
pub mod uart_rx_cfg;
#[doc = "UART_TX_SADDR (rw) register accessor: uDMA TX UART buffer base address configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_saddr`]
module"]
pub type UART_TX_SADDR = crate::Reg<uart_tx_saddr::UART_TX_SADDR_SPEC>;
#[doc = "uDMA TX UART buffer base address configuration register."]
pub mod uart_tx_saddr;
#[doc = "UART_TX_SIZE (rw) register accessor: uDMA TX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_size`]
module"]
pub type UART_TX_SIZE = crate::Reg<uart_tx_size::UART_TX_SIZE_SPEC>;
#[doc = "uDMA TX UART buffer size configuration register"]
pub mod uart_tx_size;
#[doc = "UART_TX_CFG (rw) register accessor: uDMA TX UART stream configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_cfg`]
module"]
pub type UART_TX_CFG = crate::Reg<uart_tx_cfg::UART_TX_CFG_SPEC>;
#[doc = "uDMA TX UART stream configuration register."]
pub mod uart_tx_cfg;
#[doc = "UART_ERROR (r) register accessor: uDMA UART Error status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_error::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_error`]
module"]
pub type UART_ERROR = crate::Reg<uart_error::UART_ERROR_SPEC>;
#[doc = "uDMA UART Error status"]
pub mod uart_error;
#[doc = "SPIM_RX_SIZE (rw) register accessor: RX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_size`]
module"]
pub type SPIM_RX_SIZE = crate::Reg<spim_rx_size::SPIM_RX_SIZE_SPEC>;
#[doc = "RX SPI uDMA transfer size of buffer"]
pub mod spim_rx_size;
#[doc = "UART_DATA (r) register accessor: RX read data for polling or interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_data`]
module"]
pub type UART_DATA = crate::Reg<uart_data::UART_DATA_SPEC>;
#[doc = "RX read data for polling or interrupt"]
pub mod uart_data;
#[doc = "SPIM_RX_SADDR (rw) register accessor: RX SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_saddr`]
module"]
pub type SPIM_RX_SADDR = crate::Reg<spim_rx_saddr::SPIM_RX_SADDR_SPEC>;
#[doc = "RX SPI uDMA transfer address of associated buffer"]
pub mod spim_rx_saddr;
#[doc = "UART_STATUS (r) register accessor: uDMA UART status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_status`]
module"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "uDMA UART status register"]
pub mod uart_status;
#[doc = "UART_IRQ_EN (rw) register accessor: uDMA UART Read or Error interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_irq_en`]
module"]
pub type UART_IRQ_EN = crate::Reg<uart_irq_en::UART_IRQ_EN_SPEC>;
#[doc = "uDMA UART Read or Error interrupt enable register"]
pub mod uart_irq_en;
#[doc = "UART_VALID (r) register accessor: uDMA UART Read polling data valid flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_valid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_valid`]
module"]
pub type UART_VALID = crate::Reg<uart_valid::UART_VALID_SPEC>;
#[doc = "uDMA UART Read polling data valid flag register"]
pub mod uart_valid;
#[doc = "SPIM_CMD_SADDR (rw) register accessor: CMD SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_saddr`]
module"]
pub type SPIM_CMD_SADDR = crate::Reg<spim_cmd_saddr::SPIM_CMD_SADDR_SPEC>;
#[doc = "CMD SPI uDMA transfer address of associated buffer"]
pub mod spim_cmd_saddr;
#[doc = "SPIM_TX_SADDR (rw) register accessor: TX SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_saddr`]
module"]
pub type SPIM_TX_SADDR = crate::Reg<spim_tx_saddr::SPIM_TX_SADDR_SPEC>;
#[doc = "TX SPI uDMA transfer address of associated buffer"]
pub mod spim_tx_saddr;
#[doc = "SPIM_CMD_SIZE (rw) register accessor: CMD SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_size`]
module"]
pub type SPIM_CMD_SIZE = crate::Reg<spim_cmd_size::SPIM_CMD_SIZE_SPEC>;
#[doc = "CMD SPI uDMA transfer size of buffer"]
pub mod spim_cmd_size;
#[doc = "SPIM_TX_SIZE (rw) register accessor: TX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_size`]
module"]
pub type SPIM_TX_SIZE = crate::Reg<spim_tx_size::SPIM_TX_SIZE_SPEC>;
#[doc = "TX SPI uDMA transfer size of buffer"]
pub mod spim_tx_size;
#[doc = "SPIM_CMD_CFG (rw) register accessor: CMD SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_cfg`]
module"]
pub type SPIM_CMD_CFG = crate::Reg<spim_cmd_cfg::SPIM_CMD_CFG_SPEC>;
#[doc = "CMD SPI uDMA transfer configuration"]
pub mod spim_cmd_cfg;
#[doc = "UART_SETUP (rw) register accessor: UDMA UART configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_setup`]
module"]
pub type UART_SETUP = crate::Reg<uart_setup::UART_SETUP_SPEC>;
#[doc = "UDMA UART configuration register."]
pub mod uart_setup;
#[doc = "SPIM_TX_CFG (rw) register accessor: TX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_cfg`]
module"]
pub type SPIM_TX_CFG = crate::Reg<spim_tx_cfg::SPIM_TX_CFG_SPEC>;
#[doc = "TX SPI uDMA transfer configuration"]
pub mod spim_tx_cfg;
#[doc = "SPIM_RX_CFG (rw) register accessor: RX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_cfg`]
module"]
pub type SPIM_RX_CFG = crate::Reg<spim_rx_cfg::SPIM_RX_CFG_SPEC>;
#[doc = "RX SPI uDMA transfer configuration"]
pub mod spim_rx_cfg;
