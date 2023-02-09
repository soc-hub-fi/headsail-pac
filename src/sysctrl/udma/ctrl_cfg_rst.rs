#[doc = "Register `CTRL_CFG_RST` reader"]
pub struct R(crate::R<CTRL_CFG_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_CFG_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_CFG_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_CFG_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_CFG_RST` writer"]
pub struct W(crate::W<CTRL_CFG_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_CFG_RST_SPEC>;
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
impl From<crate::W<CTRL_CFG_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_CFG_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL_CFG_RST` reader - uDMA peripherals reset trigger (unimplemented)"]
pub type CTRL_CFG_RST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CTRL_CFG_RST` writer - uDMA peripherals reset trigger (unimplemented)"]
pub type CTRL_CFG_RST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_CFG_RST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub fn ctrl_cfg_rst(&self) -> CTRL_CFG_RST_R {
        CTRL_CFG_RST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_cfg_rst(&mut self) -> CTRL_CFG_RST_W<0> {
        CTRL_CFG_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA peripherals reset trigger (unimplemented)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_cfg_rst](index.html) module"]
pub struct CTRL_CFG_RST_SPEC;
impl crate::RegisterSpec for CTRL_CFG_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_cfg_rst::R](R) reader structure"]
impl crate::Readable for CTRL_CFG_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_cfg_rst::W](W) writer structure"]
impl crate::Writable for CTRL_CFG_RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_RST to value 0"]
impl crate::Resettable for CTRL_CFG_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
