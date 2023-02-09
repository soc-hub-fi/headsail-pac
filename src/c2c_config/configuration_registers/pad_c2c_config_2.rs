#[doc = "Register `PAD_C2C_CONFIG_2` reader"]
pub struct R(crate::R<PAD_C2C_CONFIG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_C2C_CONFIG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_C2C_CONFIG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_C2C_CONFIG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_C2C_CONFIG_2` writer"]
pub struct W(crate::W<PAD_C2C_CONFIG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_C2C_CONFIG_2_SPEC>;
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
impl From<crate::W<PAD_C2C_CONFIG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_C2C_CONFIG_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_ACK_RX_CONFIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_ACK_RX_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_C2C_CONFIG_2_SPEC, u16, u16, 10, O>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_VALID_RX_CONFIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_VALID_RX_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_C2C_CONFIG_2_SPEC, u16, u16, 10, O>;
#[doc = "Field `pad_PHY_DATA_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_RX_CONFIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pad_PHY_DATA_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_RX_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_C2C_CONFIG_2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_ack_rx_config(&self) -> PAD_PHY_DATA_ACK_RX_CONFIG_R {
        PAD_PHY_DATA_ACK_RX_CONFIG_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_valid_rx_config(&self) -> PAD_PHY_DATA_VALID_RX_CONFIG_R {
        PAD_PHY_DATA_VALID_RX_CONFIG_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_rx_config(&self) -> PAD_PHY_DATA_RX_CONFIG_R {
        PAD_PHY_DATA_RX_CONFIG_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_ack_rx_config(&mut self) -> PAD_PHY_DATA_ACK_RX_CONFIG_W<0> {
        PAD_PHY_DATA_ACK_RX_CONFIG_W::new(self)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_valid_rx_config(&mut self) -> PAD_PHY_DATA_VALID_RX_CONFIG_W<10> {
        PAD_PHY_DATA_VALID_RX_CONFIG_W::new(self)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_rx_config(&mut self) -> PAD_PHY_DATA_RX_CONFIG_W<20> {
        PAD_PHY_DATA_RX_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_c2c_config_2](index.html) module"]
pub struct PAD_C2C_CONFIG_2_SPEC;
impl crate::RegisterSpec for PAD_C2C_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_c2c_config_2::R](R) reader structure"]
impl crate::Readable for PAD_C2C_CONFIG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_c2c_config_2::W](W) writer structure"]
impl crate::Writable for PAD_C2C_CONFIG_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_C2C_CONFIG_2 to value 0"]
impl crate::Resettable for PAD_C2C_CONFIG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
