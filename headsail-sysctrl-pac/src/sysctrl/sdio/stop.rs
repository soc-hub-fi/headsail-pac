#[doc = "Register `STOP` writer"]
pub type W = crate::W<STOP_SPEC>;
#[doc = "Field `STOP` writer - "]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<STOP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<STOP_SPEC> {
        STOP_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STOP_SPEC;
impl crate::RegisterSpec for STOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stop::W`](W) writer structure"]
impl crate::Writable for STOP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for STOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
