#[doc = "Register `D_DESTINATION_AXI_CONFIG` reader"]
pub type R = crate::R<D_DESTINATION_AXI_CONFIG_SPEC>;
#[doc = "Register `D_DESTINATION_AXI_CONFIG` writer"]
pub type W = crate::W<D_DESTINATION_AXI_CONFIG_SPEC>;
#[doc = "Field `Burst_Length_AWLEN` reader - "]
pub type BURST_LENGTH_AWLEN_R = crate::FieldReader;
#[doc = "Field `Burst_Length_AWLEN` writer - "]
pub type BURST_LENGTH_AWLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Size_of_Transfer_AWSIZE` reader - "]
pub type SIZE_OF_TRANSFER_AWSIZE_R = crate::FieldReader;
#[doc = "Field `Size_of_Transfer_AWSIZE` writer - "]
pub type SIZE_OF_TRANSFER_AWSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Type_of_Burst_AWBURST` reader - "]
pub type TYPE_OF_BURST_AWBURST_R = crate::FieldReader;
#[doc = "Field `Type_of_Burst_AWBURST` writer - "]
pub type TYPE_OF_BURST_AWBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D_DESTINATION_AXI_CONFIG")
            .field("burst_length_awlen", &self.burst_length_awlen())
            .field("size_of_transfer_awsize", &self.size_of_transfer_awsize())
            .field("type_of_burst_awburst", &self.type_of_burst_awburst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn burst_length_awlen(&mut self) -> BURST_LENGTH_AWLEN_W<D_DESTINATION_AXI_CONFIG_SPEC> {
        BURST_LENGTH_AWLEN_W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn size_of_transfer_awsize(
        &mut self,
    ) -> SIZE_OF_TRANSFER_AWSIZE_W<D_DESTINATION_AXI_CONFIG_SPEC> {
        SIZE_OF_TRANSFER_AWSIZE_W::new(self, 8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn type_of_burst_awburst(
        &mut self,
    ) -> TYPE_OF_BURST_AWBURST_W<D_DESTINATION_AXI_CONFIG_SPEC> {
        TYPE_OF_BURST_AWBURST_W::new(self, 11)
    }
}
#[doc = "AXI configuration signals for write Enable should be done by CPU '0' = Not Enabled specific reg group '1' = Enabling specific reg group. Producer should be updated by CPU '0' = Configuring Reg Group 0 '1' = Configuring Reg Group 1 Consumer should be read by CPU '0' = Hardware source output by Reg Group 0 '1' = Hardware source output by Reg Group 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_axi_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_axi_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D_DESTINATION_AXI_CONFIG_SPEC;
impl crate::RegisterSpec for D_DESTINATION_AXI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_destination_axi_config::R`](R) reader structure"]
impl crate::Readable for D_DESTINATION_AXI_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d_destination_axi_config::W`](W) writer structure"]
impl crate::Writable for D_DESTINATION_AXI_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D_DESTINATION_AXI_CONFIG to value 0"]
impl crate::Resettable for D_DESTINATION_AXI_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
