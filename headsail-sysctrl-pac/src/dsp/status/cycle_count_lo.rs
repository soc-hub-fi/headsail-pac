#[doc = "Register `cycle_count_lo` reader"]
pub type R = crate::R<CycleCountLoSpec>;
#[doc = "Field `lo` reader - Cycle count (low 32 bits)"]
pub type LoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cycle count (low 32 bits)"]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cycle_count_lo")
            .field("lo", &self.lo())
            .finish()
    }
}
#[doc = "Low part of Cycle count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle_count_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CycleCountLoSpec;
impl crate::RegisterSpec for CycleCountLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cycle_count_lo::R`](R) reader structure"]
impl crate::Readable for CycleCountLoSpec {}
#[doc = "`reset()` method sets cycle_count_lo to value 0"]
impl crate::Resettable for CycleCountLoSpec {
    const RESET_VALUE: u32 = 0;
}
