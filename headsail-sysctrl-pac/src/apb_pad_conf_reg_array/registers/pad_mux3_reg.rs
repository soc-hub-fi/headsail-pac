#[doc = "Register `pad_mux3_reg` reader"]
pub struct R(crate::R<PAD_MUX3_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX3_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX3_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX3_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pad_mux3_reg` writer"]
pub struct W(crate::W<PAD_MUX3_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX3_REG_SPEC>;
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
impl From<crate::W<PAD_MUX3_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX3_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mux_select` reader - "]
pub type MUX_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mux_select` writer - "]
pub type MUX_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_MUX3_REG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mux_select(&self) -> MUX_SELECT_R {
        MUX_SELECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn mux_select(&mut self) -> MUX_SELECT_W<0> {
        MUX_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux3_reg](index.html) module"]
pub struct PAD_MUX3_REG_SPEC;
impl crate::RegisterSpec for PAD_MUX3_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux3_reg::R](R) reader structure"]
impl crate::Readable for PAD_MUX3_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux3_reg::W](W) writer structure"]
impl crate::Writable for PAD_MUX3_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pad_mux3_reg to value 0"]
impl crate::Resettable for PAD_MUX3_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
