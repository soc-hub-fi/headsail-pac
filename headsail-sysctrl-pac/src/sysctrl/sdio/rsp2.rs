#[doc = "Register `RSP2` reader"]
pub type R = crate::R<RSP2_SPEC>;
#[doc = "Field `RESPONSE_WORD2` reader - "]
pub type RESPONSE_WORD2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word2(&self) -> RESPONSE_WORD2_R {
        RESPONSE_WORD2_R::new(self.bits)
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
pub struct RSP2_SPEC;
impl crate::RegisterSpec for RSP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp2::R`](R) reader structure"]
impl crate::Readable for RSP2_SPEC {}
#[doc = "`reset()` method sets RSP2 to value 0"]
impl crate::Resettable for RSP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
