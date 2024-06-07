#[doc = "Register `buf_data_wait_a` reader"]
pub type R = crate::R<BufDataWaitASpec>;
#[doc = "Field `data_a` reader - "]
pub type DataAR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_a(&self) -> DataAR {
        DataAR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_data_wait_a")
            .field("data_a", &self.data_a())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufDataWaitASpec;
impl crate::RegisterSpec for BufDataWaitASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_wait_a::R`](R) reader structure"]
impl crate::Readable for BufDataWaitASpec {}
#[doc = "`reset()` method sets buf_data_wait_a to value 0"]
impl crate::Resettable for BufDataWaitASpec {
    const RESET_VALUE: u32 = 0;
}
