#[doc = "Register `RSP3` reader"]
pub type R = crate::R<Rsp3Spec>;
#[doc = "Field `RESPONSE_WORD3` reader - "]
pub type ResponseWord3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word3(&self) -> ResponseWord3R {
        ResponseWord3R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP3")
            .field("response_word3", &self.response_word3())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp3Spec;
impl crate::RegisterSpec for Rsp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp3::R`](R) reader structure"]
impl crate::Readable for Rsp3Spec {}
#[doc = "`reset()` method sets RSP3 to value 0"]
impl crate::Resettable for Rsp3Spec {
    const RESET_VALUE: u32 = 0;
}
