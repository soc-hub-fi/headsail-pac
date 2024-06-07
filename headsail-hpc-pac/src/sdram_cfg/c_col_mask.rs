#[doc = "Register `c_col_mask` reader"]
pub type R = crate::R<CColMaskSpec>;
#[doc = "Register `c_col_mask` writer"]
pub type W = crate::W<CColMaskSpec>;
#[doc = "Field `c_col_mask` reader - "]
pub type CColMaskR = crate::FieldReader<u32>;
#[doc = "Field `c_col_mask` writer - "]
pub type CColMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_col_mask(&self) -> CColMaskR {
        CColMaskR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_col_mask")
            .field("c_col_mask", &self.c_col_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_col_mask(&mut self) -> CColMaskW<CColMaskSpec> {
        CColMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CColMaskSpec;
impl crate::RegisterSpec for CColMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_col_mask::R`](R) reader structure"]
impl crate::Readable for CColMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`c_col_mask::W`](W) writer structure"]
impl crate::Writable for CColMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_col_mask to value 0x03ff"]
impl crate::Resettable for CColMaskSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
