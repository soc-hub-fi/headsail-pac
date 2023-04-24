#[doc = r"Register block"]
#[repr(C)]
pub struct CLINT {
    #[doc = "0x00..0x20 - Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register."]
    pub msip: [MSIP; 4],
    _reserved1: [u8; 0x3fe0],
    #[doc = "0x4000..0x4020 - Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
    pub mtimecmp: [MTIMECMP; 4],
    _reserved2: [u8; 0x7fd8],
    #[doc = "0xbff8..0xc000 - Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
    pub mtime: MTIME,
}
#[doc = "mtimecmp (rw) register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
pub mod mtimecmp;
#[doc = "msip (rw) register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register."]
pub mod msip;
#[doc = "mtime (rw) register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
pub mod mtime;
