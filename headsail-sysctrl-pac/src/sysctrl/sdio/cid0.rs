#[doc = "Register `CID0` reader"]
pub type R = crate::R<Cid0Spec>;
#[doc = "Field `card_id_word0` reader - "]
pub type CardIdWord0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word0(&self) -> CardIdWord0R {
        CardIdWord0R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID0")
            .field("card_id_word0", &self.card_id_word0())
            .finish()
    }
}
#[doc = "Card Identification Word0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid0Spec;
impl crate::RegisterSpec for Cid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid0::R`](R) reader structure"]
impl crate::Readable for Cid0Spec {}
#[doc = "`reset()` method sets CID0 to value 0"]
impl crate::Resettable for Cid0Spec {
    const RESET_VALUE: u32 = 0;
}
