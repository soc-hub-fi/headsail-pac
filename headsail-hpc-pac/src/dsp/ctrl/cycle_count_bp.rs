#[doc = "Register `cycle_count_bp` reader"]
pub type R = crate::R<CYCLE_COUNT_BP_SPEC>;
#[doc = "Register `cycle_count_bp` writer"]
pub type W = crate::W<CYCLE_COUNT_BP_SPEC>;
#[doc = "Field `bp` reader - "]
pub type BP_R = crate::FieldReader<u32>;
#[doc = "Field `bp` writer - "]
pub type BP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cycle_count_bp")
            .field("bp", &self.bp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<CYCLE_COUNT_BP_SPEC> {
        BP_W::new(self, 0)
    }
}
#[doc = "Cycle count breakpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_bp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle_count_bp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CYCLE_COUNT_BP_SPEC;
impl crate::RegisterSpec for CYCLE_COUNT_BP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle_count_bp::R`](R) reader structure"]
impl crate::Readable for CYCLE_COUNT_BP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cycle_count_bp::W`](W) writer structure"]
impl crate::Writable for CYCLE_COUNT_BP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cycle_count_bp to value 0"]
impl crate::Resettable for CYCLE_COUNT_BP_SPEC {
    const RESET_VALUE: u32 = 0;
}
