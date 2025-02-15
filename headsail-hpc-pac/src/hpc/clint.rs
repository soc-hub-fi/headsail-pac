#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "clint"]
#[doc(alias = "clint")]
pub struct Clint {
    msip: [Msip; 4],
    _reserved1: [u8; 0x3fe0],
    mtimecmp: [Mtimecmp; 4],
    _reserved2: [u8; 0x7fd8],
    mtime: Mtime,
}
impl Clint {
    #[doc = "0x00..0x20 - Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register."]
    #[inline(always)]
    pub const fn msip(&self, n: usize) -> &Msip {
        &self.msip[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register."]
    #[inline(always)]
    pub fn msip_iter(&self) -> impl Iterator<Item = &Msip> {
        self.msip.iter()
    }
    #[doc = "0x4000..0x4020 - Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
    #[inline(always)]
    pub const fn mtimecmp(&self, n: usize) -> &Mtimecmp {
        &self.mtimecmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4000..0x4020 - Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
    #[inline(always)]
    pub fn mtimecmp_iter(&self) -> impl Iterator<Item = &Mtimecmp> {
        self.mtimecmp.iter()
    }
    #[doc = "0xbff8..0xc000 - Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
}
#[doc = "mtimecmp (rw) register accessor: Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
#[doc(alias = "mtimecmp")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
#[doc = "Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
pub mod mtimecmp;
#[doc = "msip (rw) register accessor: Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "msip")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register."]
pub mod msip;
#[doc = "mtime (rw) register accessor: Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
#[doc(alias = "mtime")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
#[doc = "Cycle counter. A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register."]
pub mod mtime;
