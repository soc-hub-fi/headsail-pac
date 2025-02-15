#[doc = "Register `non_idempotent_region_addr_base0` reader"]
pub type R = crate::R<NonIdempotentRegionAddrBase0Spec>;
#[doc = "Register `non_idempotent_region_addr_base0` writer"]
pub type W = crate::W<NonIdempotentRegionAddrBase0Spec>;
#[doc = "Field `base` reader - Base address for side-effectful region #\\[%s\\]"]
pub type BaseR = crate::FieldReader<u64>;
#[doc = "Field `base` writer - Base address for side-effectful region #\\[%s\\]"]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - Base address for side-effectful region #\\[%s\\]"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("non_idempotent_region_addr_base0")
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63 - Base address for side-effectful region #\\[%s\\]"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<NonIdempotentRegionAddrBase0Spec> {
        BaseW::new(self, 0)
    }
}
#[doc = "Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_idempotent_region_addr_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_idempotent_region_addr_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonIdempotentRegionAddrBase0Spec;
impl crate::RegisterSpec for NonIdempotentRegionAddrBase0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`non_idempotent_region_addr_base0::R`](R) reader structure"]
impl crate::Readable for NonIdempotentRegionAddrBase0Spec {}
#[doc = "`write(|w| ..)` method takes [`non_idempotent_region_addr_base0::W`](W) writer structure"]
impl crate::Writable for NonIdempotentRegionAddrBase0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets non_idempotent_region_addr_base0 to value 0"]
impl crate::Resettable for NonIdempotentRegionAddrBase0Spec {
    const RESET_VALUE: u64 = 0;
}
