#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "registers"]
#[doc(alias = "registers")]
pub struct REGISTERS {
    clk_prescaler: CLK_PRESCALER,
    ctrl: CTRL,
    rx: RX,
    status: STATUS,
    tx: TX,
    cmd: CMD,
}
impl REGISTERS {
    #[doc = "0x00 - Clock Prescale Register"]
    #[inline(always)]
    pub const fn clk_prescaler(&self) -> &CLK_PRESCALER {
        &self.clk_prescaler
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x08 - Receive Register"]
    #[inline(always)]
    pub const fn rx(&self) -> &RX {
        &self.rx
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x10 - Transmit Register"]
    #[inline(always)]
    pub const fn tx(&self) -> &TX {
        &self.tx
    }
    #[doc = "0x14 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
}
#[doc = "CLK_PRESCALER (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_prescaler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_prescaler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_prescaler`]
module"]
pub type CLK_PRESCALER = crate::Reg<clk_prescaler::CLK_PRESCALER_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clk_prescaler;
#[doc = "RX (r) register accessor: Receive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`]
module"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "Receive Register"]
pub mod rx;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "TX (rw) register accessor: Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx`]
module"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "Transmit Register"]
pub mod tx;
