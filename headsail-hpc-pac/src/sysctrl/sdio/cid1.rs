#[doc = "Register `CID1` reader"]
pub type R = crate::R<CID1_SPEC>;
#[doc = "Field `card_id_word1` reader - "]
pub type CARD_ID_WORD1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word1(&self) -> CARD_ID_WORD1_R {
        CARD_ID_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID1")
            .field("card_id_word1", &self.card_id_word1())
            .finish()
    }
}
#[doc = "Card Identification Word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID1_SPEC;
impl crate::RegisterSpec for CID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for CID1_SPEC {}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for CID1_SPEC {
    const RESET_VALUE: u32 = 0;
}
