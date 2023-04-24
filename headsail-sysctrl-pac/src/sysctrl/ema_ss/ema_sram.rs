#[doc = "Register `EMA_SRAM` reader"]
pub struct R(crate::R<EMA_SRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMA_SRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMA_SRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMA_SRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMA_SRAM` writer"]
pub struct W(crate::W<EMA_SRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMA_SRAM_SPEC>;
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
impl From<crate::W<EMA_SRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMA_SRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPRAM_EMA` reader - "]
pub type DPRAM_EMA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPRAM_EMA` writer - "]
pub type DPRAM_EMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMA_SRAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPRAM_EMA` reader - "]
pub type SPRAM_EMA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPRAM_EMA` writer - "]
pub type SPRAM_EMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMA_SRAM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dpram_ema(&self) -> DPRAM_EMA_R {
        DPRAM_EMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spram_ema(&self) -> SPRAM_EMA_R {
        SPRAM_EMA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dpram_ema(&mut self) -> DPRAM_EMA_W<0> {
        DPRAM_EMA_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn spram_ema(&mut self) -> SPRAM_EMA_W<8> {
        SPRAM_EMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ema_sram](index.html) module"]
pub struct EMA_SRAM_SPEC;
impl crate::RegisterSpec for EMA_SRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ema_sram::R](R) reader structure"]
impl crate::Readable for EMA_SRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ema_sram::W](W) writer structure"]
impl crate::Writable for EMA_SRAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMA_SRAM to value 0x00ff_00ff"]
impl crate::Resettable for EMA_SRAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_00ff;
}
