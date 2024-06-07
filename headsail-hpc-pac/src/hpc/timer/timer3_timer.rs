#[doc = "Register `timer3_timer` reader"]
pub type R = crate::R<Timer3TimerSpec>;
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
        f.debug_struct("timer3_timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[doc = "Monotonically increasing timer register for timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_timer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer3TimerSpec;
impl crate::RegisterSpec for Timer3TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3_timer::R`](R) reader structure"]
impl crate::Readable for Timer3TimerSpec {}
#[doc = "`reset()` method sets timer3_timer to value 0"]
impl crate::Resettable for Timer3TimerSpec {
    const RESET_VALUE: u32 = 0;
}
