#[doc = "Register `INTTYPE1` reader"]
pub type R = crate::R<INTTYPE1_SPEC>;
#[doc = "Register `INTTYPE1` writer"]
pub type W = crate::W<INTTYPE1_SPEC>;
#[doc = "Field `INTTYPE1` reader - "]
pub type INTTYPE1_R = crate::FieldReader<u32>;
#[doc = "Field `INTTYPE1` writer - "]
pub type INTTYPE1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTTYPE1")
            .field("inttype1", &self.inttype1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn inttype1(&mut self) -> INTTYPE1_W<INTTYPE1_SPEC> {
        INTTYPE1_W::new(self, 0)
    }
}
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTTYPE1_SPEC;
impl crate::RegisterSpec for INTTYPE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttype1::R`](R) reader structure"]
impl crate::Readable for INTTYPE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inttype1::W`](W) writer structure"]
impl crate::Writable for INTTYPE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPE1 to value 0"]
impl crate::Resettable for INTTYPE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
