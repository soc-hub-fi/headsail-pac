#[doc = "Register `c_bank_mask` reader"]
pub struct R(crate::R<C_BANK_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C_BANK_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C_BANK_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C_BANK_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c_bank_mask` writer"]
pub struct W(crate::W<C_BANK_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C_BANK_MASK_SPEC>;
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
impl From<crate::W<C_BANK_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C_BANK_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c_bank_mask` reader - "]
pub type C_BANK_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `c_bank_mask` writer - "]
pub type C_BANK_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, C_BANK_MASK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_bank_mask(&self) -> C_BANK_MASK_R {
        C_BANK_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_bank_mask(&mut self) -> C_BANK_MASK_W<0> {
        C_BANK_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_bank_mask](index.html) module"]
pub struct C_BANK_MASK_SPEC;
impl crate::RegisterSpec for C_BANK_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c_bank_mask::R](R) reader structure"]
impl crate::Readable for C_BANK_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c_bank_mask::W](W) writer structure"]
impl crate::Writable for C_BANK_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets c_bank_mask to value 0x03"]
impl crate::Resettable for C_BANK_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
