#[doc = "Register `FC_MASK1` reader"]
pub struct R(crate::R<FC_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC_MASK1` writer"]
pub struct W(crate::W<FC_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FC_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC_MASK1` reader - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FC_MASK1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FC_MASK1` writer - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub type FC_MASK1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FC_MASK1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    pub fn fc_mask1(&self) -> FC_MASK1_R {
        FC_MASK1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn fc_mask1(&mut self) -> FC_MASK1_W<0> {
        FC_MASK1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events 32-63 dispatch mask to FC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc_mask1](index.html) module"]
pub struct FC_MASK1_SPEC;
impl crate::RegisterSpec for FC_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc_mask1::R](R) reader structure"]
impl crate::Readable for FC_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc_mask1::W](W) writer structure"]
impl crate::Writable for FC_MASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for FC_MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
