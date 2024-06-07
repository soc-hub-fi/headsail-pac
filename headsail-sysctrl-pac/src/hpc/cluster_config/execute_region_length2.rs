#[doc = "Register `execute_region_length2` reader"]
pub type R = crate::R<ExecuteRegionLength2Spec>;
#[doc = "Register `execute_region_length2` writer"]
pub type W = crate::W<ExecuteRegionLength2Spec>;
#[doc = "Field `len` reader - "]
pub type LenR = crate::FieldReader<u64>;
#[doc = "Field `len` writer - "]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("execute_region_length2")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<ExecuteRegionLength2Spec> {
        LenW::new(self, 0)
    }
}
#[doc = "Length for execute region #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExecuteRegionLength2Spec;
impl crate::RegisterSpec for ExecuteRegionLength2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`execute_region_length2::R`](R) reader structure"]
impl crate::Readable for ExecuteRegionLength2Spec {}
#[doc = "`write(|w| ..)` method takes [`execute_region_length2::W`](W) writer structure"]
impl crate::Writable for ExecuteRegionLength2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets execute_region_length2 to value 0"]
impl crate::Resettable for ExecuteRegionLength2Spec {
    const RESET_VALUE: u64 = 0;
}
