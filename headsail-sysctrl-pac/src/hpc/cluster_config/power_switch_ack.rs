#[doc = "Register `power_switch_ack` reader"]
pub type R = crate::R<POWER_SWITCH_ACK_SPEC>;
#[doc = "Field `ack` reader - "]
pub type ACK_R = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("power_switch_ack")
            .field("ack", &self.ack())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_switch_ack::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_SWITCH_ACK_SPEC;
impl crate::RegisterSpec for POWER_SWITCH_ACK_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`power_switch_ack::R`](R) reader structure"]
impl crate::Readable for POWER_SWITCH_ACK_SPEC {}
#[doc = "`reset()` method sets power_switch_ack to value 0"]
impl crate::Resettable for POWER_SWITCH_ACK_SPEC {
    const RESET_VALUE: u64 = 0;
}
