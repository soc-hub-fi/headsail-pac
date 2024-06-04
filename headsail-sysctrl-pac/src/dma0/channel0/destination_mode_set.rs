#[doc = "Register `DESTINATION_MODE_SET` writer"]
pub type W = crate::W<DESTINATION_MODE_SET_SPEC>;
#[doc = "Field `Destination_Mode_Set` writer - "]
pub type DESTINATION_MODE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTINATION_MODE_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode_set(&mut self) -> DESTINATION_MODE_SET_W<DESTINATION_MODE_SET_SPEC> {
        DESTINATION_MODE_SET_W::new(self, 0)
    }
}
#[doc = "Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTINATION_MODE_SET_SPEC;
impl crate::RegisterSpec for DESTINATION_MODE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`destination_mode_set::W`](W) writer structure"]
impl crate::Writable for DESTINATION_MODE_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESTINATION_MODE_SET to value 0"]
impl crate::Resettable for DESTINATION_MODE_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
