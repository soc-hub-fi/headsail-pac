#[doc = "Register `DESTINATION_LENGTH` reader"]
pub type R = crate::R<DESTINATION_LENGTH_SPEC>;
#[doc = "Register `DESTINATION_LENGTH` writer"]
pub type W = crate::W<DESTINATION_LENGTH_SPEC>;
#[doc = "Field `Destination_Length` reader - "]
pub type DESTINATION_LENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `Destination_Length` writer - "]
pub type DESTINATION_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn destination_length(&self) -> DESTINATION_LENGTH_R {
        DESTINATION_LENGTH_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION_LENGTH")
            .field("destination_length", &self.destination_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_length(&mut self) -> DESTINATION_LENGTH_W<DESTINATION_LENGTH_SPEC> {
        DESTINATION_LENGTH_W::new(self, 0)
    }
}
#[doc = "Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTINATION_LENGTH_SPEC;
impl crate::RegisterSpec for DESTINATION_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination_length::R`](R) reader structure"]
impl crate::Readable for DESTINATION_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`destination_length::W`](W) writer structure"]
impl crate::Writable for DESTINATION_LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESTINATION_LENGTH to value 0"]
impl crate::Resettable for DESTINATION_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
