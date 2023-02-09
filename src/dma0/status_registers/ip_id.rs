#[doc = "Register `IP_ID` reader"]
pub struct R(crate::R<IP_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IP_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IP_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IP_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - "]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "Constant 32'h70646d61 default value ; \"pdma\" in ascii\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip_id](index.html) module"]
pub struct IP_ID_SPEC;
impl crate::RegisterSpec for IP_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ip_id::R](R) reader structure"]
impl crate::Readable for IP_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IP_ID to value 0"]
impl crate::Resettable for IP_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
