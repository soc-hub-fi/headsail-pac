#[doc = "Register `stall_count_lo` reader"]
pub type R = crate::R<STALL_COUNT_LO_SPEC>;
#[doc = "Field `lo` reader - Stall count (low 32 bits)"]
pub type LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stall count (low 32 bits)"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("stall_count_lo")
            .field("lo", &self.lo())
            .finish()
    }
}
#[doc = "Low part of Stall count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_count_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STALL_COUNT_LO_SPEC;
impl crate::RegisterSpec for STALL_COUNT_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall_count_lo::R`](R) reader structure"]
impl crate::Readable for STALL_COUNT_LO_SPEC {}
#[doc = "`reset()` method sets stall_count_lo to value 0"]
impl crate::Resettable for STALL_COUNT_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
