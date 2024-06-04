#[doc = "Register `CID3` reader"]
pub type R = crate::R<CID3_SPEC>;
#[doc = "Field `card_id_word3` reader - "]
pub type CARD_ID_WORD3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word3(&self) -> CARD_ID_WORD3_R {
        CARD_ID_WORD3_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID3")
            .field("card_id_word3", &self.card_id_word3())
            .finish()
    }
}
#[doc = "Card Identification Word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CID3_SPEC;
impl crate::RegisterSpec for CID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid3::R`](R) reader structure"]
impl crate::Readable for CID3_SPEC {}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for CID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
