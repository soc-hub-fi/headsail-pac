#[doc = "Register `PR_MASK2` reader"]
pub type R = crate::R<PR_MASK2_SPEC>;
#[doc = "Register `PR_MASK2` writer"]
pub type W = crate::W<PR_MASK2_SPEC>;
#[doc = "Field `PR_MASK2` reader - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PR_MASK2_R = crate::FieldReader<u32>;
#[doc = "Field `PR_MASK2` writer - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PR_MASK2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    pub fn pr_mask2(&self) -> PR_MASK2_R {
        PR_MASK2_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR_MASK2")
            .field("pr_mask2", &self.pr_mask2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn pr_mask2(&mut self) -> PR_MASK2_W<PR_MASK2_SPEC> {
        PR_MASK2_W::new(self, 0)
    }
}
#[doc = "Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr_mask2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr_mask2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR_MASK2_SPEC;
impl crate::RegisterSpec for PR_MASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_mask2::R`](R) reader structure"]
impl crate::Readable for PR_MASK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pr_mask2::W`](W) writer structure"]
impl crate::Writable for PR_MASK2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_MASK2 to value 0xffff_ffff"]
impl crate::Resettable for PR_MASK2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
