#[doc = "Register `mtimecmp[%s]` reader"]
pub type R = crate::R<MTIMECMP_SPEC>;
#[doc = "Register `mtimecmp[%s]` writer"]
pub type W = crate::W<MTIMECMP_SPEC>;
#[doc = "Field `mtimecmp` reader - "]
pub type MTIMECMP_R = crate::FieldReader<u64>;
#[doc = "Field `mtimecmp` writer - "]
pub type MTIMECMP_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn mtimecmp(&self) -> MTIMECMP_R {
        MTIMECMP_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mtimecmp")
            .field("mtimecmp", &self.mtimecmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn mtimecmp(&mut self) -> MTIMECMP_W<MTIMECMP_SPEC> {
        MTIMECMP_W::new(self, 0)
    }
}
#[doc = "Array of machine mode timer compare registers for all Harts A timer interrupt for Hart #n is signalled whenever mtime is greater than or equal to the value in mtimecmp\\[n\\]. The timer interrupt is reflected in mtip bit of the mip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_SPEC;
impl crate::RegisterSpec for MTIMECMP_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtimecmp::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets mtimecmp[%s]
to value 0"]
impl crate::Resettable for MTIMECMP_SPEC {
    const RESET_VALUE: u64 = 0;
}
