#[doc = "Register `MASK_set` reader"]
pub type R = crate::R<MASK_SET_SPEC>;
#[doc = "Register `MASK_set` writer"]
pub type W = crate::W<MASK_SET_SPEC>;
#[doc = "Field `MASK_set` reader - "]
pub type MASK_SET_R = crate::FieldReader<u32>;
#[doc = "Field `MASK_set` writer - "]
pub type MASK_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mask_set(&self) -> MASK_SET_R {
        MASK_SET_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_set")
            .field("mask_set", &self.mask_set())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mask_set(&mut self) -> MASK_SET_W<MASK_SET_SPEC> {
        MASK_SET_W::new(self, 0)
    }
}
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK_SET_SPEC;
impl crate::RegisterSpec for MASK_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_set::R`](R) reader structure"]
impl crate::Readable for MASK_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask_set::W`](W) writer structure"]
impl crate::Writable for MASK_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_set to value 0"]
impl crate::Resettable for MASK_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
