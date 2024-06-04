#[doc = "Register `CID2` reader"]
pub type R = crate::R<CID2_SPEC>;
#[doc = "Field `card_id_word2` reader - "]
pub type CARD_ID_WORD2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word2(&self) -> CARD_ID_WORD2_R {
        CARD_ID_WORD2_R::new(self.bits)
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
pub struct CID2_SPEC;
impl crate::RegisterSpec for CID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid2::R`](R) reader structure"]
impl crate::Readable for CID2_SPEC {}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for CID2_SPEC {
    const RESET_VALUE: u32 = 0;
}
