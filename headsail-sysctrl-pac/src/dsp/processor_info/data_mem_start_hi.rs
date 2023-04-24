#[doc = "Register `data_mem_start_hi` reader"]
pub struct R(crate::R<DATA_MEM_START_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_MEM_START_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_MEM_START_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_MEM_START_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `start_hi` reader - "]
pub type START_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_hi(&self) -> START_HI_R {
        START_HI_R::new(self.bits)
    }
}
#[doc = "Data memory start, high 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_mem_start_hi](index.html) module"]
pub struct DATA_MEM_START_HI_SPEC;
impl crate::RegisterSpec for DATA_MEM_START_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_mem_start_hi::R](R) reader structure"]
impl crate::Readable for DATA_MEM_START_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_mem_start_hi to value 0"]
impl crate::Resettable for DATA_MEM_START_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
