#[doc = "Register `power_stat` reader"]
pub struct R(crate::R<POWER_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ack1` reader - "]
pub type ACK1_R = crate::BitReader<bool>;
#[doc = "Field `ack2` reader - "]
pub type ACK2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ack1(&self) -> ACK1_R {
        ACK1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ack2(&self) -> ACK2_R {
        ACK2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_stat](index.html) module"]
pub struct POWER_STAT_SPEC;
impl crate::RegisterSpec for POWER_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_stat::R](R) reader structure"]
impl crate::Readable for POWER_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets power_stat to value 0"]
impl crate::Resettable for POWER_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
