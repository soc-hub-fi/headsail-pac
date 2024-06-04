#[doc = "Register `INTTYPE0` reader"]
pub type R = crate::R<INTTYPE0_SPEC>;
#[doc = "Register `INTTYPE0` writer"]
pub type W = crate::W<INTTYPE0_SPEC>;
#[doc = "Field `INTTYPE0` reader - "]
pub type INTTYPE0_R = crate::FieldReader<u32>;
#[doc = "Field `INTTYPE0` writer - "]
pub type INTTYPE0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn inttype0(&self) -> INTTYPE0_R {
        INTTYPE0_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTTYPE0")
            .field("inttype0", &self.inttype0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn inttype0(&mut self) -> INTTYPE0_W<INTTYPE0_SPEC> {
        INTTYPE0_W::new(self, 0)
    }
}
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTTYPE0_SPEC;
impl crate::RegisterSpec for INTTYPE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttype0::R`](R) reader structure"]
impl crate::Readable for INTTYPE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inttype0::W`](W) writer structure"]
impl crate::Writable for INTTYPE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPE0 to value 0"]
impl crate::Resettable for INTTYPE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
