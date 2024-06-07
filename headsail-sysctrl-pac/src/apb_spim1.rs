#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    clkdiv: Clkdiv,
    spicmd: Spicmd,
    spiadr: Spiadr,
    spilen: Spilen,
    spidum: Spidum,
    txfifo: Txfifo,
    _reserved7: [u8; 0x04],
    rxfifo: Rxfifo,
    intcfg: Intcfg,
    intsta: Intsta,
}
impl RegisterBlock {
    #[doc = "0x00 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x08 - SPI Command"]
    #[inline(always)]
    pub const fn spicmd(&self) -> &Spicmd {
        &self.spicmd
    }
    #[doc = "0x0c - SPI Address"]
    #[inline(always)]
    pub const fn spiadr(&self) -> &Spiadr {
        &self.spiadr
    }
    #[doc = "0x10 - SPI Transfer Length"]
    #[inline(always)]
    pub const fn spilen(&self) -> &Spilen {
        &self.spilen
    }
    #[doc = "0x14 - SPI Dummy Cycles"]
    #[inline(always)]
    pub const fn spidum(&self) -> &Spidum {
        &self.spidum
    }
    #[doc = "0x18 - SPI Transmit FIFO"]
    #[inline(always)]
    pub const fn txfifo(&self) -> &Txfifo {
        &self.txfifo
    }
    #[doc = "0x20 - SPI Receive FIFO"]
    #[inline(always)]
    pub const fn rxfifo(&self) -> &Rxfifo {
        &self.rxfifo
    }
    #[doc = "0x24 - Interrupt Configuration"]
    #[inline(always)]
    pub const fn intcfg(&self) -> &Intcfg {
        &self.intcfg
    }
    #[doc = "0x28 - This register isn't properly specified so we need to look at this"]
    #[inline(always)]
    pub const fn intsta(&self) -> &Intsta {
        &self.intsta
    }
}
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod clkdiv;
#[doc = "SPICMD (rw) register accessor: SPI Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spicmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spicmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spicmd`]
module"]
#[doc(alias = "SPICMD")]
pub type Spicmd = crate::Reg<spicmd::SpicmdSpec>;
#[doc = "SPI Command"]
pub mod spicmd;
#[doc = "SPIADR (rw) register accessor: SPI Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spiadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spiadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiadr`]
module"]
#[doc(alias = "SPIADR")]
pub type Spiadr = crate::Reg<spiadr::SpiadrSpec>;
#[doc = "SPI Address"]
pub mod spiadr;
#[doc = "SPILEN (rw) register accessor: SPI Transfer Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spilen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spilen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spilen`]
module"]
#[doc(alias = "SPILEN")]
pub type Spilen = crate::Reg<spilen::SpilenSpec>;
#[doc = "SPI Transfer Length"]
pub mod spilen;
#[doc = "SPIDUM (rw) register accessor: SPI Dummy Cycles\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spidum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spidum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidum`]
module"]
#[doc(alias = "SPIDUM")]
pub type Spidum = crate::Reg<spidum::SpidumSpec>;
#[doc = "SPI Dummy Cycles"]
pub mod spidum;
#[doc = "TXFIFO (w) register accessor: SPI Transmit FIFO\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo`]
module"]
#[doc(alias = "TXFIFO")]
pub type Txfifo = crate::Reg<txfifo::TxfifoSpec>;
#[doc = "SPI Transmit FIFO"]
pub mod txfifo;
#[doc = "RXFIFO (r) register accessor: SPI Receive FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo`]
module"]
#[doc(alias = "RXFIFO")]
pub type Rxfifo = crate::Reg<rxfifo::RxfifoSpec>;
#[doc = "SPI Receive FIFO"]
pub mod rxfifo;
#[doc = "INTCFG (rw) register accessor: Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcfg`]
module"]
#[doc(alias = "INTCFG")]
pub type Intcfg = crate::Reg<intcfg::IntcfgSpec>;
#[doc = "Interrupt Configuration"]
pub mod intcfg;
#[doc = "INTSTA (r) register accessor: This register isn't properly specified so we need to look at this\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsta`]
module"]
#[doc(alias = "INTSTA")]
pub type Intsta = crate::Reg<intsta::IntstaSpec>;
#[doc = "This register isn't properly specified so we need to look at this"]
pub mod intsta;
