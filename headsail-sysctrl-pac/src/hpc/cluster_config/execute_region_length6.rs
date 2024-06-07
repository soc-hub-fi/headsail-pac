#[doc = "Register `execute_region_length6` reader"]
pub type R = crate::R<ExecuteRegionLength6Spec>;
#[doc = "Register `execute_region_length6` writer"]
pub type W = crate::W<ExecuteRegionLength6Spec>;
#[doc = "Field `len_0` reader - "]
pub type Len0R = crate::FieldReader<u64>;
#[doc = "Field `len_0` writer - "]
pub type Len0W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len_0(&self) -> Len0R {
        Len0R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("execute_region_length6")
            .field("len_0", &self.len_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len_0(&mut self) -> Len0W<ExecuteRegionLength6Spec> {
        Len0W::new(self, 0)
    }
}
#[doc = "Length for execute region #6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExecuteRegionLength6Spec;
impl crate::RegisterSpec for ExecuteRegionLength6Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`execute_region_length6::R`](R) reader structure"]
impl crate::Readable for ExecuteRegionLength6Spec {}
#[doc = "`write(|w| ..)` method takes [`execute_region_length6::W`](W) writer structure"]
impl crate::Writable for ExecuteRegionLength6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets execute_region_length6 to value 0"]
impl crate::Resettable for ExecuteRegionLength6Spec {
    const RESET_VALUE: u64 = 0;
}
