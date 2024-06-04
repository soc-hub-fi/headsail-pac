#[doc = "Register `power_stat` reader"]
pub type R = crate::R<POWER_STAT_SPEC>;
#[doc = "Field `ack1` reader - "]
pub type ACK1_R = crate::BitReader;
#[doc = "Field `ack2` reader - "]
pub type ACK2_R = crate::BitReader;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("power_stat")
            .field("ack1", &self.ack1())
            .field("ack2", &self.ack2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_STAT_SPEC;
impl crate::RegisterSpec for POWER_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_stat::R`](R) reader structure"]
impl crate::Readable for POWER_STAT_SPEC {}
#[doc = "`reset()` method sets power_stat to value 0"]
impl crate::Resettable for POWER_STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
