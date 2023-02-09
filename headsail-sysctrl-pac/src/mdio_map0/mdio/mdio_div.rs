#[doc = "Register `MDIO_DIV` reader"]
pub struct R(crate::R<MDIO_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIO_DIV` writer"]
pub struct W(crate::W<MDIO_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIO_DIV_SPEC>;
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
impl From<crate::W<MDIO_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIO_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIO_DIV` reader - "]
pub type MDIO_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDIO_DIV` writer - "]
pub type MDIO_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDIO_DIV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mdio_div(&self) -> MDIO_DIV_R {
        MDIO_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_div(&mut self) -> MDIO_DIV_W<0> {
        MDIO_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_div](index.html) module"]
pub struct MDIO_DIV_SPEC;
impl crate::RegisterSpec for MDIO_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_div::R](R) reader structure"]
impl crate::Readable for MDIO_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdio_div::W](W) writer structure"]
impl crate::Writable for MDIO_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIO_DIV to value 0x0a"]
impl crate::Resettable for MDIO_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
