#[doc = "Register `pp_axi_write` reader"]
pub struct R(crate::R<PP_AXI_WRITE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_AXI_WRITE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_AXI_WRITE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_AXI_WRITE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pp_axi_write` writer"]
pub struct W(crate::W<PP_AXI_WRITE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_AXI_WRITE_SPEC>;
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
impl From<crate::W<PP_AXI_WRITE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_AXI_WRITE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `addr` reader - "]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `addr` writer - "]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PP_AXI_WRITE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp_axi_write](index.html) module"]
pub struct PP_AXI_WRITE_SPEC;
impl crate::RegisterSpec for PP_AXI_WRITE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp_axi_write::R](R) reader structure"]
impl crate::Readable for PP_AXI_WRITE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp_axi_write::W](W) writer structure"]
impl crate::Writable for PP_AXI_WRITE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pp_axi_write to value 0"]
impl crate::Resettable for PP_AXI_WRITE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
