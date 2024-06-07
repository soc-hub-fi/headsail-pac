#[doc = "Register `PAD_C2C_CONFIG_1` reader"]
pub type R = crate::R<PadC2cConfig1Spec>;
#[doc = "Register `PAD_C2C_CONFIG_1` writer"]
pub type W = crate::W<PadC2cConfig1Spec>;
#[doc = "Field `pad_PHY_DATA_ACK_TX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataAckTxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_ACK_TX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataAckTxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_VALID_TX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataValidTxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_VALID_TX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataValidTxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_TX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataTxConfigR = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_TX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PadPhyDataTxConfigW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_ack_tx_config(&self) -> PadPhyDataAckTxConfigR {
        PadPhyDataAckTxConfigR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_valid_tx_config(&self) -> PadPhyDataValidTxConfigR {
        PadPhyDataValidTxConfigR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    pub fn pad_phy_data_tx_config(&self) -> PadPhyDataTxConfigR {
        PadPhyDataTxConfigR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_C2C_CONFIG_1")
            .field(
                "pad_phy_data_ack_tx_config",
                &self.pad_phy_data_ack_tx_config(),
            )
            .field(
                "pad_phy_data_valid_tx_config",
                &self.pad_phy_data_valid_tx_config(),
            )
            .field("pad_phy_data_tx_config", &self.pad_phy_data_tx_config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_ack_tx_config(&mut self) -> PadPhyDataAckTxConfigW<PadC2cConfig1Spec> {
        PadPhyDataAckTxConfigW::new(self, 0)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_valid_tx_config(&mut self) -> PadPhyDataValidTxConfigW<PadC2cConfig1Spec> {
        PadPhyDataValidTxConfigW::new(self, 10)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_tx_config(&mut self) -> PadPhyDataTxConfigW<PadC2cConfig1Spec> {
        PadPhyDataTxConfigW::new(self, 20)
    }
}
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadC2cConfig1Spec;
impl crate::RegisterSpec for PadC2cConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_c2c_config_1::R`](R) reader structure"]
impl crate::Readable for PadC2cConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_c2c_config_1::W`](W) writer structure"]
impl crate::Writable for PadC2cConfig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_C2C_CONFIG_1 to value 0"]
impl crate::Resettable for PadC2cConfig1Spec {
    const RESET_VALUE: u32 = 0;
}
