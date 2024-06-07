#[doc = "Register `cq_mem_size_lo` reader"]
pub type R = crate::R<CqMemSizeLoSpec>;
#[doc = "Field `size_lo` reader - "]
pub type SizeLoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size_lo(&self) -> SizeLoR {
        SizeLoR::new(self.bits)
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
pub struct CqMemSizeLoSpec;
impl crate::RegisterSpec for CqMemSizeLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cq_mem_size_lo::R`](R) reader structure"]
impl crate::Readable for CqMemSizeLoSpec {}
#[doc = "`reset()` method sets cq_mem_size_lo to value 0x0100"]
impl crate::Resettable for CqMemSizeLoSpec {
    const RESET_VALUE: u32 = 0x0100;
}
