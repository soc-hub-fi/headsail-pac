#[doc = "Register `non_idempotent_region_length0` reader"]
pub type R = crate::R<NonIdempotentRegionLength0Spec>;
#[doc = "Register `non_idempotent_region_length0` writer"]
pub type W = crate::W<NonIdempotentRegionLength0Spec>;
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
        f.debug_struct("non_idempotent_region_length0")
            .field("len", &self.len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<NonIdempotentRegionLength0Spec> {
        LenW::new(self, 0)
    }
}
#[doc = "Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_idempotent_region_length0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_idempotent_region_length0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonIdempotentRegionLength0Spec;
impl crate::RegisterSpec for NonIdempotentRegionLength0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`non_idempotent_region_length0::R`](R) reader structure"]
impl crate::Readable for NonIdempotentRegionLength0Spec {}
#[doc = "`write(|w| ..)` method takes [`non_idempotent_region_length0::W`](W) writer structure"]
impl crate::Writable for NonIdempotentRegionLength0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets non_idempotent_region_length0 to value 0"]
impl crate::Resettable for NonIdempotentRegionLength0Spec {
    const RESET_VALUE: u64 = 0;
}
