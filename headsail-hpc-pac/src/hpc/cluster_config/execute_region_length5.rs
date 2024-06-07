#[doc = "Register `execute_region_length5` reader"]
pub type R = crate::R<ExecuteRegionLength5Spec>;
#[doc = "Register `execute_region_length5` writer"]
pub type W = crate::W<ExecuteRegionLength5Spec>;
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
        f.debug_struct("execute_region_length5")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<ExecuteRegionLength5Spec> {
        LenW::new(self, 0)
    }
}
#[doc = "Length for execute region #5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExecuteRegionLength5Spec;
impl crate::RegisterSpec for ExecuteRegionLength5Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`execute_region_length5::R`](R) reader structure"]
impl crate::Readable for ExecuteRegionLength5Spec {}
#[doc = "`write(|w| ..)` method takes [`execute_region_length5::W`](W) writer structure"]
impl crate::Writable for ExecuteRegionLength5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets execute_region_length5 to value 0"]
impl crate::Resettable for ExecuteRegionLength5Spec {
    const RESET_VALUE: u64 = 0;
}
