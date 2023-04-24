#[doc = "Register `data_mem_size_lo` reader"]
pub struct R(crate::R<DATA_MEM_SIZE_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_MEM_SIZE_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_MEM_SIZE_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_MEM_SIZE_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `size_lo` reader - "]
pub type SIZE_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn size_lo(&self) -> SIZE_LO_R {
        SIZE_LO_R::new(self.bits)
    }
}
#[doc = "Data memory size in bytes, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_mem_size_lo](index.html) module"]
pub struct DATA_MEM_SIZE_LO_SPEC;
impl crate::RegisterSpec for DATA_MEM_SIZE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_mem_size_lo::R](R) reader structure"]
impl crate::Readable for DATA_MEM_SIZE_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_mem_size_lo to value 0xff00"]
impl crate::Resettable for DATA_MEM_SIZE_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
