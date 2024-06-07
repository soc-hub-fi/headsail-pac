#[doc = "Register `RSP0` reader"]
pub type R = crate::R<Rsp0Spec>;
#[doc = "Field `RESPONSE_WORD0` reader - "]
pub type ResponseWord0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn response_word0(&self) -> ResponseWord0R {
        ResponseWord0R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP0")
            .field("response_word0", &self.response_word0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp0Spec;
impl crate::RegisterSpec for Rsp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp0::R`](R) reader structure"]
impl crate::Readable for Rsp0Spec {}
#[doc = "`reset()` method sets RSP0 to value 0"]
impl crate::Resettable for Rsp0Spec {
    const RESET_VALUE: u32 = 0;
}
