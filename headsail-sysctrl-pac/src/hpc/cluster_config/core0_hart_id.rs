#[doc = "Register `core0_hart_id` reader"]
pub struct R(crate::R<CORE0_HART_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_HART_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_HART_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_HART_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `hart_id` reader - "]
pub type HART_ID_R = crate::FieldReader<u64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn hart_id(&self) -> HART_ID_R {
        HART_ID_R::new(self.bits)
    }
}
#[doc = "Core #0 Hart ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_hart_id](index.html) module"]
pub struct CORE0_HART_ID_SPEC;
impl crate::RegisterSpec for CORE0_HART_ID_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core0_hart_id::R](R) reader structure"]
impl crate::Readable for CORE0_HART_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets core0_hart_id to value 0"]
impl crate::Resettable for CORE0_HART_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
