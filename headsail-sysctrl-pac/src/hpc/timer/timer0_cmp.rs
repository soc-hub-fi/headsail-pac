#[doc = "Register `timer0_cmp` reader"]
pub type R = crate::R<TIMER0_CMP_SPEC>;
#[doc = "Register `timer0_cmp` writer"]
pub type W = crate::W<TIMER0_CMP_SPEC>;
#[doc = "Field `cmp` reader - "]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `cmp` writer - "]
pub type CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("timer0_cmp")
            .field("cmp", &self.cmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<TIMER0_CMP_SPEC> {
        CMP_W::new(self, 0)
    }
}
#[doc = "Timer compare register for timer 0. Writing this will zero timer register for timer 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER0_CMP_SPEC;
impl crate::RegisterSpec for TIMER0_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_cmp::R`](R) reader structure"]
impl crate::Readable for TIMER0_CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer0_cmp::W`](W) writer structure"]
impl crate::Writable for TIMER0_CMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timer0_cmp to value 0"]
impl crate::Resettable for TIMER0_CMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
