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
#[doc = "Field `version` reader - Version number"]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `axi_addr_width` reader - AXI Address Width"]
pub type AXI_ADDR_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `axi_data_width` reader - AXI Data Width"]
pub type AXI_DATA_WIDTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Version number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AXI Address Width"]
    #[inline(always)]
    pub fn axi_addr_width(&self) -> AXI_ADDR_WIDTH_R {
        AXI_ADDR_WIDTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AXI Data Width"]
    #[inline(always)]
    pub fn axi_data_width(&self) -> AXI_DATA_WIDTH_R {
        AXI_DATA_WIDTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Version number - AXI Address width - AXI Data width-Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0x2000_2000"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_2000;
}
