#[doc = "Register `nr_cached_regions` reader"]
pub type R = crate::R<NrCachedRegionsSpec>;
#[doc = "Register `nr_cached_regions` writer"]
pub type W = crate::W<NrCachedRegionsSpec>;
#[doc = "Field `nr` reader - "]
pub type NrR = crate::FieldReader<u64>;
#[doc = "Field `nr` writer - "]
pub type NrW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn nr(&self) -> NrR {
        NrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("nr_cached_regions")
            .field("nr", &self.nr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NrW<NrCachedRegionsSpec> {
        NrW::new(self, 0)
    }
}
#[doc = "Number of cached regions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nr_cached_regions::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nr_cached_regions::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrCachedRegionsSpec;
impl crate::RegisterSpec for NrCachedRegionsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`nr_cached_regions::R`](R) reader structure"]
impl crate::Readable for NrCachedRegionsSpec {}
#[doc = "`write(|w| ..)` method takes [`nr_cached_regions::W`](W) writer structure"]
impl crate::Writable for NrCachedRegionsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets nr_cached_regions to value 0"]
impl crate::Resettable for NrCachedRegionsSpec {
    const RESET_VALUE: u64 = 0;
}
