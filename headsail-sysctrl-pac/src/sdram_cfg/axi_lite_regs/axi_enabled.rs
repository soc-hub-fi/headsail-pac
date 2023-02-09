#[doc = "Register `axi_enabled` reader"]
pub struct R(crate::R<AXI_ENABLED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXI_ENABLED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXI_ENABLED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXI_ENABLED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `axi_enabled` writer"]
pub struct W(crate::W<AXI_ENABLED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXI_ENABLED_SPEC>;
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
impl From<crate::W<AXI_ENABLED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXI_ENABLED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `axi_enabled` reader - "]
pub type AXI_ENABLED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `axi_enabled` writer - "]
pub type AXI_ENABLED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_ENABLED_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn axi_enabled(&self) -> AXI_ENABLED_R {
        AXI_ENABLED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn axi_enabled(&mut self) -> AXI_ENABLED_W<0> {
        AXI_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [axi_enabled](index.html) module"]
pub struct AXI_ENABLED_SPEC;
impl crate::RegisterSpec for AXI_ENABLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [axi_enabled::R](R) reader structure"]
impl crate::Readable for AXI_ENABLED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [axi_enabled::W](W) writer structure"]
impl crate::Writable for AXI_ENABLED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets axi_enabled to value 0x01"]
impl crate::Resettable for AXI_ENABLED_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
