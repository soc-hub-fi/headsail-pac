#[doc = "Register `cq_mem_size_lo` reader"]
pub type R = crate::R<CQ_MEM_SIZE_LO_SPEC>;
#[doc = "Field `size_lo` reader - "]
pub type SIZE_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size_lo(&self) -> SIZE_LO_R {
        SIZE_LO_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cq_mem_size_lo")
            .field("size_lo", &self.size_lo())
            .finish()
    }
}
#[doc = "CQ memory size, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQ_MEM_SIZE_LO_SPEC;
impl crate::RegisterSpec for CQ_MEM_SIZE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cq_mem_size_lo::R`](R) reader structure"]
impl crate::Readable for CQ_MEM_SIZE_LO_SPEC {}
#[doc = "`reset()` method sets cq_mem_size_lo to value 0x0100"]
impl crate::Resettable for CQ_MEM_SIZE_LO_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
