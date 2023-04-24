#[doc = "Register `buf_data_wait_b` reader"]
pub struct R(crate::R<BUF_DATA_WAIT_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_DATA_WAIT_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_DATA_WAIT_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_DATA_WAIT_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `data_b` reader - "]
pub type DATA_B_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_b(&self) -> DATA_B_R {
        DATA_B_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_data_wait_b](index.html) module"]
pub struct BUF_DATA_WAIT_B_SPEC;
impl crate::RegisterSpec for BUF_DATA_WAIT_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_data_wait_b::R](R) reader structure"]
impl crate::Readable for BUF_DATA_WAIT_B_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets buf_data_wait_b to value 0"]
impl crate::Resettable for BUF_DATA_WAIT_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
