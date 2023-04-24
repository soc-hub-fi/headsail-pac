#[doc = "Register `ptr_size` reader"]
pub struct R(crate::R<PTR_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTR_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTR_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTR_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ptr_size` reader - "]
pub type PTR_SIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ptr_size(&self) -> PTR_SIZE_R {
        PTR_SIZE_R::new(self.bits)
    }
}
#[doc = "Pointer size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr_size](index.html) module"]
pub struct PTR_SIZE_SPEC;
impl crate::RegisterSpec for PTR_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptr_size::R](R) reader structure"]
impl crate::Readable for PTR_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ptr_size to value 0x04"]
impl crate::Resettable for PTR_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
