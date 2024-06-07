#[doc = "Register `CMP_HI` reader"]
pub type R = crate::R<CmpHiSpec>;
#[doc = "Register `CMP_HI` writer"]
pub type W = crate::W<CmpHiSpec>;
#[doc = "Field `CMP_HI` reader - "]
pub type CmpHiR = crate::FieldReader<u32>;
#[doc = "Field `CMP_HI` writer - "]
pub type CmpHiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp_hi(&self) -> CmpHiR {
        CmpHiR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP_HI")
            .field("cmp_hi", &self.cmp_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_hi(&mut self) -> CmpHiW<CmpHiSpec> {
        CmpHiW::new(self, 0)
    }
}
#[doc = "Timer High comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpHiSpec;
impl crate::RegisterSpec for CmpHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_hi::R`](R) reader structure"]
impl crate::Readable for CmpHiSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_hi::W`](W) writer structure"]
impl crate::Writable for CmpHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_HI to value 0"]
impl crate::Resettable for CmpHiSpec {
    const RESET_VALUE: u32 = 0;
}
