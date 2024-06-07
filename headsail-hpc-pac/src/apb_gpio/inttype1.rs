#[doc = "Register `INTTYPE1` reader"]
pub type R = crate::R<Inttype1Spec>;
#[doc = "Register `INTTYPE1` writer"]
pub type W = crate::W<Inttype1Spec>;
#[doc = "Field `INTTYPE1` reader - "]
pub type Inttype1R = crate::FieldReader<u32>;
#[doc = "Field `INTTYPE1` writer - "]
pub type Inttype1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype1(&self) -> Inttype1R {
        Inttype1R::new(self.bits)
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
    pub fn inttype1(&mut self) -> Inttype1W<Inttype1Spec> {
        Inttype1W::new(self, 0)
    }
}
#[doc = "Interrupt Type 1. Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inttype1Spec;
impl crate::RegisterSpec for Inttype1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttype1::R`](R) reader structure"]
impl crate::Readable for Inttype1Spec {}
#[doc = "`write(|w| ..)` method takes [`inttype1::W`](W) writer structure"]
impl crate::Writable for Inttype1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPE1 to value 0"]
impl crate::Resettable for Inttype1Spec {
    const RESET_VALUE: u32 = 0;
}
