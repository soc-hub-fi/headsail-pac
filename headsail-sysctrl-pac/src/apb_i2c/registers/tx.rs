#[doc = "Register `TX` reader"]
pub struct R(crate::R<TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX` writer"]
pub struct W(crate::W<TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SPEC>;
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
impl From<crate::W<TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` reader - Transmit Register"]
pub type TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX` writer - Transmit Register"]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Register"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Register"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<0> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx](index.html) module"]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx::R](R) reader structure"]
impl crate::Readable for TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx::W](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX to value 0"]
impl crate::Resettable for TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
