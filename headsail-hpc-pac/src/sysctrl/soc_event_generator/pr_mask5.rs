#[doc = "Register `PR_MASK5` reader"]
pub type R = crate::R<PrMask5Spec>;
#[doc = "Register `PR_MASK5` writer"]
pub type W = crate::W<PrMask5Spec>;
#[doc = "Field `PR_MASK5` reader - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PrMask5R = crate::FieldReader<u32>;
#[doc = "Field `PR_MASK5` writer - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PrMask5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    pub fn pr_mask5(&self) -> PrMask5R {
        PrMask5R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR_MASK5")
            .field("pr_mask5", &self.pr_mask5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn pr_mask5(&mut self) -> PrMask5W<PrMask5Spec> {
        PrMask5W::new(self, 0)
    }
}
#[doc = "Events 160-191 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrMask5Spec;
impl crate::RegisterSpec for PrMask5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_mask5::R`](R) reader structure"]
impl crate::Readable for PrMask5Spec {}
#[doc = "`write(|w| ..)` method takes [`pr_mask5::W`](W) writer structure"]
impl crate::Writable for PrMask5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_MASK5 to value 0xffff_ffff"]
impl crate::Resettable for PrMask5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
