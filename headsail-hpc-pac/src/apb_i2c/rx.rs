#[doc = "Register `RX` reader"]
pub type R = crate::R<RX_SPEC>;
#[doc = "Field `RX` reader - Receive Register"]
pub type RX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Register"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX").field("rx", &self.rx()).finish()
    }
}
#[doc = "Receive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_SPEC;
impl crate::RegisterSpec for RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx::R`](R) reader structure"]
impl crate::Readable for RX_SPEC {}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RX_SPEC {
    const RESET_VALUE: u32 = 0;
}
