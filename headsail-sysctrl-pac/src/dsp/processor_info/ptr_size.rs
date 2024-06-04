#[doc = "Register `ptr_size` reader"]
pub type R = crate::R<PTR_SIZE_SPEC>;
#[doc = "Field `ptr_size` reader - "]
pub type PTR_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ptr_size(&self) -> PTR_SIZE_R {
        PTR_SIZE_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ptr_size")
            .field("ptr_size", &self.ptr_size())
            .finish()
    }
}
#[doc = "Pointer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTR_SIZE_SPEC;
impl crate::RegisterSpec for PTR_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr_size::R`](R) reader structure"]
impl crate::Readable for PTR_SIZE_SPEC {}
#[doc = "`reset()` method sets ptr_size to value 0x04"]
impl crate::Resettable for PTR_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
