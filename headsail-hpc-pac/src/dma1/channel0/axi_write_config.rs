#[doc = "Register `AXI_WRITE_CONFIG` reader"]
pub type R = crate::R<AxiWriteConfigSpec>;
#[doc = "Register `AXI_WRITE_CONFIG` writer"]
pub type W = crate::W<AxiWriteConfigSpec>;
#[doc = "Field `Burst_Length_AWLEN` reader - "]
pub type BurstLengthAwlenR = crate::FieldReader;
#[doc = "Field `Burst_Length_AWLEN` writer - "]
pub type BurstLengthAwlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Size_of_Transfer_AWSIZE` reader - "]
pub type SizeOfTransferAwsizeR = crate::FieldReader;
#[doc = "Field `Size_of_Transfer_AWSIZE` writer - "]
pub type SizeOfTransferAwsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Type_of_Burst_AWBURST` reader - "]
pub type TypeOfBurstAwburstR = crate::FieldReader;
#[doc = "Field `Type_of_Burst_AWBURST` writer - "]
pub type TypeOfBurstAwburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Enable_Reg_Group_0_for_Write` reader - "]
pub type EnableRegGroup0ForWriteR = crate::BitReader;
#[doc = "Field `Enable_Reg_Group_0_for_Write` writer - "]
pub type EnableRegGroup0ForWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enable_Reg_Group_1_for_Write` reader - "]
pub type EnableRegGroup1ForWriteR = crate::BitReader;
#[doc = "Field `Enable_Reg_Group_1_for_Write` writer - "]
pub type EnableRegGroup1ForWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Producer_Write` reader - "]
pub type ProducerWriteR = crate::BitReader;
#[doc = "Field `Producer_Write` writer - "]
pub type ProducerWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Consumer_Write` reader - "]
pub type ConsumerWriteR = crate::BitReader;
#[doc = "Field `Consumer_Write` writer - "]
pub type ConsumerWriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn burst_length_awlen(&self) -> BurstLengthAwlenR {
        BurstLengthAwlenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn size_of_transfer_awsize(&self) -> SizeOfTransferAwsizeR {
        SizeOfTransferAwsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn type_of_burst_awburst(&self) -> TypeOfBurstAwburstR {
        TypeOfBurstAwburstR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enable_reg_group_0_for_write(&self) -> EnableRegGroup0ForWriteR {
        EnableRegGroup0ForWriteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn enable_reg_group_1_for_write(&self) -> EnableRegGroup1ForWriteR {
        EnableRegGroup1ForWriteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn producer_write(&self) -> ProducerWriteR {
        ProducerWriteR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn consumer_write(&self) -> ConsumerWriteR {
        ConsumerWriteR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_WRITE_CONFIG")
            .field("burst_length_awlen", &self.burst_length_awlen())
            .field("size_of_transfer_awsize", &self.size_of_transfer_awsize())
            .field("type_of_burst_awburst", &self.type_of_burst_awburst())
            .field(
                "enable_reg_group_0_for_write",
                &self.enable_reg_group_0_for_write(),
            )
            .field(
                "enable_reg_group_1_for_write",
                &self.enable_reg_group_1_for_write(),
            )
            .field("producer_write", &self.producer_write())
            .field("consumer_write", &self.consumer_write())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length_awlen(&mut self) -> BurstLengthAwlenW<AxiWriteConfigSpec> {
        BurstLengthAwlenW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn size_of_transfer_awsize(&mut self) -> SizeOfTransferAwsizeW<AxiWriteConfigSpec> {
        SizeOfTransferAwsizeW::new(self, 8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn type_of_burst_awburst(&mut self) -> TypeOfBurstAwburstW<AxiWriteConfigSpec> {
        TypeOfBurstAwburstW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_0_for_write(&mut self) -> EnableRegGroup0ForWriteW<AxiWriteConfigSpec> {
        EnableRegGroup0ForWriteW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn enable_reg_group_1_for_write(&mut self) -> EnableRegGroup1ForWriteW<AxiWriteConfigSpec> {
        EnableRegGroup1ForWriteW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn producer_write(&mut self) -> ProducerWriteW<AxiWriteConfigSpec> {
        ProducerWriteW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn consumer_write(&mut self) -> ConsumerWriteW<AxiWriteConfigSpec> {
        ConsumerWriteW::new(self, 16)
    }
}
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_write_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_write_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiWriteConfigSpec;
impl crate::RegisterSpec for AxiWriteConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_write_config::R`](R) reader structure"]
impl crate::Readable for AxiWriteConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_write_config::W`](W) writer structure"]
impl crate::Writable for AxiWriteConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXI_WRITE_CONFIG to value 0"]
impl crate::Resettable for AxiWriteConfigSpec {
    const RESET_VALUE: u32 = 0;
}
