#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: STATUS,
    clkdiv: CLKDIV,
    spicmd: SPICMD,
    spiadr: SPIADR,
    spilen: SPILEN,
    spidum: SPIDUM,
    txfifo: TXFIFO,
    _reserved7: [u8; 0x04],
    rxfifo: RXFIFO,
    intcfg: INTCFG,
    intsta: INTSTA,
}
impl RegisterBlock {
    #[doc = "0x00 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x04 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    #[doc = "0x08 - SPI Command"]
    #[inline(always)]
    pub const fn spicmd(&self) -> &SPICMD {
        &self.spicmd
    }
    #[doc = "0x0c - SPI Address"]
    #[inline(always)]
    pub const fn spiadr(&self) -> &SPIADR {
        &self.spiadr
    }
    #[doc = "0x10 - SPI Transfer Length"]
    #[inline(always)]
    pub const fn spilen(&self) -> &SPILEN {
        &self.spilen
    }
    #[doc = "0x14 - SPI Dummy Cycles"]
    #[inline(always)]
    pub const fn spidum(&self) -> &SPIDUM {
        &self.spidum
    }
    #[doc = "0x18 - SPI Transmit FIFO"]
    #[inline(always)]
    pub const fn txfifo(&self) -> &TXFIFO {
        &self.txfifo
    }
    #[doc = "0x20 - SPI Receive FIFO"]
    #[inline(always)]
    pub const fn rxfifo(&self) -> &RXFIFO {
        &self.rxfifo
    }
    #[doc = "0x24 - Interrupt Configuration"]
    #[inline(always)]
    pub const fn intcfg(&self) -> &INTCFG {
        &self.intcfg
    }
    #[doc = "0x28 - This register isn't properly specified so we need to look at this"]
    #[inline(always)]
    pub const fn intsta(&self) -> &INTSTA {
        &self.intsta
    }
}
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider"]
pub mod clkdiv;
#[doc = "SPICMD (rw) register accessor: SPI Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spicmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spicmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spicmd`]
module"]
pub type SPICMD = crate::Reg<spicmd::SPICMD_SPEC>;
#[doc = "SPI Command"]
pub mod spicmd;
#[doc = "SPIADR (rw) register accessor: SPI Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spiadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spiadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spiadr`]
module"]
pub type SPIADR = crate::Reg<spiadr::SPIADR_SPEC>;
#[doc = "SPI Address"]
pub mod spiadr;
#[doc = "SPILEN (rw) register accessor: SPI Transfer Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spilen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spilen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spilen`]
module"]
pub type SPILEN = crate::Reg<spilen::SPILEN_SPEC>;
#[doc = "SPI Transfer Length"]
pub mod spilen;
#[doc = "SPIDUM (rw) register accessor: SPI Dummy Cycles\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spidum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spidum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spidum`]
module"]
pub type SPIDUM = crate::Reg<spidum::SPIDUM_SPEC>;
#[doc = "SPI Dummy Cycles"]
pub mod spidum;
#[doc = "TXFIFO (w) register accessor: SPI Transmit FIFO\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo`]
module"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "SPI Transmit FIFO"]
pub mod txfifo;
#[doc = "RXFIFO (r) register accessor: SPI Receive FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo`]
module"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "SPI Receive FIFO"]
pub mod rxfifo;
#[doc = "INTCFG (rw) register accessor: Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intcfg`]
module"]
pub type INTCFG = crate::Reg<intcfg::INTCFG_SPEC>;
#[doc = "Interrupt Configuration"]
pub mod intcfg;
#[doc = "INTSTA (r) register accessor: This register isn't properly specified so we need to look at this\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsta`]
module"]
pub type INTSTA = crate::Reg<intsta::INTSTA_SPEC>;
#[doc = "This register isn't properly specified so we need to look at this"]
pub mod intsta;
