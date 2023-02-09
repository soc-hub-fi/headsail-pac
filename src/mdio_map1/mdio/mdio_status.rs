#[doc = "Register `MDIO_STATUS` reader"]
pub struct R(crate::R<MDIO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO_STATUS` reader - "]
pub type MDIO_STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mdio_status(&self) -> MDIO_STATUS_R {
        MDIO_STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_status](index.html) module"]
pub struct MDIO_STATUS_SPEC;
impl crate::RegisterSpec for MDIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_status::R](R) reader structure"]
impl crate::Readable for MDIO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIO_STATUS to value 0"]
impl crate::Resettable for MDIO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
