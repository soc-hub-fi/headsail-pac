#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Version` reader - "]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `No_of_Channels` reader - "]
pub type NO_OF_CHANNELS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_Data_Width` reader - "]
pub type AXI_DATA_WIDTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn no_of_channels(&self) -> NO_OF_CHANNELS_R {
        NO_OF_CHANNELS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn axi_data_width(&self) -> AXI_DATA_WIDTH_R {
        AXI_DATA_WIDTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "32'h00200401 default value Version number Number of available channels AXI data width\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
