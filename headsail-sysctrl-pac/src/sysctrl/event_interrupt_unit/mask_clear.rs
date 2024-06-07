#[doc = "Register `MASK_clear` reader"]
pub type R = crate::R<MaskClearSpec>;
#[doc = "Register `MASK_clear` writer"]
pub type W = crate::W<MaskClearSpec>;
#[doc = "Field `MASK_clear` reader - "]
pub type MaskClearR = crate::FieldReader<u32>;
#[doc = "Field `MASK_clear` writer - "]
pub type MaskClearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mask_clear(&self) -> MaskClearR {
        MaskClearR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_clear")
            .field("mask_clear", &self.mask_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mask_clear(&mut self) -> MaskClearW<MaskClearSpec> {
        MaskClearW::new(self, 0)
    }
}
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskClearSpec;
impl crate::RegisterSpec for MaskClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_clear::R`](R) reader structure"]
impl crate::Readable for MaskClearSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_clear::W`](W) writer structure"]
impl crate::Writable for MaskClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_clear to value 0"]
impl crate::Resettable for MaskClearSpec {
    const RESET_VALUE: u32 = 0;
}
