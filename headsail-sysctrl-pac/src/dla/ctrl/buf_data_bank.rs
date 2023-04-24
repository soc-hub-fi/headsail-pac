#[doc = "Register `buf_data_bank` reader"]
pub struct R(crate::R<BUF_DATA_BANK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_DATA_BANK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_DATA_BANK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_DATA_BANK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `buf_data_bank` writer"]
pub struct W(crate::W<BUF_DATA_BANK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_DATA_BANK_SPEC>;
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
impl From<crate::W<BUF_DATA_BANK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_DATA_BANK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `a` reader - "]
pub type A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `a` writer - "]
pub type A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_DATA_BANK_SPEC, u8, u8, 4, O>;
#[doc = "Field `b` reader - "]
pub type B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `b` writer - "]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_DATA_BANK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<0> {
        A_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<16> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_data_bank](index.html) module"]
pub struct BUF_DATA_BANK_SPEC;
impl crate::RegisterSpec for BUF_DATA_BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_data_bank::R](R) reader structure"]
impl crate::Readable for BUF_DATA_BANK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_data_bank::W](W) writer structure"]
impl crate::Writable for BUF_DATA_BANK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets buf_data_bank to value 0"]
impl crate::Resettable for BUF_DATA_BANK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
