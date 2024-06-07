#[doc = "Register `PAD_C2C_CONFIG_2` reader"]
pub type R = crate::R<PadC2cConfig2Spec>;
#[doc = "Register `PAD_C2C_CONFIG_2` writer"]
pub type W = crate::W<PadC2cConfig2Spec>;
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataAckRxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataAckRxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataValidRxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataValidRxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataRxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataRxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_ack_rx_config(&self) -> PadPhyDataAckRxConfigR {
        PadPhyDataAckRxConfigR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_valid_rx_config(&self) -> PadPhyDataValidRxConfigR {
        PadPhyDataValidRxConfigR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_rx_config(&self) -> PadPhyDataRxConfigR {
        PadPhyDataRxConfigR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_C2C_CONFIG_2")
            .field(
                "pad_phy_data_ack_rx_config",
                &self.pad_phy_data_ack_rx_config(),
            )
            .field(
                "pad_phy_data_valid_rx_config",
                &self.pad_phy_data_valid_rx_config(),
            )
            .field("pad_phy_data_rx_config", &self.pad_phy_data_rx_config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_ack_rx_config(&mut self) -> PadPhyDataAckRxConfigW<PadC2cConfig2Spec> {
        PadPhyDataAckRxConfigW::new(self, 0)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_valid_rx_config(&mut self) -> PadPhyDataValidRxConfigW<PadC2cConfig2Spec> {
        PadPhyDataValidRxConfigW::new(self, 10)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_rx_config(&mut self) -> PadPhyDataRxConfigW<PadC2cConfig2Spec> {
        PadPhyDataRxConfigW::new(self, 20)
    }
}
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadC2cConfig2Spec;
impl crate::RegisterSpec for PadC2cConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_c2c_config_2::R`](R) reader structure"]
impl crate::Readable for PadC2cConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_c2c_config_2::W`](W) writer structure"]
impl crate::Writable for PadC2cConfig2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_C2C_CONFIG_2 to value 0"]
impl crate::Resettable for PadC2cConfig2Spec {
    const RESET_VALUE: u32 = 0;
}
