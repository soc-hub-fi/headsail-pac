#[doc = "Register `cq_mem_start_lo` reader"]
pub type R = crate::R<CQ_MEM_START_LO_SPEC>;
#[doc = "Field `start_lo` reader - "]
pub type START_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_lo(&self) -> START_LO_R {
        START_LO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cq_mem_start_lo")
            .field("start_lo", &self.start_lo())
            .finish()
    }
}
#[doc = "CQ memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_start_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQ_MEM_START_LO_SPEC;
impl crate::RegisterSpec for CQ_MEM_START_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cq_mem_start_lo::R`](R) reader structure"]
impl crate::Readable for CQ_MEM_START_LO_SPEC {}
#[doc = "`reset()` method sets cq_mem_start_lo to value 0xff83_ff00"]
impl crate::Resettable for CQ_MEM_START_LO_SPEC {
    const RESET_VALUE: u32 = 0xff83_ff00;
}
