#[doc = "Register `d_wrseparation` reader"]
pub type R = crate::R<DWrseparationSpec>;
#[doc = "Register `d_wrseparation` writer"]
pub type W = crate::W<DWrseparationSpec>;
#[doc = "Field `d_wrseparation` reader - "]
pub type DWrseparationR = crate::FieldReader<u32>;
#[doc = "Field `d_wrseparation` writer - "]
pub type DWrseparationW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_wrseparation(&self) -> DWrseparationR {
        DWrseparationR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("d_wrseparation")
            .field("d_wrseparation", &self.d_wrseparation())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_wrseparation(&mut self) -> DWrseparationW<DWrseparationSpec> {
        DWrseparationW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrseparation::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrseparation::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWrseparationSpec;
impl crate::RegisterSpec for DWrseparationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_wrseparation::R`](R) reader structure"]
impl crate::Readable for DWrseparationSpec {}
#[doc = "`write(|w| ..)` method takes [`d_wrseparation::W`](W) writer structure"]
impl crate::Writable for DWrseparationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets d_wrseparation to value 0"]
impl crate::Resettable for DWrseparationSpec {
    const RESET_VALUE: u32 = 0;
}
