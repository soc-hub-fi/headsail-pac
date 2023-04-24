#[doc = "Register `PAD_CFG_6` reader"]
pub struct R(crate::R<PAD_CFG_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_6` writer"]
pub struct W(crate::W<PAD_CFG_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_6_SPEC>;
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
impl From<crate::W<PAD_CFG_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFGICN_DIV` reader - "]
pub type CFGICN_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFGICN_DIV` writer - "]
pub type CFGICN_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_6_SPEC, u8, u8, 8, O>;
#[doc = "Field `LPICN_DIV` reader - "]
pub type LPICN_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPICN_DIV` writer - "]
pub type LPICN_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_6_SPEC, u8, u8, 8, O>;
#[doc = "Field `HPICN_DIV` reader - "]
pub type HPICN_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPICN_DIV` writer - "]
pub type HPICN_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_CFG_6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfgicn_div(&self) -> CFGICN_DIV_R {
        CFGICN_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lpicn_div(&self) -> LPICN_DIV_R {
        LPICN_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hpicn_div(&self) -> HPICN_DIV_R {
        HPICN_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cfgicn_div(&mut self) -> CFGICN_DIV_W<0> {
        CFGICN_DIV_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn lpicn_div(&mut self) -> LPICN_DIV_W<8> {
        LPICN_DIV_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn hpicn_div(&mut self) -> HPICN_DIV_W<16> {
        HPICN_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider ratio for the 3 Interconnect modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_6](index.html) module"]
pub struct PAD_CFG_6_SPEC;
impl crate::RegisterSpec for PAD_CFG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_6::R](R) reader structure"]
impl crate::Readable for PAD_CFG_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_6::W](W) writer structure"]
impl crate::Writable for PAD_CFG_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_6 to value 0"]
impl crate::Resettable for PAD_CFG_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
