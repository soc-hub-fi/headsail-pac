#[doc = "Register `DESTINATION_MODE_SET` writer"]
pub struct W(crate::W<DESTINATION_MODE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATION_MODE_SET_SPEC>;
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
impl From<crate::W<DESTINATION_MODE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATION_MODE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Destination_Mode_Set` writer - "]
pub type DESTINATION_MODE_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATION_MODE_SET_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode_set(&mut self) -> DESTINATION_MODE_SET_W<0> {
        DESTINATION_MODE_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to set destination mode register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destination_mode_set](index.html) module"]
pub struct DESTINATION_MODE_SET_SPEC;
impl crate::RegisterSpec for DESTINATION_MODE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [destination_mode_set::W](W) writer structure"]
impl crate::Writable for DESTINATION_MODE_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATION_MODE_SET to value 0"]
impl crate::Resettable for DESTINATION_MODE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
