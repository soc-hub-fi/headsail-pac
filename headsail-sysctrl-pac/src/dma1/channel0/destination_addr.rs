#[doc = "Register `DESTINATION_ADDR` reader"]
pub type R = crate::R<DestinationAddrSpec>;
#[doc = "Register `DESTINATION_ADDR` writer"]
pub type W = crate::W<DestinationAddrSpec>;
#[doc = "Field `Destination_Address` reader - "]
pub type DestinationAddressR = crate::FieldReader<u32>;
#[doc = "Field `Destination_Address` writer - "]
pub type DestinationAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn destination_address(&self) -> DestinationAddressR {
        DestinationAddressR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION_ADDR")
            .field("destination_address", &self.destination_address())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address(&mut self) -> DestinationAddressW<DestinationAddrSpec> {
        DestinationAddressW::new(self, 0)
    }
}
#[doc = "Start address for write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DestinationAddrSpec;
impl crate::RegisterSpec for DestinationAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination_addr::R`](R) reader structure"]
impl crate::Readable for DestinationAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`destination_addr::W`](W) writer structure"]
impl crate::Writable for DestinationAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESTINATION_ADDR to value 0"]
impl crate::Resettable for DestinationAddrSpec {
    const RESET_VALUE: u32 = 0;
}
