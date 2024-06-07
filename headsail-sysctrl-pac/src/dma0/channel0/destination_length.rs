#[doc = "Register `DESTINATION_LENGTH` reader"]
pub type R = crate::R<DestinationLengthSpec>;
#[doc = "Register `DESTINATION_LENGTH` writer"]
pub type W = crate::W<DestinationLengthSpec>;
#[doc = "Field `Destination_Length` reader - "]
pub type DestinationLengthR = crate::FieldReader<u32>;
#[doc = "Field `Destination_Length` writer - "]
pub type DestinationLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn destination_length(&self) -> DestinationLengthR {
        DestinationLengthR::new(self.bits)
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
    pub fn destination_length(&mut self) -> DestinationLengthW<DestinationLengthSpec> {
        DestinationLengthW::new(self, 0)
    }
}
#[doc = "Length of write transfer in bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DestinationLengthSpec;
impl crate::RegisterSpec for DestinationLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination_length::R`](R) reader structure"]
impl crate::Readable for DestinationLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`destination_length::W`](W) writer structure"]
impl crate::Writable for DestinationLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESTINATION_LENGTH to value 0"]
impl crate::Resettable for DestinationLengthSpec {
    const RESET_VALUE: u32 = 0;
}
