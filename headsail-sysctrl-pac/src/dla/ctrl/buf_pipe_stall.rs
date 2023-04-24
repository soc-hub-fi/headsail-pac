#[doc = "Register `buf_pipe_stall` reader"]
pub struct R(crate::R<BUF_PIPE_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_PIPE_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_PIPE_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_PIPE_STALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cycles` reader - "]
pub type CYCLES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_pipe_stall](index.html) module"]
pub struct BUF_PIPE_STALL_SPEC;
impl crate::RegisterSpec for BUF_PIPE_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_pipe_stall::R](R) reader structure"]
impl crate::Readable for BUF_PIPE_STALL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets buf_pipe_stall to value 0"]
impl crate::Resettable for BUF_PIPE_STALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
