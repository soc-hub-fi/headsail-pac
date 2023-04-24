#[doc = r"Register block"]
#[repr(C)]
pub struct ADDRESS_BLOCK {
    #[doc = "0x00 - Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
    pub paddir: PADDIR,
    #[doc = "0x04 - Input Values."]
    pub padin: PADIN,
    #[doc = "0x08 - Output Values."]
    pub padout: PADOUT,
    #[doc = "0x0c - Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
    pub inten: INTEN,
    #[doc = "0x10 - Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
    pub inttype0: INTTYPE0,
    #[doc = "0x14 - Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
    pub inttype1: INTTYPE1,
    #[doc = "0x18 - Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
    pub intstatus: INTSTATUS,
    #[doc = "0x1c - "]
    pub powerevent: POWEREVENT,
    #[doc = "0x20 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg0: PADCFG0,
    #[doc = "0x24 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg1: PADCFG1,
    #[doc = "0x28 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg2: PADCFG2,
    #[doc = "0x2c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg3: PADCFG3,
    #[doc = "0x30 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg4: PADCFG4,
    #[doc = "0x34 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg5: PADCFG5,
    #[doc = "0x38 - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg6: PADCFG6,
    #[doc = "0x3c - Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
    pub padcfg7: PADCFG7,
}
#[doc = "PADDIR (rw) register accessor: an alias for `Reg<PADDIR_SPEC>`"]
pub type PADDIR = crate::Reg<paddir::PADDIR_SPEC>;
#[doc = "Pad Direction. Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
pub mod paddir;
#[doc = "PADIN (w) register accessor: an alias for `Reg<PADIN_SPEC>`"]
pub type PADIN = crate::Reg<padin::PADIN_SPEC>;
#[doc = "Input Values."]
pub mod padin;
#[doc = "PADOUT (rw) register accessor: an alias for `Reg<PADOUT_SPEC>`"]
pub type PADOUT = crate::Reg<padout::PADOUT_SPEC>;
#[doc = "Output Values."]
pub mod padout;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable. Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior."]
pub mod inten;
#[doc = "INTTYPE0 (rw) register accessor: an alias for `Reg<INTTYPE0_SPEC>`"]
pub type INTTYPE0 = crate::Reg<inttype0::INTTYPE0_SPEC>;
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
pub mod inttype0;
#[doc = "INTTYPE1 (rw) register accessor: an alias for `Reg<INTTYPE1_SPEC>`"]
pub type INTTYPE1 = crate::Reg<inttype1::INTTYPE1_SPEC>;
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
pub mod inttype1;
#[doc = "INTSTATUS (r) register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status. Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
pub mod intstatus;
#[doc = "POWEREVENT (rw) register accessor: an alias for `Reg<POWEREVENT_SPEC>`"]
pub type POWEREVENT = crate::Reg<powerevent::POWEREVENT_SPEC>;
#[doc = ""]
pub mod powerevent;
#[doc = "PADCFG0 (rw) register accessor: an alias for `Reg<PADCFG0_SPEC>`"]
pub type PADCFG0 = crate::Reg<padcfg0::PADCFG0_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg0;
#[doc = "PADCFG1 (rw) register accessor: an alias for `Reg<PADCFG1_SPEC>`"]
pub type PADCFG1 = crate::Reg<padcfg1::PADCFG1_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg1;
#[doc = "PADCFG2 (rw) register accessor: an alias for `Reg<PADCFG2_SPEC>`"]
pub type PADCFG2 = crate::Reg<padcfg2::PADCFG2_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg2;
#[doc = "PADCFG3 (rw) register accessor: an alias for `Reg<PADCFG3_SPEC>`"]
pub type PADCFG3 = crate::Reg<padcfg3::PADCFG3_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg3;
#[doc = "PADCFG4 (rw) register accessor: an alias for `Reg<PADCFG4_SPEC>`"]
pub type PADCFG4 = crate::Reg<padcfg4::PADCFG4_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg4;
#[doc = "PADCFG5 (rw) register accessor: an alias for `Reg<PADCFG5_SPEC>`"]
pub type PADCFG5 = crate::Reg<padcfg5::PADCFG5_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg5;
#[doc = "PADCFG6 (rw) register accessor: an alias for `Reg<PADCFG6_SPEC>`"]
pub type PADCFG6 = crate::Reg<padcfg6::PADCFG6_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg6;
#[doc = "PADCFG7 (rw) register accessor: an alias for `Reg<PADCFG7_SPEC>`"]
pub type PADCFG7 = crate::Reg<padcfg7::PADCFG7_SPEC>;
#[doc = "Pad Configuration Registers. The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded."]
pub mod padcfg7;
