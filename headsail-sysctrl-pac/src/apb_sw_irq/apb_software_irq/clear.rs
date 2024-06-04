#[doc = "Register `clear` writer"]
pub type W = crate::W<CLEAR_SPEC>;
#[doc = "Field `clear` writer - "]
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CLEAR_SPEC> {
        CLEAR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clear to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
