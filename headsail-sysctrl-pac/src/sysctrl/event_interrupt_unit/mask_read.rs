#[doc = "Register `MASK_read` reader"]
pub type R = crate::R<MaskReadSpec>;
#[doc = "Register `MASK_read` writer"]
pub type W = crate::W<MaskReadSpec>;
#[doc = "Field `MASK` reader - "]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - "]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_read")
            .field("mask", &self.mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MaskReadSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_read::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_read::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskReadSpec;
impl crate::RegisterSpec for MaskReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_read::R`](R) reader structure"]
impl crate::Readable for MaskReadSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_read::W`](W) writer structure"]
impl crate::Writable for MaskReadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_read to value 0"]
impl crate::Resettable for MaskReadSpec {
    const RESET_VALUE: u32 = 0;
}
