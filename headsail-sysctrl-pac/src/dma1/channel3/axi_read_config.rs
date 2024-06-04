#[doc = "Register `AXI_READ_CONFIG` reader"]
pub type R = crate::R<AXI_READ_CONFIG_SPEC>;
#[doc = "Register `AXI_READ_CONFIG` writer"]
pub type W = crate::W<AXI_READ_CONFIG_SPEC>;
#[doc = "Field `Burst_Length_ARLEN` reader - "]
pub type BURST_LENGTH_ARLEN_R = crate::FieldReader;
#[doc = "Field `Burst_Length_ARLEN` writer - "]
pub type BURST_LENGTH_ARLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Size_of_Transfer_ARSIZE` reader - "]
pub type SIZE_OF_TRANSFER_ARSIZE_R = crate::FieldReader;
#[doc = "Field `Size_of_Transfer_ARSIZE` writer - "]
pub type SIZE_OF_TRANSFER_ARSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Type_of_Burst_ARBURST` reader - "]
pub type TYPE_OF_BURST_ARBURST_R = crate::FieldReader;
#[doc = "Field `Type_of_Burst_ARBURST` writer - "]
pub type TYPE_OF_BURST_ARBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Enable_Reg_Group_0_for_Read` reader - "]
pub type ENABLE_REG_GROUP_0_FOR_READ_R = crate::BitReader;
#[doc = "Field `Enable_Reg_Group_0_for_Read` writer - "]
pub type ENABLE_REG_GROUP_0_FOR_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enable_Reg_Group_1_for_Read` reader - "]
pub type ENABLE_REG_GROUP_1_FOR_READ_R = crate::BitReader;
#[doc = "Field `Enable_Reg_Group_1_for_Read` writer - "]
pub type ENABLE_REG_GROUP_1_FOR_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Producer_Read` reader - "]
pub type PRODUCER_READ_R = crate::BitReader;
#[doc = "Field `Producer_Read` writer - "]
pub type PRODUCER_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Consumer_Read` reader - "]
pub type CONSUMER_READ_R = crate::BitReader;
#[doc = "Field `Consumer_Read` writer - "]
pub type CONSUMER_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_READ_CONFIG")
            .field("burst_length_arlen", &self.burst_length_arlen())
            .field("size_of_transfer_arsize", &self.size_of_transfer_arsize())
            .field("type_of_burst_arburst", &self.type_of_burst_arburst())
            .field(
                "enable_reg_group_0_for_read",
                &self.enable_reg_group_0_for_read(),
            )
            .field(
                "enable_reg_group_1_for_read",
                &self.enable_reg_group_1_for_read(),
            )
            .field("producer_read", &self.producer_read())
            .field("consumer_read", &self.consumer_read())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length_arlen(&mut self) -> BURST_LENGTH_ARLEN_W<AXI_READ_CONFIG_SPEC> {
        BURST_LENGTH_ARLEN_W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn size_of_transfer_arsize(&mut self) -> SIZE_OF_TRANSFER_ARSIZE_W<AXI_READ_CONFIG_SPEC> {
        SIZE_OF_TRANSFER_ARSIZE_W::new(self, 8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn type_of_burst_arburst(&mut self) -> TYPE_OF_BURST_ARBURST_W<AXI_READ_CONFIG_SPEC> {
        TYPE_OF_BURST_ARBURST_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_0_for_read(
        &mut self,
    ) -> ENABLE_REG_GROUP_0_FOR_READ_W<AXI_READ_CONFIG_SPEC> {
        ENABLE_REG_GROUP_0_FOR_READ_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_1_for_read(
        &mut self,
    ) -> ENABLE_REG_GROUP_1_FOR_READ_W<AXI_READ_CONFIG_SPEC> {
        ENABLE_REG_GROUP_1_FOR_READ_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn producer_read(&mut self) -> PRODUCER_READ_W<AXI_READ_CONFIG_SPEC> {
        PRODUCER_READ_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn consumer_read(&mut self) -> CONSUMER_READ_W<AXI_READ_CONFIG_SPEC> {
        CONSUMER_READ_W::new(self, 16)
    }
}
#[doc = "AXI configuration signals for read Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_read_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_read_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_READ_CONFIG_SPEC;
impl crate::RegisterSpec for AXI_READ_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_read_config::R`](R) reader structure"]
impl crate::Readable for AXI_READ_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_read_config::W`](W) writer structure"]
impl crate::Writable for AXI_READ_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXI_READ_CONFIG to value 0"]
impl crate::Resettable for AXI_READ_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
