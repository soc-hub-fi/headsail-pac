#[doc = "Register `data_mem_size_lo` reader"]
pub type R = crate::R<DATA_MEM_SIZE_LO_SPEC>;
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
        f.debug_struct("data_mem_size_lo")
            .field("size_lo", &self.size_lo())
            .finish()
    }
}
#[doc = "Data memory size in bytes, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_MEM_SIZE_LO_SPEC;
impl crate::RegisterSpec for DATA_MEM_SIZE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_mem_size_lo::R`](R) reader structure"]
impl crate::Readable for DATA_MEM_SIZE_LO_SPEC {}
#[doc = "`reset()` method sets data_mem_size_lo to value 0xff00"]
impl crate::Resettable for DATA_MEM_SIZE_LO_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
