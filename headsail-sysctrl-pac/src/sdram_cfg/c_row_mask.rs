#[doc = "Register `c_row_mask` reader"]
pub type R = crate::R<CRowMaskSpec>;
#[doc = "Register `c_row_mask` writer"]
pub type W = crate::W<CRowMaskSpec>;
#[doc = "Field `c_row_mask` reader - "]
pub type CRowMaskR = crate::FieldReader<u32>;
#[doc = "Field `c_row_mask` writer - "]
pub type CRowMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_row_mask(&self) -> CRowMaskR {
        CRowMaskR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_row_mask")
            .field("c_row_mask", &self.c_row_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_row_mask(&mut self) -> CRowMaskW<CRowMaskSpec> {
        CRowMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRowMaskSpec;
impl crate::RegisterSpec for CRowMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_row_mask::R`](R) reader structure"]
impl crate::Readable for CRowMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`c_row_mask::W`](W) writer structure"]
impl crate::Writable for CRowMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_row_mask to value 0x3fff"]
impl crate::Resettable for CRowMaskSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
