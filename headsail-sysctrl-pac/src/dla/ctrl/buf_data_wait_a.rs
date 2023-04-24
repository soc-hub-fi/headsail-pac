#[doc = "Register `buf_data_wait_a` reader"]
pub struct R(crate::R<BUF_DATA_WAIT_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_DATA_WAIT_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_DATA_WAIT_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_DATA_WAIT_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `data_a` reader - "]
pub type DATA_A_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_a(&self) -> DATA_A_R {
        DATA_A_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_data_wait_a](index.html) module"]
pub struct BUF_DATA_WAIT_A_SPEC;
impl crate::RegisterSpec for BUF_DATA_WAIT_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_data_wait_a::R](R) reader structure"]
impl crate::Readable for BUF_DATA_WAIT_A_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets buf_data_wait_a to value 0"]
impl crate::Resettable for BUF_DATA_WAIT_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
