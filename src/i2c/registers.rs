#[doc = r"Register block"]
#[repr(C)]
pub struct REGISTERS {
    #[doc = "0x00 - Clock Prescale Register"]
    pub clk_prescaler: CLK_PRESCALER,
    #[doc = "0x04 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Receive Register"]
    pub rx: RX,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Transmit Register"]
    pub tx: TX,
    #[doc = "0x14 - Command Register"]
    pub cmd: CMD,
}
#[doc = "CLK_PRESCALER (rw) register accessor: an alias for `Reg<CLK_PRESCALER_SPEC>`"]
pub type CLK_PRESCALER = crate::Reg<clk_prescaler::CLK_PRESCALER_SPEC>;
#[doc = "Clock Prescale Register"]
pub mod clk_prescaler;
#[doc = "RX (r) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "Receive Register"]
pub mod rx;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "TX (rw) register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "Transmit Register"]
pub mod tx;
