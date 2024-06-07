#[doc = "Register `timer2_prio[%s]` reader"]
pub type R = crate::R<Timer2PrioSpec>;
#[doc = "Register `timer2_prio[%s]` writer"]
pub type W = crate::W<Timer2PrioSpec>;
#[doc = "Field `prio` reader - "]
pub type PrioR = crate::FieldReader<u32>;
#[doc = "Field `prio` writer - "]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer2_prio")
            .field("prio", &self.prio())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Timer2PrioSpec> {
        PrioW::new(self, 0)
    }
}
#[doc = "Interrupt priority for timer 2 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_prio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_prio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2PrioSpec;
impl crate::RegisterSpec for Timer2PrioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_prio::R`](R) reader structure"]
impl crate::Readable for Timer2PrioSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2_prio::W`](W) writer structure"]
impl crate::Writable for Timer2PrioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer2_prio[%s]
to value 0"]
impl crate::Resettable for Timer2PrioSpec {
    const RESET_VALUE: u32 = 0;
}
