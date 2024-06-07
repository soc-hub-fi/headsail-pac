#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_prescaler: ClkPrescaler,
    ctrl: Ctrl,
    rx: Rx,
    status: Status,
    tx: Tx,
    cmd: Cmd,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clk_prescaler(&self) -> &ClkPrescaler {
        &self.clk_prescaler
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Receive Register"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Transmit Register"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
    #[doc = "0x14 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
}
#[doc = "CLK_PRESCALER (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_prescaler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_prescaler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_prescaler`]
module"]
#[doc(alias = "CLK_PRESCALER")]
pub type ClkPrescaler = crate::Reg<clk_prescaler::ClkPrescalerSpec>;
#[doc = "Clock Prescale Register"]
pub mod clk_prescaler;
#[doc = "RX (r) register accessor: Receive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`]
module"]
#[doc(alias = "RX")]
pub type Rx = crate::Reg<rx::RxSpec>;
#[doc = "Receive Register"]
pub mod rx;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "TX (rw) register accessor: Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx`]
module"]
#[doc(alias = "TX")]
pub type Tx = crate::Reg<tx::TxSpec>;
#[doc = "Transmit Register"]
pub mod tx;
