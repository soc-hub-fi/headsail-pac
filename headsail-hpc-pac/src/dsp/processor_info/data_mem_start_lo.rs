#[doc = "Register `data_mem_start_lo` reader"]
pub type R = crate::R<DataMemStartLoSpec>;
#[doc = "Field `start_lo` reader - "]
pub type StartLoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_lo(&self) -> StartLoR {
        StartLoR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("data_mem_start_lo")
            .field("start_lo", &self.start_lo())
            .finish()
    }
}
#[doc = "Data memory start, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_mem_start_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMemStartLoSpec;
impl crate::RegisterSpec for DataMemStartLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_mem_start_lo::R`](R) reader structure"]
impl crate::Readable for DataMemStartLoSpec {}
#[doc = "`reset()` method sets data_mem_start_lo to value 0xff83_0000"]
impl crate::Resettable for DataMemStartLoSpec {
    const RESET_VALUE: u32 = 0xff83_0000;
}
