#[doc = r"Register block"]
#[repr(C)]
pub struct REGISTERS {
    #[doc = "0x00 - Status Register"]
    pub status: STATUS,
    #[doc = "0x04 - Clock Divider"]
    pub clkdiv: CLKDIV,
    #[doc = "0x08 - SPI Command"]
    pub spicmd: SPICMD,
    #[doc = "0x0c - SPI Address"]
    pub spiadr: SPIADR,
    #[doc = "0x10 - SPI Transfer Length"]
    pub spilen: SPILEN,
    #[doc = "0x14 - SPI Dummy Cycles"]
    pub spidum: SPIDUM,
    #[doc = "0x18 - SPI Transmit FIFO"]
    pub txfifo: TXFIFO,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - SPI Receive FIFO"]
    pub rxfifo: RXFIFO,
    #[doc = "0x24 - Interrupt Configuration"]
    pub intcfg: INTCFG,
    #[doc = "0x28 - This register isn't properly specified so we need to look at this"]
    pub intsta: INTSTA,
}
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider"]
pub mod clkdiv;
#[doc = "SPICMD (rw) register accessor: an alias for `Reg<SPICMD_SPEC>`"]
pub type SPICMD = crate::Reg<spicmd::SPICMD_SPEC>;
#[doc = "SPI Command"]
pub mod spicmd;
#[doc = "SPIADR (rw) register accessor: an alias for `Reg<SPIADR_SPEC>`"]
pub type SPIADR = crate::Reg<spiadr::SPIADR_SPEC>;
#[doc = "SPI Address"]
pub mod spiadr;
#[doc = "SPILEN (rw) register accessor: an alias for `Reg<SPILEN_SPEC>`"]
pub type SPILEN = crate::Reg<spilen::SPILEN_SPEC>;
#[doc = "SPI Transfer Length"]
pub mod spilen;
#[doc = "SPIDUM (rw) register accessor: an alias for `Reg<SPIDUM_SPEC>`"]
pub type SPIDUM = crate::Reg<spidum::SPIDUM_SPEC>;
#[doc = "SPI Dummy Cycles"]
pub mod spidum;
#[doc = "TXFIFO (w) register accessor: an alias for `Reg<TXFIFO_SPEC>`"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "SPI Transmit FIFO"]
pub mod txfifo;
#[doc = "RXFIFO (r) register accessor: an alias for `Reg<RXFIFO_SPEC>`"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "SPI Receive FIFO"]
pub mod rxfifo;
#[doc = "INTCFG (rw) register accessor: an alias for `Reg<INTCFG_SPEC>`"]
pub type INTCFG = crate::Reg<intcfg::INTCFG_SPEC>;
#[doc = "Interrupt Configuration"]
pub mod intcfg;
#[doc = "INTSTA (r) register accessor: an alias for `Reg<INTSTA_SPEC>`"]
pub type INTSTA = crate::Reg<intsta::INTSTA_SPEC>;
#[doc = "This register isn't properly specified so we need to look at this"]
pub mod intsta;
