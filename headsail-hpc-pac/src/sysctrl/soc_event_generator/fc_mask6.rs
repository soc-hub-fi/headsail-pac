#[doc = "Register `FC_MASK6` reader"]
pub type R = crate::R<FC_MASK6_SPEC>;
#[doc = "Register `FC_MASK6` writer"]
pub type W = crate::W<FC_MASK6_SPEC>;
#[doc = "Field `FC_MASK6` reader - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FC_MASK6_R = crate::FieldReader<u32>;
#[doc = "Field `FC_MASK6` writer - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FC_MASK6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    pub fn fc_mask6(&self) -> FC_MASK6_R {
        FC_MASK6_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_MASK6")
            .field("fc_mask6", &self.fc_mask6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn fc_mask6(&mut self) -> FC_MASK6_W<FC_MASK6_SPEC> {
        FC_MASK6_W::new(self, 0)
    }
}
#[doc = "Events 191-223 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC_MASK6_SPEC;
impl crate::RegisterSpec for FC_MASK6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_mask6::R`](R) reader structure"]
impl crate::Readable for FC_MASK6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc_mask6::W`](W) writer structure"]
impl crate::Writable for FC_MASK6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC_MASK6 to value 0xffff_ffff"]
impl crate::Resettable for FC_MASK6_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
