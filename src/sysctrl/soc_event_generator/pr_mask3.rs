#[doc = "Register `PR_MASK3` reader"]
pub struct R(crate::R<PR_MASK3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_MASK3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_MASK3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_MASK3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_MASK3` writer"]
pub struct W(crate::W<PR_MASK3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_MASK3_SPEC>;
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
impl From<crate::W<PR_MASK3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_MASK3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR_MASK3` reader - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PR_MASK3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PR_MASK3` writer - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
pub type PR_MASK3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PR_MASK3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    pub fn pr_mask3(&self) -> PR_MASK3_R {
        PR_MASK3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to peripherals. Setting bit\\[i\\]
to 0b1 disables dispatching event to peripherals. Setting bit\\[i\\]
to 0b0 enables dispatching event to peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn pr_mask3(&mut self) -> PR_MASK3_W<0> {
        PR_MASK3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events 96-127 dispatch mask to peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_mask3](index.html) module"]
pub struct PR_MASK3_SPEC;
impl crate::RegisterSpec for PR_MASK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_mask3::R](R) reader structure"]
impl crate::Readable for PR_MASK3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_mask3::W](W) writer structure"]
impl crate::Writable for PR_MASK3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR_MASK3 to value 0xffff_ffff"]
impl crate::Resettable for PR_MASK3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
