#[doc = "Register `INTCFG` reader"]
pub struct R(crate::R<INTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCFG` writer"]
pub struct W(crate::W<INTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCFG_SPEC>;
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
impl From<crate::W<INTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THTX` reader - "]
pub type THTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THTX` writer - "]
pub type THTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `THRX` reader - "]
pub type THRX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRX` writer - "]
pub type THRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CNTTX` reader - "]
pub type CNTTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTTX` writer - "]
pub type CNTTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CNTRX` reader - "]
pub type CNTRX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTRX` writer - "]
pub type CNTRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CNTEN` reader - "]
pub type CNTEN_R = crate::BitReader<bool>;
#[doc = "Field `CNTEN` writer - "]
pub type CNTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCFG_SPEC, bool, O>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn thtx(&self) -> THTX_R {
        THTX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn thrx(&self) -> THRX_R {
        THRX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cnttx(&self) -> CNTTX_R {
        CNTTX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cntrx(&self) -> CNTRX_R {
        CNTRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn thtx(&mut self) -> THTX_W<0> {
        THTX_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn thrx(&mut self) -> THRX_W<8> {
        THRX_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn cnttx(&mut self) -> CNTTX_W<16> {
        CNTTX_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn cntrx(&mut self) -> CNTRX_W<24> {
        CNTRX_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<30> {
        CNTEN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcfg](index.html) module"]
pub struct INTCFG_SPEC;
impl crate::RegisterSpec for INTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intcfg::R](R) reader structure"]
impl crate::Readable for INTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intcfg::W](W) writer structure"]
impl crate::Writable for INTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCFG to value 0"]
impl crate::Resettable for INTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
