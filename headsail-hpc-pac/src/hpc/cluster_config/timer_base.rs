#[doc = "Register `timer_base` reader"]
pub type R = crate::R<TIMER_BASE_SPEC>;
#[doc = "Register `timer_base` writer"]
pub type W = crate::W<TIMER_BASE_SPEC>;
#[doc = "Field `base` reader - "]
pub type BASE_R = crate::FieldReader<u64>;
#[doc = "Field `base` writer - "]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer_base")
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<TIMER_BASE_SPEC> {
        BASE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_BASE_SPEC;
impl crate::RegisterSpec for TIMER_BASE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`timer_base::R`](R) reader structure"]
impl crate::Readable for TIMER_BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_base::W`](W) writer structure"]
impl crate::Writable for TIMER_BASE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets timer_base to value 0x0005_0000"]
impl crate::Resettable for TIMER_BASE_SPEC {
    const RESET_VALUE: u64 = 0x0005_0000;
}
