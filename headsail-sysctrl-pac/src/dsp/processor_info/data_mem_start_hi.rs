#[doc = "Register `data_mem_start_hi` reader"]
pub type R = crate::R<DATA_MEM_START_HI_SPEC>;
#[doc = "Field `start_hi` reader - "]
pub type START_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_hi(&self) -> START_HI_R {
        START_HI_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("data_mem_start_hi")
            .field("start_hi", &self.start_hi())
            .finish()
    }
}
#[doc = "Data memory start, high 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_MEM_START_HI_SPEC;
impl crate::RegisterSpec for DATA_MEM_START_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_mem_start_hi::R`](R) reader structure"]
impl crate::Readable for DATA_MEM_START_HI_SPEC {}
#[doc = "`reset()` method sets data_mem_start_hi to value 0"]
impl crate::Resettable for DATA_MEM_START_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
