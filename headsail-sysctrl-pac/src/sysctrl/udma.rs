#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "UDMA"]
#[doc(alias = "UDMA")]
pub struct Udma {
    ctrl_cfg_cg: CtrlCfgCg,
    ctrl_cfg_event: CtrlCfgEvent,
    ctrl_cfg_rst: CtrlCfgRst,
    _reserved3: [u8; 0x74],
    uart_rx_saddr: UartRxSaddr,
    uart_rx_size: UartRxSize,
    uart_rx_cfg: UartRxCfg,
    _reserved6: [u8; 0x04],
    uart_tx_saddr: UartTxSaddr,
    uart_tx_size: UartTxSize,
    uart_tx_cfg: UartTxCfg,
    _reserved9: [u8; 0x04],
    uart_status: UartStatus,
    uart_setup: UartSetup,
    uart_error: UartError,
    uart_irq_en: UartIrqEn,
    uart_valid: UartValid,
    uart_data: UartData,
    _reserved15: [u8; 0x48],
    spim_rx_saddr: SpimRxSaddr,
    spim_rx_size: SpimRxSize,
    spim_rx_cfg: SpimRxCfg,
    _reserved18: [u8; 0x04],
    spim_tx_saddr: SpimTxSaddr,
    spim_tx_size: SpimTxSize,
    spim_tx_cfg: SpimTxCfg,
    _reserved21: [u8; 0x04],
    spim_cmd_saddr: SpimCmdSaddr,
    spim_cmd_size: SpimCmdSize,
    spim_cmd_cfg: SpimCmdCfg,
}
impl Udma {
    #[doc = "0x00 - uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
    #[inline(always)]
    pub const fn ctrl_cfg_cg(&self) -> &CtrlCfgCg {
        &self.ctrl_cfg_cg
    }
    #[doc = "0x04 - uDMA peripherals external event configuration"]
    #[inline(always)]
    pub const fn ctrl_cfg_event(&self) -> &CtrlCfgEvent {
        &self.ctrl_cfg_event
    }
    #[doc = "0x08 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub const fn ctrl_cfg_rst(&self) -> &CtrlCfgRst {
        &self.ctrl_cfg_rst
    }
    #[doc = "0x80 - uDMA RX UART buffer base address configuration register"]
    #[inline(always)]
    pub const fn uart_rx_saddr(&self) -> &UartRxSaddr {
        &self.uart_rx_saddr
    }
    #[doc = "0x84 - uDMA RX UART buffer size configuration register"]
    #[inline(always)]
    pub const fn uart_rx_size(&self) -> &UartRxSize {
        &self.uart_rx_size
    }
    #[doc = "0x88 - uDMA RX UART stream configuration register"]
    #[inline(always)]
    pub const fn uart_rx_cfg(&self) -> &UartRxCfg {
        &self.uart_rx_cfg
    }
    #[doc = "0x90 - uDMA TX UART buffer base address configuration register."]
    #[inline(always)]
    pub const fn uart_tx_saddr(&self) -> &UartTxSaddr {
        &self.uart_tx_saddr
    }
    #[doc = "0x94 - uDMA TX UART buffer size configuration register"]
    #[inline(always)]
    pub const fn uart_tx_size(&self) -> &UartTxSize {
        &self.uart_tx_size
    }
    #[doc = "0x98 - uDMA TX UART stream configuration register."]
    #[inline(always)]
    pub const fn uart_tx_cfg(&self) -> &UartTxCfg {
        &self.uart_tx_cfg
    }
    #[doc = "0xa0 - uDMA UART status register"]
    #[inline(always)]
    pub const fn uart_status(&self) -> &UartStatus {
        &self.uart_status
    }
    #[doc = "0xa4 - UDMA UART configuration register."]
    #[inline(always)]
    pub const fn uart_setup(&self) -> &UartSetup {
        &self.uart_setup
    }
    #[doc = "0xa8 - uDMA UART Error status"]
    #[inline(always)]
    pub const fn uart_error(&self) -> &UartError {
        &self.uart_error
    }
    #[doc = "0xac - uDMA UART Read or Error interrupt enable register"]
    #[inline(always)]
    pub const fn uart_irq_en(&self) -> &UartIrqEn {
        &self.uart_irq_en
    }
    #[doc = "0xb0 - uDMA UART Read polling data valid flag register"]
    #[inline(always)]
    pub const fn uart_valid(&self) -> &UartValid {
        &self.uart_valid
    }
    #[doc = "0xb4 - RX read data for polling or interrupt"]
    #[inline(always)]
    pub const fn uart_data(&self) -> &UartData {
        &self.uart_data
    }
    #[doc = "0x100 - RX SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_rx_saddr(&self) -> &SpimRxSaddr {
        &self.spim_rx_saddr
    }
    #[doc = "0x104 - RX SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_rx_size(&self) -> &SpimRxSize {
        &self.spim_rx_size
    }
    #[doc = "0x108 - RX SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_rx_cfg(&self) -> &SpimRxCfg {
        &self.spim_rx_cfg
    }
    #[doc = "0x110 - TX SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_tx_saddr(&self) -> &SpimTxSaddr {
        &self.spim_tx_saddr
    }
    #[doc = "0x114 - TX SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_tx_size(&self) -> &SpimTxSize {
        &self.spim_tx_size
    }
    #[doc = "0x118 - TX SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_tx_cfg(&self) -> &SpimTxCfg {
        &self.spim_tx_cfg
    }
    #[doc = "0x120 - CMD SPI uDMA transfer address of associated buffer"]
    #[inline(always)]
    pub const fn spim_cmd_saddr(&self) -> &SpimCmdSaddr {
        &self.spim_cmd_saddr
    }
    #[doc = "0x124 - CMD SPI uDMA transfer size of buffer"]
    #[inline(always)]
    pub const fn spim_cmd_size(&self) -> &SpimCmdSize {
        &self.spim_cmd_size
    }
    #[doc = "0x128 - CMD SPI uDMA transfer configuration"]
    #[inline(always)]
    pub const fn spim_cmd_cfg(&self) -> &SpimCmdCfg {
        &self.spim_cmd_cfg
    }
}
#[doc = "CTRL_CFG_CG (rw) register accessor: uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_cg`]
module"]
#[doc(alias = "CTRL_CFG_CG")]
pub type CtrlCfgCg = crate::Reg<ctrl_cfg_cg::CtrlCfgCgSpec>;
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
pub mod ctrl_cfg_cg;
#[doc = "CTRL_CFG_EVENT (rw) register accessor: uDMA peripherals external event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_event`]
module"]
#[doc(alias = "CTRL_CFG_EVENT")]
pub type CtrlCfgEvent = crate::Reg<ctrl_cfg_event::CtrlCfgEventSpec>;
#[doc = "uDMA peripherals external event configuration"]
pub mod ctrl_cfg_event;
#[doc = "CTRL_CFG_RST (rw) register accessor: uDMA peripherals reset trigger (unimplemented)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg_rst`]
module"]
#[doc(alias = "CTRL_CFG_RST")]
pub type CtrlCfgRst = crate::Reg<ctrl_cfg_rst::CtrlCfgRstSpec>;
#[doc = "uDMA peripherals reset trigger (unimplemented)"]
pub mod ctrl_cfg_rst;
#[doc = "UART_RX_SADDR (rw) register accessor: uDMA RX UART buffer base address configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_saddr`]
module"]
#[doc(alias = "UART_RX_SADDR")]
pub type UartRxSaddr = crate::Reg<uart_rx_saddr::UartRxSaddrSpec>;
#[doc = "uDMA RX UART buffer base address configuration register"]
pub mod uart_rx_saddr;
#[doc = "UART_RX_SIZE (rw) register accessor: uDMA RX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_size`]
module"]
#[doc(alias = "UART_RX_SIZE")]
pub type UartRxSize = crate::Reg<uart_rx_size::UartRxSizeSpec>;
#[doc = "uDMA RX UART buffer size configuration register"]
pub mod uart_rx_size;
#[doc = "UART_RX_CFG (rw) register accessor: uDMA RX UART stream configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_rx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_cfg`]
module"]
#[doc(alias = "UART_RX_CFG")]
pub type UartRxCfg = crate::Reg<uart_rx_cfg::UartRxCfgSpec>;
#[doc = "uDMA RX UART stream configuration register"]
pub mod uart_rx_cfg;
#[doc = "UART_TX_SADDR (rw) register accessor: uDMA TX UART buffer base address configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_saddr`]
module"]
#[doc(alias = "UART_TX_SADDR")]
pub type UartTxSaddr = crate::Reg<uart_tx_saddr::UartTxSaddrSpec>;
#[doc = "uDMA TX UART buffer base address configuration register."]
pub mod uart_tx_saddr;
#[doc = "UART_TX_SIZE (rw) register accessor: uDMA TX UART buffer size configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_size`]
module"]
#[doc(alias = "UART_TX_SIZE")]
pub type UartTxSize = crate::Reg<uart_tx_size::UartTxSizeSpec>;
#[doc = "uDMA TX UART buffer size configuration register"]
pub mod uart_tx_size;
#[doc = "UART_TX_CFG (rw) register accessor: uDMA TX UART stream configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_cfg`]
module"]
#[doc(alias = "UART_TX_CFG")]
pub type UartTxCfg = crate::Reg<uart_tx_cfg::UartTxCfgSpec>;
#[doc = "uDMA TX UART stream configuration register."]
pub mod uart_tx_cfg;
#[doc = "UART_ERROR (r) register accessor: uDMA UART Error status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_error::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_error`]
module"]
#[doc(alias = "UART_ERROR")]
pub type UartError = crate::Reg<uart_error::UartErrorSpec>;
#[doc = "uDMA UART Error status"]
pub mod uart_error;
#[doc = "SPIM_RX_SIZE (rw) register accessor: RX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_size`]
module"]
#[doc(alias = "SPIM_RX_SIZE")]
pub type SpimRxSize = crate::Reg<spim_rx_size::SpimRxSizeSpec>;
#[doc = "RX SPI uDMA transfer size of buffer"]
pub mod spim_rx_size;
#[doc = "UART_DATA (r) register accessor: RX read data for polling or interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_data`]
module"]
#[doc(alias = "UART_DATA")]
pub type UartData = crate::Reg<uart_data::UartDataSpec>;
#[doc = "RX read data for polling or interrupt"]
pub mod uart_data;
#[doc = "SPIM_RX_SADDR (rw) register accessor: RX SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_saddr`]
module"]
#[doc(alias = "SPIM_RX_SADDR")]
pub type SpimRxSaddr = crate::Reg<spim_rx_saddr::SpimRxSaddrSpec>;
#[doc = "RX SPI uDMA transfer address of associated buffer"]
pub mod spim_rx_saddr;
#[doc = "UART_STATUS (r) register accessor: uDMA UART status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_status`]
module"]
#[doc(alias = "UART_STATUS")]
pub type UartStatus = crate::Reg<uart_status::UartStatusSpec>;
#[doc = "uDMA UART status register"]
pub mod uart_status;
#[doc = "UART_IRQ_EN (rw) register accessor: uDMA UART Read or Error interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_irq_en`]
module"]
#[doc(alias = "UART_IRQ_EN")]
pub type UartIrqEn = crate::Reg<uart_irq_en::UartIrqEnSpec>;
#[doc = "uDMA UART Read or Error interrupt enable register"]
pub mod uart_irq_en;
#[doc = "UART_VALID (r) register accessor: uDMA UART Read polling data valid flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_valid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_valid`]
module"]
#[doc(alias = "UART_VALID")]
pub type UartValid = crate::Reg<uart_valid::UartValidSpec>;
#[doc = "uDMA UART Read polling data valid flag register"]
pub mod uart_valid;
#[doc = "SPIM_CMD_SADDR (rw) register accessor: CMD SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_saddr`]
module"]
#[doc(alias = "SPIM_CMD_SADDR")]
pub type SpimCmdSaddr = crate::Reg<spim_cmd_saddr::SpimCmdSaddrSpec>;
#[doc = "CMD SPI uDMA transfer address of associated buffer"]
pub mod spim_cmd_saddr;
#[doc = "SPIM_TX_SADDR (rw) register accessor: TX SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_saddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_saddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_saddr`]
module"]
#[doc(alias = "SPIM_TX_SADDR")]
pub type SpimTxSaddr = crate::Reg<spim_tx_saddr::SpimTxSaddrSpec>;
#[doc = "TX SPI uDMA transfer address of associated buffer"]
pub mod spim_tx_saddr;
#[doc = "SPIM_CMD_SIZE (rw) register accessor: CMD SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_size`]
module"]
#[doc(alias = "SPIM_CMD_SIZE")]
pub type SpimCmdSize = crate::Reg<spim_cmd_size::SpimCmdSizeSpec>;
#[doc = "CMD SPI uDMA transfer size of buffer"]
pub mod spim_cmd_size;
#[doc = "SPIM_TX_SIZE (rw) register accessor: TX SPI uDMA transfer size of buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_size`]
module"]
#[doc(alias = "SPIM_TX_SIZE")]
pub type SpimTxSize = crate::Reg<spim_tx_size::SpimTxSizeSpec>;
#[doc = "TX SPI uDMA transfer size of buffer"]
pub mod spim_tx_size;
#[doc = "SPIM_CMD_CFG (rw) register accessor: CMD SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_cmd_cfg`]
module"]
#[doc(alias = "SPIM_CMD_CFG")]
pub type SpimCmdCfg = crate::Reg<spim_cmd_cfg::SpimCmdCfgSpec>;
#[doc = "CMD SPI uDMA transfer configuration"]
pub mod spim_cmd_cfg;
#[doc = "UART_SETUP (rw) register accessor: UDMA UART configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_setup`]
module"]
#[doc(alias = "UART_SETUP")]
pub type UartSetup = crate::Reg<uart_setup::UartSetupSpec>;
#[doc = "UDMA UART configuration register."]
pub mod uart_setup;
#[doc = "SPIM_TX_CFG (rw) register accessor: TX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_tx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_tx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_tx_cfg`]
module"]
#[doc(alias = "SPIM_TX_CFG")]
pub type SpimTxCfg = crate::Reg<spim_tx_cfg::SpimTxCfgSpec>;
#[doc = "TX SPI uDMA transfer configuration"]
pub mod spim_tx_cfg;
#[doc = "SPIM_RX_CFG (rw) register accessor: RX SPI uDMA transfer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_rx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_rx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spim_rx_cfg`]
module"]
#[doc(alias = "SPIM_RX_CFG")]
pub type SpimRxCfg = crate::Reg<spim_rx_cfg::SpimRxCfgSpec>;
#[doc = "RX SPI uDMA transfer configuration"]
pub mod spim_rx_cfg;
