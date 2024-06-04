#[doc = "Register `timer3_timer` reader"]
pub type R = crate::R<TIMER3_TIMER_SPEC>;
#[doc = "Field `timer` reader - "]
pub type TIMER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer3_timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[doc = "Monotonically increasing timer register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_timer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER3_TIMER_SPEC;
impl crate::RegisterSpec for TIMER3_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3_timer::R`](R) reader structure"]
impl crate::Readable for TIMER3_TIMER_SPEC {}
#[doc = "`reset()` method sets timer3_timer to value 0"]
impl crate::Resettable for TIMER3_TIMER_SPEC {
    const RESET_VALUE: u32 = 0;
}
