#[doc = "Register `CID1` reader"]
pub type R = crate::R<Cid1Spec>;
#[doc = "Field `card_id_word1` reader - "]
pub type CardIdWord1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word1(&self) -> CardIdWord1R {
        CardIdWord1R::new(self.bits)
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
pub struct Cid1Spec;
impl crate::RegisterSpec for Cid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cid1::R`](R) reader structure"]
impl crate::Readable for Cid1Spec {}
#[doc = "`reset()` method sets CID1 to value 0"]
impl crate::Resettable for Cid1Spec {
    const RESET_VALUE: u32 = 0;
}
