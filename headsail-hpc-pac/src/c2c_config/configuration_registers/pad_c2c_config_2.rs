#[doc = "Register `PAD_C2C_CONFIG_2` reader"]
pub type R = crate::R<PAD_C2C_CONFIG_2_SPEC>;
#[doc = "Register `PAD_C2C_CONFIG_2` writer"]
pub type W = crate::W<PAD_C2C_CONFIG_2_SPEC>;
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_ACK_RX_CONFIG_R = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_ACK_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_ACK_RX_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_VALID_RX_CONFIG_R = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_VALID_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_VALID_RX_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `pad_PHY_DATA_RX_config` reader - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_RX_CONFIG_R = crate::FieldReader<u16>;
#[doc = "Field `pad_PHY_DATA_RX_config` writer - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub type PAD_PHY_DATA_RX_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
    pub fn pad_phy_data_ack_rx_config(
        &mut self,
    ) -> PAD_PHY_DATA_ACK_RX_CONFIG_W<PAD_C2C_CONFIG_2_SPEC> {
        PAD_PHY_DATA_ACK_RX_CONFIG_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_valid_rx_config(
        &mut self,
    ) -> PAD_PHY_DATA_VALID_RX_CONFIG_W<PAD_C2C_CONFIG_2_SPEC> {
        PAD_PHY_DATA_VALID_RX_CONFIG_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    #[inline(always)]
    #[must_use]
    pub fn pad_phy_data_rx_config(&mut self) -> PAD_PHY_DATA_RX_CONFIG_W<PAD_C2C_CONFIG_2_SPEC> {
        PAD_PHY_DATA_RX_CONFIG_W::new(self, 20)
    }
}
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_c2c_config_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_c2c_config_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_C2C_CONFIG_2_SPEC;
impl crate::RegisterSpec for PAD_C2C_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_c2c_config_2::R`](R) reader structure"]
impl crate::Readable for PAD_C2C_CONFIG_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_c2c_config_2::W`](W) writer structure"]
impl crate::Writable for PAD_C2C_CONFIG_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_C2C_CONFIG_2 to value 0"]
impl crate::Resettable for PAD_C2C_CONFIG_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
