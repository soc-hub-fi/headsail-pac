#[doc = "Register `AXI_WRITE_CONFIG` reader"]
pub struct R(crate::R<AXI_WRITE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXI_WRITE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXI_WRITE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXI_WRITE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AXI_WRITE_CONFIG` writer"]
pub struct W(crate::W<AXI_WRITE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXI_WRITE_CONFIG_SPEC>;
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
impl From<crate::W<AXI_WRITE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXI_WRITE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Burst_Length_AWLEN` reader - "]
pub type BURST_LENGTH_AWLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Burst_Length_AWLEN` writer - "]
pub type BURST_LENGTH_AWLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `Size_of_Transfer_AWSIZE` reader - "]
pub type SIZE_OF_TRANSFER_AWSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Size_of_Transfer_AWSIZE` writer - "]
pub type SIZE_OF_TRANSFER_AWSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `Type_of_Burst_AWBURST` reader - "]
pub type TYPE_OF_BURST_AWBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Type_of_Burst_AWBURST` writer - "]
pub type TYPE_OF_BURST_AWBURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `Enable_Reg_Group_0_for_Write` reader - "]
pub type ENABLE_REG_GROUP_0_FOR_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `Enable_Reg_Group_0_for_Write` writer - "]
pub type ENABLE_REG_GROUP_0_FOR_WRITE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, bool, O>;
#[doc = "Field `Enable_Reg_Group_1_for_Write` reader - "]
pub type ENABLE_REG_GROUP_1_FOR_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `Enable_Reg_Group_1_for_Write` writer - "]
pub type ENABLE_REG_GROUP_1_FOR_WRITE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, bool, O>;
#[doc = "Field `Producer_Write` reader - "]
pub type PRODUCER_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `Producer_Write` writer - "]
pub type PRODUCER_WRITE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, bool, O>;
#[doc = "Field `Consumer_Write` reader - "]
pub type CONSUMER_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `Consumer_Write` writer - "]
pub type CONSUMER_WRITE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_WRITE_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn burst_length_awlen(&self) -> BURST_LENGTH_AWLEN_R {
        BURST_LENGTH_AWLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn size_of_transfer_awsize(&self) -> SIZE_OF_TRANSFER_AWSIZE_R {
        SIZE_OF_TRANSFER_AWSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn type_of_burst_awburst(&self) -> TYPE_OF_BURST_AWBURST_R {
        TYPE_OF_BURST_AWBURST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enable_reg_group_0_for_write(&self) -> ENABLE_REG_GROUP_0_FOR_WRITE_R {
        ENABLE_REG_GROUP_0_FOR_WRITE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn enable_reg_group_1_for_write(&self) -> ENABLE_REG_GROUP_1_FOR_WRITE_R {
        ENABLE_REG_GROUP_1_FOR_WRITE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn producer_write(&self) -> PRODUCER_WRITE_R {
        PRODUCER_WRITE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn consumer_write(&self) -> CONSUMER_WRITE_R {
        CONSUMER_WRITE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length_awlen(&mut self) -> BURST_LENGTH_AWLEN_W<0> {
        BURST_LENGTH_AWLEN_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn size_of_transfer_awsize(&mut self) -> SIZE_OF_TRANSFER_AWSIZE_W<8> {
        SIZE_OF_TRANSFER_AWSIZE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn type_of_burst_awburst(&mut self) -> TYPE_OF_BURST_AWBURST_W<11> {
        TYPE_OF_BURST_AWBURST_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_0_for_write(&mut self) -> ENABLE_REG_GROUP_0_FOR_WRITE_W<13> {
        ENABLE_REG_GROUP_0_FOR_WRITE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_1_for_write(&mut self) -> ENABLE_REG_GROUP_1_FOR_WRITE_W<14> {
        ENABLE_REG_GROUP_1_FOR_WRITE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn producer_write(&mut self) -> PRODUCER_WRITE_W<15> {
        PRODUCER_WRITE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn consumer_write(&mut self) -> CONSUMER_WRITE_W<16> {
        CONSUMER_WRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [axi_write_config](index.html) module"]
pub struct AXI_WRITE_CONFIG_SPEC;
impl crate::RegisterSpec for AXI_WRITE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [axi_write_config::R](R) reader structure"]
impl crate::Readable for AXI_WRITE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [axi_write_config::W](W) writer structure"]
impl crate::Writable for AXI_WRITE_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_WRITE_CONFIG to value 0"]
impl crate::Resettable for AXI_WRITE_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
