#[doc = "Register `cycle_count_hi` reader"]
pub type R = crate::R<CycleCountHiSpec>;
#[doc = "Field `hi` reader - Cycle count (high 32 bits)"]
pub type HiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle count (high 32 bits)"]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cycle_count_hi")
            .field("hi", &self.hi())
            .finish()
    }
}
#[doc = "High part of Cycle count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycleCountHiSpec;
impl crate::RegisterSpec for CycleCountHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle_count_hi::R`](R) reader structure"]
impl crate::Readable for CycleCountHiSpec {}
#[doc = "`reset()` method sets cycle_count_hi to value 0"]
impl crate::Resettable for CycleCountHiSpec {
    const RESET_VALUE: u32 = 0;
}
