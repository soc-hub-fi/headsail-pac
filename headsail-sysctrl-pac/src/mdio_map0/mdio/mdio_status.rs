#[doc = "Register `MDIO_STATUS` reader"]
pub type R = crate::R<MDIO_STATUS_SPEC>;
#[doc = "Field `MDIO_STATUS` reader - "]
pub type MDIO_STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mdio_status(&self) -> MDIO_STATUS_R {
        MDIO_STATUS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_STATUS")
            .field("mdio_status", &self.mdio_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIO_STATUS_SPEC;
impl crate::RegisterSpec for MDIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_status::R`](R) reader structure"]
impl crate::Readable for MDIO_STATUS_SPEC {}
#[doc = "`reset()` method sets MDIO_STATUS to value 0"]
impl crate::Resettable for MDIO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
