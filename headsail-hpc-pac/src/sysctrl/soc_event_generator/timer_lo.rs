#[doc = "Register `TIMER_LO` reader"]
pub type R = crate::R<TimerLoSpec>;
#[doc = "Register `TIMER_LO` writer"]
pub type W = crate::W<TimerLoSpec>;
#[doc = "Field `TIMER_LO_EVENT` reader - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
pub type TimerLoEventR = crate::FieldReader;
#[doc = "Field `TIMER_LO_EVENT` writer - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
pub type TimerLoEventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
    #[inline(always)]
    pub fn timer_lo_event(&self) -> TimerLoEventR {
        TimerLoEventR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_LO")
            .field("timer_lo_event", &self.timer_lo_event())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger and start APB Timer LO by the event with id that equals TIMER_LO_EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn timer_lo_event(&mut self) -> TimerLoEventW<TimerLoSpec> {
        TimerLoEventW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerLoSpec;
impl crate::RegisterSpec for TimerLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_lo::R`](R) reader structure"]
impl crate::Readable for TimerLoSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_lo::W`](W) writer structure"]
impl crate::Writable for TimerLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_LO to value 0"]
impl crate::Resettable for TimerLoSpec {
    const RESET_VALUE: u32 = 0;
}
