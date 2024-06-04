#[doc = "Register `buf_data_wait_b` reader"]
pub type R = crate::R<BUF_DATA_WAIT_B_SPEC>;
#[doc = "Field `data_b` reader - "]
pub type DATA_B_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_b(&self) -> DATA_B_R {
        DATA_B_R::new(self.bits)
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
pub struct BUF_DATA_WAIT_B_SPEC;
impl crate::RegisterSpec for BUF_DATA_WAIT_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_wait_b::R`](R) reader structure"]
impl crate::Readable for BUF_DATA_WAIT_B_SPEC {}
#[doc = "`reset()` method sets buf_data_wait_b to value 0"]
impl crate::Resettable for BUF_DATA_WAIT_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
