#[doc = "Register `timer3_prio[%s]` reader"]
pub type R = crate::R<TIMER3_PRIO_SPEC>;
#[doc = "Register `timer3_prio[%s]` writer"]
pub type W = crate::W<TIMER3_PRIO_SPEC>;
#[doc = "Field `prio` reader - "]
pub type PRIO_R = crate::FieldReader<u32>;
#[doc = "Field `prio` writer - "]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer3_prio")
            .field("prio", &self.prio())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<TIMER3_PRIO_SPEC> {
        PRIO_W::new(self, 0)
    }
}
#[doc = "Interrupt priority for timer 3 interrupt \\[%s\\]
(timer overflow interrupt). Max priority is 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3_prio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3_prio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER3_PRIO_SPEC;
impl crate::RegisterSpec for TIMER3_PRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3_prio::R`](R) reader structure"]
impl crate::Readable for TIMER3_PRIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer3_prio::W`](W) writer structure"]
impl crate::Writable for TIMER3_PRIO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer3_prio[%s]
to value 0"]
impl crate::Resettable for TIMER3_PRIO_SPEC {
    const RESET_VALUE: u32 = 0;
}
