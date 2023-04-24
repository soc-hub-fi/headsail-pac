#[doc = "Register `D_SOURCE_AXI_CONFIG` reader"]
pub struct R(crate::R<D_SOURCE_AXI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SOURCE_AXI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SOURCE_AXI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SOURCE_AXI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_SOURCE_AXI_CONFIG` writer"]
pub struct W(crate::W<D_SOURCE_AXI_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SOURCE_AXI_CONFIG_SPEC>;
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
impl From<crate::W<D_SOURCE_AXI_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SOURCE_AXI_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Burst_Length_ARLEN` reader - "]
pub type BURST_LENGTH_ARLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Burst_Length_ARLEN` writer - "]
pub type BURST_LENGTH_ARLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_SOURCE_AXI_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `Size_of_Transfer_ARSIZE` reader - "]
pub type SIZE_OF_TRANSFER_ARSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Size_of_Transfer_ARSIZE` writer - "]
pub type SIZE_OF_TRANSFER_ARSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_SOURCE_AXI_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `Type_of_Burst_ARBURST` reader - "]
pub type TYPE_OF_BURST_ARBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Type_of_Burst_ARBURST` writer - "]
pub type TYPE_OF_BURST_ARBURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_SOURCE_AXI_CONFIG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn burst_length_arlen(&self) -> BURST_LENGTH_ARLEN_R {
        BURST_LENGTH_ARLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn size_of_transfer_arsize(&self) -> SIZE_OF_TRANSFER_ARSIZE_R {
        SIZE_OF_TRANSFER_ARSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn type_of_burst_arburst(&self) -> TYPE_OF_BURST_ARBURST_R {
        TYPE_OF_BURST_ARBURST_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length_arlen(&mut self) -> BURST_LENGTH_ARLEN_W<0> {
        BURST_LENGTH_ARLEN_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn size_of_transfer_arsize(&mut self) -> SIZE_OF_TRANSFER_ARSIZE_W<8> {
        SIZE_OF_TRANSFER_ARSIZE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn type_of_burst_arburst(&mut self) -> TYPE_OF_BURST_ARBURST_W<11> {
        TYPE_OF_BURST_ARBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_source_axi_config](index.html) module"]
pub struct D_SOURCE_AXI_CONFIG_SPEC;
impl crate::RegisterSpec for D_SOURCE_AXI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_source_axi_config::R](R) reader structure"]
impl crate::Readable for D_SOURCE_AXI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_source_axi_config::W](W) writer structure"]
impl crate::Writable for D_SOURCE_AXI_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_SOURCE_AXI_CONFIG to value 0"]
impl crate::Resettable for D_SOURCE_AXI_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
