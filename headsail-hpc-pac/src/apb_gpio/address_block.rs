#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "addressBlock"]
#[doc(alias = "addressBlock")]
pub struct ADDRESS_BLOCK {
    paddir: PADDIR,
    padin: PADIN,
    padout: PADOUT,
    inten: INTEN,
    inttype0: INTTYPE0,
    inttype1: INTTYPE1,
    intstatus: INTSTATUS,
    powerevent: POWEREVENT,
    padcfg0: PADCFG0,
    padcfg1: PADCFG1,
    padcfg2: PADCFG2,
    padcfg3: PADCFG3,
    padcfg4: PADCFG4,
    padcfg5: PADCFG5,
    padcfg6: PADCFG6,
    padcfg7: PADCFG7,
}
impl ADDRESS_BLOCK {
    #[doc = "0x00 - Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
    #[inline(always)]
    pub const fn paddir(&self) -> &PADDIR {
        &self.paddir
    }
    #[doc = "0x04 - Input Values."]
    #[inline(always)]
    pub const fn padin(&self) -> &PADIN {
        &self.padin
    }
    #[doc = "0x08 - Output Values."]
    #[inline(always)]
    pub const fn padout(&self) -> &PADOUT {
        &self.padout
    }
    #[doc = "0x0c - Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x10 - Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
    #[inline(always)]
    pub const fn inttype0(&self) -> &INTTYPE0 {
        &self.inttype0
    }
    #[doc = "0x14 - Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
    #[inline(always)]
    pub const fn inttype1(&self) -> &INTTYPE1 {
        &self.inttype1
    }
    #[doc = "0x18 - Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
    #[inline(always)]
    pub const fn intstatus(&self) -> &INTSTATUS {
        &self.intstatus
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn powerevent(&self) -> &POWEREVENT {
        &self.powerevent
    }
    #[doc = "0x20 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg0(&self) -> &PADCFG0 {
        &self.padcfg0
    }
    #[doc = "0x24 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg1(&self) -> &PADCFG1 {
        &self.padcfg1
    }
    #[doc = "0x28 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg2(&self) -> &PADCFG2 {
        &self.padcfg2
    }
    #[doc = "0x2c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg3(&self) -> &PADCFG3 {
        &self.padcfg3
    }
    #[doc = "0x30 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg4(&self) -> &PADCFG4 {
        &self.padcfg4
    }
    #[doc = "0x34 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg5(&self) -> &PADCFG5 {
        &self.padcfg5
    }
    #[doc = "0x38 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg6(&self) -> &PADCFG6 {
        &self.padcfg6
    }
    #[doc = "0x3c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg7(&self) -> &PADCFG7 {
        &self.padcfg7
    }
}
#[doc = "PADDIR (rw) register accessor: Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddir`]
module"]
pub type PADDIR = crate::Reg<paddir::PADDIR_SPEC>;
#[doc = "Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
pub mod paddir;
#[doc = "PADIN (w) register accessor: Input Values.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padin`]
module"]
pub type PADIN = crate::Reg<padin::PADIN_SPEC>;
#[doc = "Input Values."]
pub mod padin;
#[doc = "PADOUT (rw) register accessor: Output Values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padout`]
module"]
pub type PADOUT = crate::Reg<padout::PADOUT_SPEC>;
#[doc = "Output Values."]
pub mod padout;
#[doc = "INTEN (rw) register accessor: Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
pub mod inten;
#[doc = "INTTYPE0 (rw) register accessor: Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype0`]
module"]
pub type INTTYPE0 = crate::Reg<inttype0::INTTYPE0_SPEC>;
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
pub mod inttype0;
#[doc = "INTTYPE1 (rw) register accessor: Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype1`]
module"]
pub type INTTYPE1 = crate::Reg<inttype1::INTTYPE1_SPEC>;
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
pub mod inttype1;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
pub mod intstatus;
#[doc = "POWEREVENT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerevent::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerevent::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerevent`]
module"]
pub type POWEREVENT = crate::Reg<powerevent::POWEREVENT_SPEC>;
#[doc = ""]
pub mod powerevent;
#[doc = "PADCFG0 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg0`]
module"]
pub type PADCFG0 = crate::Reg<padcfg0::PADCFG0_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg0;
#[doc = "PADCFG1 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg1`]
module"]
pub type PADCFG1 = crate::Reg<padcfg1::PADCFG1_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg1;
#[doc = "PADCFG2 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg2`]
module"]
pub type PADCFG2 = crate::Reg<padcfg2::PADCFG2_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg2;
#[doc = "PADCFG3 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg3`]
module"]
pub type PADCFG3 = crate::Reg<padcfg3::PADCFG3_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg3;
#[doc = "PADCFG4 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg4`]
module"]
pub type PADCFG4 = crate::Reg<padcfg4::PADCFG4_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg4;
#[doc = "PADCFG5 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg5`]
module"]
pub type PADCFG5 = crate::Reg<padcfg5::PADCFG5_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg5;
#[doc = "PADCFG6 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg6`]
module"]
pub type PADCFG6 = crate::Reg<padcfg6::PADCFG6_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg6;
#[doc = "PADCFG7 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg7`]
module"]
pub type PADCFG7 = crate::Reg<padcfg7::PADCFG7_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg7;
