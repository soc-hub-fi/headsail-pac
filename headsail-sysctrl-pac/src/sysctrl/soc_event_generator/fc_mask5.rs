#[doc = "Register `FC_MASK5` reader"]
pub type R = crate::R<FcMask5Spec>;
#[doc = "Register `FC_MASK5` writer"]
pub type W = crate::W<FcMask5Spec>;
#[doc = "Field `FC_MASK5` reader - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FcMask5R = crate::FieldReader<u32>;
#[doc = "Field `FC_MASK5` writer - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FcMask5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    pub fn fc_mask5(&self) -> FcMask5R {
        FcMask5R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC_MASK5")
            .field("fc_mask5", &self.fc_mask5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn fc_mask5(&mut self) -> FcMask5W<FcMask5Spec> {
        FcMask5W::new(self, 0)
    }
}
#[doc = "Events 160-191 dispatch mask to FC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_mask5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_mask5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcMask5Spec;
impl crate::RegisterSpec for FcMask5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc_mask5::R`](R) reader structure"]
impl crate::Readable for FcMask5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_mask5::W`](W) writer structure"]
impl crate::Writable for FcMask5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC_MASK5 to value 0xffff_ffff"]
impl crate::Resettable for FcMask5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
