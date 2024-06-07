#[doc = "Register `CID3` reader"]
pub type R = crate::R<Cid3Spec>;
#[doc = "Field `card_id_word3` reader - "]
pub type CardIdWord3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word3(&self) -> CardIdWord3R {
        CardIdWord3R::new(self.bits)
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
pub struct Cid3Spec;
impl crate::RegisterSpec for Cid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid3::R`](R) reader structure"]
impl crate::Readable for Cid3Spec {}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for Cid3Spec {
    const RESET_VALUE: u32 = 0;
}
