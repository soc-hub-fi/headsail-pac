#[doc = "Register `AXI_READ_CONFIG` reader"]
pub struct R(crate::R<AXI_READ_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXI_READ_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXI_READ_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXI_READ_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AXI_READ_CONFIG` writer"]
pub struct W(crate::W<AXI_READ_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXI_READ_CONFIG_SPEC>;
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
impl From<crate::W<AXI_READ_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXI_READ_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Burst_Length_ARLEN` reader - "]
pub type BURST_LENGTH_ARLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Burst_Length_ARLEN` writer - "]
pub type BURST_LENGTH_ARLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_READ_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `Size_of_Transfer_ARSIZE` reader - "]
pub type SIZE_OF_TRANSFER_ARSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Size_of_Transfer_ARSIZE` writer - "]
pub type SIZE_OF_TRANSFER_ARSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_READ_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `Type_of_Burst_ARBURST` reader - "]
pub type TYPE_OF_BURST_ARBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Type_of_Burst_ARBURST` writer - "]
pub type TYPE_OF_BURST_ARBURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_READ_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `Enable_Reg_Group_0_for_Read` reader - "]
pub type ENABLE_REG_GROUP_0_FOR_READ_R = crate::BitReader<bool>;
#[doc = "Field `Enable_Reg_Group_0_for_Read` writer - "]
pub type ENABLE_REG_GROUP_0_FOR_READ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_READ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Enable_Reg_Group_1_for_Read` reader - "]
pub type ENABLE_REG_GROUP_1_FOR_READ_R = crate::BitReader<bool>;
#[doc = "Field `Enable_Reg_Group_1_for_Read` writer - "]
pub type ENABLE_REG_GROUP_1_FOR_READ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_READ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Producer_Read` reader - "]
pub type PRODUCER_READ_R = crate::BitReader<bool>;
#[doc = "Field `Producer_Read` writer - "]
pub type PRODUCER_READ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_READ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Consumer_Read` reader - "]
pub type CONSUMER_READ_R = crate::BitReader<bool>;
#[doc = "Field `Consumer_Read` writer - "]
pub type CONSUMER_READ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_READ_CONFIG_SPEC, bool, O>;
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enable_reg_group_0_for_read(&self) -> ENABLE_REG_GROUP_0_FOR_READ_R {
        ENABLE_REG_GROUP_0_FOR_READ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn enable_reg_group_1_for_read(&self) -> ENABLE_REG_GROUP_1_FOR_READ_R {
        ENABLE_REG_GROUP_1_FOR_READ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn producer_read(&self) -> PRODUCER_READ_R {
        PRODUCER_READ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn consumer_read(&self) -> CONSUMER_READ_R {
        CONSUMER_READ_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_0_for_read(&mut self) -> ENABLE_REG_GROUP_0_FOR_READ_W<13> {
        ENABLE_REG_GROUP_0_FOR_READ_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_1_for_read(&mut self) -> ENABLE_REG_GROUP_1_FOR_READ_W<14> {
        ENABLE_REG_GROUP_1_FOR_READ_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn producer_read(&mut self) -> PRODUCER_READ_W<15> {
        PRODUCER_READ_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn consumer_read(&mut self) -> CONSUMER_READ_W<16> {
        CONSUMER_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [axi_read_config](index.html) module"]
pub struct AXI_READ_CONFIG_SPEC;
impl crate::RegisterSpec for AXI_READ_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [axi_read_config::R](R) reader structure"]
impl crate::Readable for AXI_READ_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [axi_read_config::W](W) writer structure"]
impl crate::Writable for AXI_READ_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_READ_CONFIG to value 0"]
impl crate::Resettable for AXI_READ_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
