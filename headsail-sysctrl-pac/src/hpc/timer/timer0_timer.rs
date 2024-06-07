#[doc = "Register `timer0_timer` reader"]
pub type R = crate::R<Timer0TimerSpec>;
#[doc = "Field `timer` reader - "]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer0_timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[doc = "Monotonically increasing timer register for timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_timer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0TimerSpec;
impl crate::RegisterSpec for Timer0TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_timer::R`](R) reader structure"]
impl crate::Readable for Timer0TimerSpec {}
#[doc = "`reset()` method sets timer0_timer to value 0"]
impl crate::Resettable for Timer0TimerSpec {
    const RESET_VALUE: u32 = 0;
}
