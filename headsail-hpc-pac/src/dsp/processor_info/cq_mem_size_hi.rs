#[doc = "Register `cq_mem_size_hi` reader"]
pub type R = crate::R<CQ_MEM_SIZE_HI_SPEC>;
#[doc = "Field `size_hi` reader - "]
pub type SIZE_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size_hi(&self) -> SIZE_HI_R {
        SIZE_HI_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cq_mem_size_hi")
            .field("size_hi", &self.size_hi())
            .finish()
    }
}
#[doc = "CQ memory size, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cq_mem_size_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQ_MEM_SIZE_HI_SPEC;
impl crate::RegisterSpec for CQ_MEM_SIZE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cq_mem_size_hi::R`](R) reader structure"]
impl crate::Readable for CQ_MEM_SIZE_HI_SPEC {}
#[doc = "`reset()` method sets cq_mem_size_hi to value 0"]
impl crate::Resettable for CQ_MEM_SIZE_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
