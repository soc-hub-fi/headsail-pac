#[doc = "Register `cq_mem_start_lo` reader"]
pub struct R(crate::R<CQ_MEM_START_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQ_MEM_START_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQ_MEM_START_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQ_MEM_START_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `start_lo` reader - "]
pub type START_LO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn start_lo(&self) -> START_LO_R {
        START_LO_R::new(self.bits)
    }
}
#[doc = "CQ memory start, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cq_mem_start_lo](index.html) module"]
pub struct CQ_MEM_START_LO_SPEC;
impl crate::RegisterSpec for CQ_MEM_START_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cq_mem_start_lo::R](R) reader structure"]
impl crate::Readable for CQ_MEM_START_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cq_mem_start_lo to value 0xff83_ff00"]
impl crate::Resettable for CQ_MEM_START_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0xff83_ff00;
}
