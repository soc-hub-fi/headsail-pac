#[doc = "Register `ptr_size` reader"]
pub type R = crate::R<PtrSizeSpec>;
#[doc = "Field `ptr_size` reader - "]
pub type PtrSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ptr_size(&self) -> PtrSizeR {
        PtrSizeR::new(self.bits)
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
pub struct PtrSizeSpec;
impl crate::RegisterSpec for PtrSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr_size::R`](R) reader structure"]
impl crate::Readable for PtrSizeSpec {}
#[doc = "`reset()` method sets ptr_size to value 0x04"]
impl crate::Resettable for PtrSizeSpec {
    const RESET_VALUE: u32 = 0x04;
}
