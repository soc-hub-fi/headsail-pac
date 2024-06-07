#[doc = "Register `data_mem_size_lo` reader"]
pub type R = crate::R<DataMemSizeLoSpec>;
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
        f.debug_struct("data_mem_size_lo")
            .field("size_lo", &self.size_lo())
            .finish()
    }
}
#[doc = "Data memory size in bytes, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_size_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMemSizeLoSpec;
impl crate::RegisterSpec for DataMemSizeLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_mem_size_lo::R`](R) reader structure"]
impl crate::Readable for DataMemSizeLoSpec {}
#[doc = "`reset()` method sets data_mem_size_lo to value 0xff00"]
impl crate::Resettable for DataMemSizeLoSpec {
    const RESET_VALUE: u32 = 0xff00;
}
