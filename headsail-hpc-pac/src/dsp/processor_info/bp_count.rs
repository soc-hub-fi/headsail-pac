#[doc = "Register `bp_count` reader"]
pub type R = crate::R<BP_COUNT_SPEC>;
#[doc = "Field `count` reader - "]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("bp_count")
            .field("count", &self.count())
            .finish()
    }
}
#[doc = "Breakpoint count (0x2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BP_COUNT_SPEC;
impl crate::RegisterSpec for BP_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp_count::R`](R) reader structure"]
impl crate::Readable for BP_COUNT_SPEC {}
#[doc = "`reset()` method sets bp_count to value 0x02"]
impl crate::Resettable for BP_COUNT_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
