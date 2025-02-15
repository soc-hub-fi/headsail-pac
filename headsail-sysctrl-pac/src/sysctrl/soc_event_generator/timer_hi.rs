#[doc = "Register `TIMER_HI` reader"]
pub type R = crate::R<TimerHiSpec>;
#[doc = "Register `TIMER_HI` writer"]
pub type W = crate::W<TimerHiSpec>;
#[doc = "Field `TIMER_HI_EVENT` reader - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
pub type TimerHiEventR = crate::FieldReader;
#[doc = "Field `TIMER_HI_EVENT` writer - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
pub type TimerHiEventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
    #[inline(always)]
    pub fn timer_hi_event(&self) -> TimerHiEventR {
        TimerHiEventR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_HI")
            .field("timer_hi_event", &self.timer_hi_event())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger and start APB Timer HI by the event with id that equals TIMER_HI_EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn timer_hi_event(&mut self) -> TimerHiEventW<TimerHiSpec> {
        TimerHiEventW::new(self, 0)
    }
}
#[doc = "Trigger Timer HI of APB Timer with event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerHiSpec;
impl crate::RegisterSpec for TimerHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_hi::R`](R) reader structure"]
impl crate::Readable for TimerHiSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_hi::W`](W) writer structure"]
impl crate::Writable for TimerHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_HI to value 0"]
impl crate::Resettable for TimerHiSpec {
    const RESET_VALUE: u32 = 0;
}
