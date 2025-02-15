#[doc = "Register `RSP2` reader"]
pub type R = crate::R<Rsp2Spec>;
#[doc = "Field `RESPONSE_WORD2` reader - "]
pub type ResponseWord2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word2(&self) -> ResponseWord2R {
        ResponseWord2R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP2")
            .field("response_word2", &self.response_word2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp2Spec;
impl crate::RegisterSpec for Rsp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp2::R`](R) reader structure"]
impl crate::Readable for Rsp2Spec {}
#[doc = "`reset()` method sets RSP2 to value 0"]
impl crate::Resettable for Rsp2Spec {
    const RESET_VALUE: u32 = 0;
}
