#[doc = "Register `c_row_shift` reader"]
pub type R = crate::R<CRowShiftSpec>;
#[doc = "Register `c_row_shift` writer"]
pub type W = crate::W<CRowShiftSpec>;
#[doc = "Field `c_row_shift` reader - "]
pub type CRowShiftR = crate::FieldReader<u32>;
#[doc = "Field `c_row_shift` writer - "]
pub type CRowShiftW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_row_shift(&self) -> CRowShiftR {
        CRowShiftR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_row_shift")
            .field("c_row_shift", &self.c_row_shift())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_row_shift(&mut self) -> CRowShiftW<CRowShiftSpec> {
        CRowShiftW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_row_shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_row_shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRowShiftSpec;
impl crate::RegisterSpec for CRowShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_row_shift::R`](R) reader structure"]
impl crate::Readable for CRowShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`c_row_shift::W`](W) writer structure"]
impl crate::Writable for CRowShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_row_shift to value 0x0d"]
impl crate::Resettable for CRowShiftSpec {
    const RESET_VALUE: u32 = 0x0d;
}
