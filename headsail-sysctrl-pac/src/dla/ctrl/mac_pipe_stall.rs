#[doc = "Register `mac_pipe_stall` reader"]
pub struct R(crate::R<MAC_PIPE_STALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_PIPE_STALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_PIPE_STALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_PIPE_STALL_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_pipe_stall](index.html) module"]
pub struct MAC_PIPE_STALL_SPEC;
impl crate::RegisterSpec for MAC_PIPE_STALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_pipe_stall::R](R) reader structure"]
impl crate::Readable for MAC_PIPE_STALL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets mac_pipe_stall to value 0"]
impl crate::Resettable for MAC_PIPE_STALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
