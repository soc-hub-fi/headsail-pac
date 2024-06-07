#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    paddir: Paddir,
    padin: Padin,
    padout: Padout,
    inten: Inten,
    inttype0: Inttype0,
    inttype1: Inttype1,
    intstatus: Intstatus,
    powerevent: Powerevent,
    padcfg0: Padcfg0,
    padcfg1: Padcfg1,
    padcfg2: Padcfg2,
    padcfg3: Padcfg3,
    padcfg4: Padcfg4,
    padcfg5: Padcfg5,
    padcfg6: Padcfg6,
    padcfg7: Padcfg7,
}
impl RegisterBlock {
    #[doc = "0x00 - Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
    #[inline(always)]
    pub const fn paddir(&self) -> &Paddir {
        &self.paddir
    }
    #[doc = "0x04 - Input Values."]
    #[inline(always)]
    pub const fn padin(&self) -> &Padin {
        &self.padin
    }
    #[doc = "0x08 - Output Values."]
    #[inline(always)]
    pub const fn padout(&self) -> &Padout {
        &self.padout
    }
    #[doc = "0x0c - Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x10 - Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
    #[inline(always)]
    pub const fn inttype0(&self) -> &Inttype0 {
        &self.inttype0
    }
    #[doc = "0x14 - Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
    #[inline(always)]
    pub const fn inttype1(&self) -> &Inttype1 {
        &self.inttype1
    }
    #[doc = "0x18 - Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn powerevent(&self) -> &Powerevent {
        &self.powerevent
    }
    #[doc = "0x20 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg0(&self) -> &Padcfg0 {
        &self.padcfg0
    }
    #[doc = "0x24 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg1(&self) -> &Padcfg1 {
        &self.padcfg1
    }
    #[doc = "0x28 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg2(&self) -> &Padcfg2 {
        &self.padcfg2
    }
    #[doc = "0x2c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg3(&self) -> &Padcfg3 {
        &self.padcfg3
    }
    #[doc = "0x30 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg4(&self) -> &Padcfg4 {
        &self.padcfg4
    }
    #[doc = "0x34 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg5(&self) -> &Padcfg5 {
        &self.padcfg5
    }
    #[doc = "0x38 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg6(&self) -> &Padcfg6 {
        &self.padcfg6
    }
    #[doc = "0x3c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    #[inline(always)]
    pub const fn padcfg7(&self) -> &Padcfg7 {
        &self.padcfg7
    }
}
#[doc = "PADDIR (rw) register accessor: Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddir`]
module"]
#[doc(alias = "PADDIR")]
pub type Paddir = crate::Reg<paddir::PaddirSpec>;
#[doc = "Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
pub mod paddir;
#[doc = "PADIN (w) register accessor: Input Values.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padin`]
module"]
#[doc(alias = "PADIN")]
pub type Padin = crate::Reg<padin::PadinSpec>;
#[doc = "Input Values."]
pub mod padin;
#[doc = "PADOUT (rw) register accessor: Output Values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padout`]
module"]
#[doc(alias = "PADOUT")]
pub type Padout = crate::Reg<padout::PadoutSpec>;
#[doc = "Output Values."]
pub mod padout;
#[doc = "INTEN (rw) register accessor: Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
pub mod inten;
#[doc = "INTTYPE0 (rw) register accessor: Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype0`]
module"]
#[doc(alias = "INTTYPE0")]
pub type Inttype0 = crate::Reg<inttype0::Inttype0Spec>;
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
pub mod inttype0;
#[doc = "INTTYPE1 (rw) register accessor: Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype1`]
module"]
#[doc(alias = "INTTYPE1")]
pub type Inttype1 = crate::Reg<inttype1::Inttype1Spec>;
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
pub mod inttype1;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
pub mod intstatus;
#[doc = "POWEREVENT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerevent::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`powerevent::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerevent`]
module"]
#[doc(alias = "POWEREVENT")]
pub type Powerevent = crate::Reg<powerevent::PowereventSpec>;
#[doc = ""]
pub mod powerevent;
#[doc = "PADCFG0 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg0`]
module"]
#[doc(alias = "PADCFG0")]
pub type Padcfg0 = crate::Reg<padcfg0::Padcfg0Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg0;
#[doc = "PADCFG1 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg1`]
module"]
#[doc(alias = "PADCFG1")]
pub type Padcfg1 = crate::Reg<padcfg1::Padcfg1Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg1;
#[doc = "PADCFG2 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg2`]
module"]
#[doc(alias = "PADCFG2")]
pub type Padcfg2 = crate::Reg<padcfg2::Padcfg2Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg2;
#[doc = "PADCFG3 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg3`]
module"]
#[doc(alias = "PADCFG3")]
pub type Padcfg3 = crate::Reg<padcfg3::Padcfg3Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg3;
#[doc = "PADCFG4 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg4`]
module"]
#[doc(alias = "PADCFG4")]
pub type Padcfg4 = crate::Reg<padcfg4::Padcfg4Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg4;
#[doc = "PADCFG5 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg5`]
module"]
#[doc(alias = "PADCFG5")]
pub type Padcfg5 = crate::Reg<padcfg5::Padcfg5Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg5;
#[doc = "PADCFG6 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg6`]
module"]
#[doc(alias = "PADCFG6")]
pub type Padcfg6 = crate::Reg<padcfg6::Padcfg6Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg6;
#[doc = "PADCFG7 (rw) register accessor: Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg7`]
module"]
#[doc(alias = "PADCFG7")]
pub type Padcfg7 = crate::Reg<padcfg7::Padcfg7Spec>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg7;
