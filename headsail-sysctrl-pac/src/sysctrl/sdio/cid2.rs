#[doc = "Register `CID2` reader"]
pub type R = crate::R<Cid2Spec>;
#[doc = "Field `card_id_word2` reader - "]
pub type CardIdWord2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word2(&self) -> CardIdWord2R {
        CardIdWord2R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID2")
            .field("card_id_word2", &self.card_id_word2())
            .finish()
    }
}
#[doc = "Card Identification Word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cid2Spec;
impl crate::RegisterSpec for Cid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for Cid2Spec {}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for Cid2Spec {
    const RESET_VALUE: u32 = 0;
}
