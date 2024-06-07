#[doc = "Register `buf_data_wait_b` reader"]
pub type R = crate::R<BufDataWaitBSpec>;
#[doc = "Field `data_b` reader - "]
pub type DataBR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_b(&self) -> DataBR {
        DataBR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_data_wait_b")
            .field("data_b", &self.data_b())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufDataWaitBSpec;
impl crate::RegisterSpec for BufDataWaitBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_wait_b::R`](R) reader structure"]
impl crate::Readable for BufDataWaitBSpec {}
#[doc = "`reset()` method sets buf_data_wait_b to value 0"]
impl crate::Resettable for BufDataWaitBSpec {
    const RESET_VALUE: u32 = 0;
}
