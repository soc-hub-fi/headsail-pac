#[doc = "Register `D_DESTINATION_MODE` reader"]
pub type R = crate::R<DDestinationModeSpec>;
#[doc = "Register `D_DESTINATION_MODE` writer"]
pub type W = crate::W<DDestinationModeSpec>;
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
        f.debug_struct("D_DESTINATION_MODE")
            .field("destination_mode", &self.destination_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode(&mut self) -> DestinationModeW<DDestinationModeSpec> {
        DestinationModeW::new(self, 0)
    }
}
#[doc = "ATM only used for initiating a transfer with write to least significant bit. LSB is unset once transfer is started, until then the address and length registers must not be modified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d_destination_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDestinationModeSpec;
impl crate::RegisterSpec for DDestinationModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_destination_mode::R`](R) reader structure"]
impl crate::Readable for DDestinationModeSpec {}
#[doc = "`write(|w| ..)` method takes [`d_destination_mode::W`](W) writer structure"]
impl crate::Writable for DDestinationModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D_DESTINATION_MODE to value 0"]
impl crate::Resettable for DDestinationModeSpec {
    const RESET_VALUE: u32 = 0;
}
