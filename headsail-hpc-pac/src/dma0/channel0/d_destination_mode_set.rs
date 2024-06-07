#[doc = "Register `D_DESTINATION_MODE_SET` writer"]
pub type W = crate::W<DDestinationModeSetSpec>;
#[doc = "Field `Destination_Mode_Set` writer - "]
pub type DestinationModeSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for crate::generic::Reg<DDestinationModeSetSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn destination_mode_set(&mut self) -> DestinationModeSetW<DDestinationModeSetSpec> {
        DestinationModeSetW::new(self, 0)
    }
}
#[doc = "Write to set destination mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d_destination_mode_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDestinationModeSetSpec;
impl crate::RegisterSpec for DDestinationModeSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`d_destination_mode_set::W`](W) writer structure"]
impl crate::Writable for DDestinationModeSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D_DESTINATION_MODE_SET to value 0"]
impl crate::Resettable for DDestinationModeSetSpec {
    const RESET_VALUE: u32 = 0;
}
