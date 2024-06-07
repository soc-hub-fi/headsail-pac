#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "l2_config"]
#[doc(alias = "l2_config")]
pub struct L2Config {
    en: En,
}
impl L2Config {
    #[doc = "0x00..0x08 - Enable L2 cache"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
}
#[doc = "en (rw) register accessor: Enable L2 cache\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "en")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Enable L2 cache"]
pub mod en;
