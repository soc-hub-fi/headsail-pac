#[doc = "Register `cq_mem_size_hi` reader"]
pub type R = crate::R<CqMemSizeHiSpec>;
#[doc = "Field `size_hi` reader - "]
pub type SizeHiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size_hi(&self) -> SizeHiR {
        SizeHiR::new(self.bits)
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
pub struct CqMemSizeHiSpec;
impl crate::RegisterSpec for CqMemSizeHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cq_mem_size_hi::R`](R) reader structure"]
impl crate::Readable for CqMemSizeHiSpec {}
#[doc = "`reset()` method sets cq_mem_size_hi to value 0"]
impl crate::Resettable for CqMemSizeHiSpec {
    const RESET_VALUE: u32 = 0;
}
