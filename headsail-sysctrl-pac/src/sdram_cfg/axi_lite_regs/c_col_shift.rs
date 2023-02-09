#[doc = "Register `c_col_shift` reader"]
pub struct R(crate::R<C_COL_SHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C_COL_SHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C_COL_SHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C_COL_SHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c_col_shift` writer"]
pub struct W(crate::W<C_COL_SHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C_COL_SHIFT_SPEC>;
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
impl From<crate::W<C_COL_SHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C_COL_SHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c_col_shift` reader - "]
pub type C_COL_SHIFT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `c_col_shift` writer - "]
pub type C_COL_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, C_COL_SHIFT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_col_shift(&self) -> C_COL_SHIFT_R {
        C_COL_SHIFT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_col_shift(&mut self) -> C_COL_SHIFT_W<0> {
        C_COL_SHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_col_shift](index.html) module"]
pub struct C_COL_SHIFT_SPEC;
impl crate::RegisterSpec for C_COL_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c_col_shift::R](R) reader structure"]
impl crate::Readable for C_COL_SHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c_col_shift::W](W) writer structure"]
impl crate::Writable for C_COL_SHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets c_col_shift to value 0x01"]
impl crate::Resettable for C_COL_SHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
