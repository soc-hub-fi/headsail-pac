#[doc = "Register `stall_count_hi` reader"]
pub type R = crate::R<StallCountHiSpec>;
#[doc = "Field `hi` reader - "]
pub type HiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("stall_count_hi")
            .field("hi", &self.hi())
            .finish()
    }
}
#[doc = "High part of Stall count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_count_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StallCountHiSpec;
impl crate::RegisterSpec for StallCountHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall_count_hi::R`](R) reader structure"]
impl crate::Readable for StallCountHiSpec {}
#[doc = "`reset()` method sets stall_count_hi to value 0"]
impl crate::Resettable for StallCountHiSpec {
    const RESET_VALUE: u32 = 0;
}
