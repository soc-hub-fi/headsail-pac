#[doc = "Register `timer1_timer` reader"]
pub type R = crate::R<Timer1TimerSpec>;
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
        f.debug_struct("timer1_timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[doc = "Monotonically increasing timer register for timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_timer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1TimerSpec;
impl crate::RegisterSpec for Timer1TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1_timer::R`](R) reader structure"]
impl crate::Readable for Timer1TimerSpec {}
#[doc = "`reset()` method sets timer1_timer to value 0"]
impl crate::Resettable for Timer1TimerSpec {
    const RESET_VALUE: u32 = 0;
}
