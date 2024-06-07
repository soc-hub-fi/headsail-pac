#[doc = "Register `SOURCE_ADDR` reader"]
pub type R = crate::R<SourceAddrSpec>;
#[doc = "Register `SOURCE_ADDR` writer"]
pub type W = crate::W<SourceAddrSpec>;
#[doc = "Field `Source_Address` reader - "]
pub type SourceAddressR = crate::FieldReader<u32>;
#[doc = "Field `Source_Address` writer - "]
pub type SourceAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_address(&self) -> SourceAddressR {
        SourceAddressR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOURCE_ADDR")
            .field("source_address", &self.source_address())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_address(&mut self) -> SourceAddressW<SourceAddrSpec> {
        SourceAddressW::new(self, 0)
    }
}
#[doc = "Start address for read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SourceAddrSpec;
impl crate::RegisterSpec for SourceAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`source_addr::R`](R) reader structure"]
impl crate::Readable for SourceAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`source_addr::W`](W) writer structure"]
impl crate::Writable for SourceAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOURCE_ADDR to value 0"]
impl crate::Resettable for SourceAddrSpec {
    const RESET_VALUE: u32 = 0;
}
