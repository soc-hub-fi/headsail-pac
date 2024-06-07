#[doc = "Register `d_wrlatcorr` reader"]
pub type R = crate::R<DWrlatcorrSpec>;
#[doc = "Register `d_wrlatcorr` writer"]
pub type W = crate::W<DWrlatcorrSpec>;
#[doc = "Field `d_wrlatcorr` reader - "]
pub type DWrlatcorrR = crate::FieldReader<u32>;
#[doc = "Field `d_wrlatcorr` writer - "]
pub type DWrlatcorrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_wrlatcorr(&self) -> DWrlatcorrR {
        DWrlatcorrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("d_wrlatcorr")
            .field("d_wrlatcorr", &self.d_wrlatcorr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_wrlatcorr(&mut self) -> DWrlatcorrW<DWrlatcorrSpec> {
        DWrlatcorrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_wrlatcorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_wrlatcorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DWrlatcorrSpec;
impl crate::RegisterSpec for DWrlatcorrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_wrlatcorr::R`](R) reader structure"]
impl crate::Readable for DWrlatcorrSpec {}
#[doc = "`write(|w| ..)` method takes [`d_wrlatcorr::W`](W) writer structure"]
impl crate::Writable for DWrlatcorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets d_wrlatcorr to value 0"]
impl crate::Resettable for DWrlatcorrSpec {
    const RESET_VALUE: u32 = 0;
}
