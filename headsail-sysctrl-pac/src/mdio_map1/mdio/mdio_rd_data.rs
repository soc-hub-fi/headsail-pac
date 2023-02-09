#[doc = "Register `MDIO_RD_DATA` reader"]
pub struct R(crate::R<MDIO_RD_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_RD_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_RD_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_RD_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO_RD_DATA` reader - "]
pub type MDIO_RD_DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_rd_data(&self) -> MDIO_RD_DATA_R {
        MDIO_RD_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_rd_data](index.html) module"]
pub struct MDIO_RD_DATA_SPEC;
impl crate::RegisterSpec for MDIO_RD_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_rd_data::R](R) reader structure"]
impl crate::Readable for MDIO_RD_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIO_RD_DATA to value 0"]
impl crate::Resettable for MDIO_RD_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
