#[doc = "Register `power_switch_ack` reader"]
pub struct R(crate::R<POWER_SWITCH_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SWITCH_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SWITCH_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SWITCH_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ack` reader - "]
pub type ACK_R = crate::FieldReader<u64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_switch_ack](index.html) module"]
pub struct POWER_SWITCH_ACK_SPEC;
impl crate::RegisterSpec for POWER_SWITCH_ACK_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [power_switch_ack::R](R) reader structure"]
impl crate::Readable for POWER_SWITCH_ACK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets power_switch_ack to value 0"]
impl crate::Resettable for POWER_SWITCH_ACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
