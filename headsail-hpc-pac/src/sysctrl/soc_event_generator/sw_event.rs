#[doc = "Register `SW_EVENT` writer"]
pub type W = crate::W<SW_EVENT_SPEC>;
#[doc = "Field `EVENT` writer - Writing a one-hot value into EVENT triggers a SoC software event. 8 software events are available."]
pub type EVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_EVENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing a one-hot value into EVENT triggers a SoC software event. 8 software events are available."]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EVENT_W<SW_EVENT_SPEC> {
        EVENT_W::new(self, 0)
    }
}
#[doc = "SoC software events trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_event::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_EVENT_SPEC;
impl crate::RegisterSpec for SW_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sw_event::W`](W) writer structure"]
impl crate::Writable for SW_EVENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_EVENT to value 0"]
impl crate::Resettable for SW_EVENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
