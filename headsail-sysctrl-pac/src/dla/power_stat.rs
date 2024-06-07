#[doc = "Register `power_stat` reader"]
pub type R = crate::R<PowerStatSpec>;
#[doc = "Field `ack1` reader - "]
pub type Ack1R = crate::BitReader;
#[doc = "Field `ack2` reader - "]
pub type Ack2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ack1(&self) -> Ack1R {
        Ack1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ack2(&self) -> Ack2R {
        Ack2R::new(((self.bits >> 1) & 1) != 0)
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
pub struct PowerStatSpec;
impl crate::RegisterSpec for PowerStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_stat::R`](R) reader structure"]
impl crate::Readable for PowerStatSpec {}
#[doc = "`reset()` method sets power_stat to value 0"]
impl crate::Resettable for PowerStatSpec {
    const RESET_VALUE: u32 = 0;
}
