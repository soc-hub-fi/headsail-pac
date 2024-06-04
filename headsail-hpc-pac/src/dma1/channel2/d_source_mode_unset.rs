#[doc = "Register `D_SOURCE_MODE_UNSET` writer"]
pub type W = crate::W<D_SOURCE_MODE_UNSET_SPEC>;
#[doc = "Field `Source_Mode_Unset` writer - "]
pub type SOURCE_MODE_UNSET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<D_SOURCE_MODE_UNSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_mode_unset(&mut self) -> SOURCE_MODE_UNSET_W<D_SOURCE_MODE_UNSET_SPEC> {
        SOURCE_MODE_UNSET_W::new(self, 0)
    }
}
#[doc = "Write to unset source mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_source_mode_unset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D_SOURCE_MODE_UNSET_SPEC;
impl crate::RegisterSpec for D_SOURCE_MODE_UNSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`d_source_mode_unset::W`](W) writer structure"]
impl crate::Writable for D_SOURCE_MODE_UNSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D_SOURCE_MODE_UNSET to value 0"]
impl crate::Resettable for D_SOURCE_MODE_UNSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
