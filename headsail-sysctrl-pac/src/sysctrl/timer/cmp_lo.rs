#[doc = "Register `CMP_LO` reader"]
pub type R = crate::R<CMP_LO_SPEC>;
#[doc = "Register `CMP_LO` writer"]
pub type W = crate::W<CMP_LO_SPEC>;
#[doc = "Field `CMP_LO` reader - "]
pub type CMP_LO_R = crate::FieldReader<u32>;
#[doc = "Field `CMP_LO` writer - "]
pub type CMP_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmp_lo(&self) -> CMP_LO_R {
        CMP_LO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP_LO")
            .field("cmp_lo", &self.cmp_lo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_lo(&mut self) -> CMP_LO_W<CMP_LO_SPEC> {
        CMP_LO_W::new(self, 0)
    }
}
#[doc = "Timer Low comparator value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_LO_SPEC;
impl crate::RegisterSpec for CMP_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_lo::R`](R) reader structure"]
impl crate::Readable for CMP_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp_lo::W`](W) writer structure"]
impl crate::Writable for CMP_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_LO to value 0"]
impl crate::Resettable for CMP_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
