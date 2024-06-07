#[doc = "Register `DESTINATION_MODE` reader"]
pub type R = crate::R<DestinationModeSpec>;
#[doc = "Register `DESTINATION_MODE` writer"]
pub type W = crate::W<DestinationModeSpec>;
#[doc = "Field `Destination_Mode` reader - "]
pub type DestinationModeR = crate::FieldReader<u32>;
#[doc = "Field `Destination_Mode` writer - "]
pub type DestinationModeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn destination_mode(&self) -> DestinationModeR {
        DestinationModeR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION_MODE")
            .field("destination_mode", &self.destination_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode(&mut self) -> DestinationModeW<DestinationModeSpec> {
        DestinationModeW::new(self, 0)
    }
}
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DestinationModeSpec;
impl crate::RegisterSpec for DestinationModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination_mode::R`](R) reader structure"]
impl crate::Readable for DestinationModeSpec {}
#[doc = "`write(|w| ..)` method takes [`destination_mode::W`](W) writer structure"]
impl crate::Writable for DestinationModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESTINATION_MODE to value 0"]
impl crate::Resettable for DestinationModeSpec {
    const RESET_VALUE: u32 = 0;
}
