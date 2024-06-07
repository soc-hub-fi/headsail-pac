#[doc = "Register `ADDRESS_MAPPING_TX` reader"]
pub type R = crate::R<AddressMappingTxSpec>;
#[doc = "Register `ADDRESS_MAPPING_TX` writer"]
pub type W = crate::W<AddressMappingTxSpec>;
#[doc = "Field `ADDRESS_MAPPING_TX` reader - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub type AddressMappingTxR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS_MAPPING_TX` writer - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub type AddressMappingTxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub fn address_mapping_tx(&self) -> AddressMappingTxR {
        AddressMappingTxR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRESS_MAPPING_TX")
            .field("address_mapping_tx", &self.address_mapping_tx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    #[must_use]
    pub fn address_mapping_tx(&mut self) -> AddressMappingTxW<AddressMappingTxSpec> {
        AddressMappingTxW::new(self, 0)
    }
}
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_mapping_tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_mapping_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressMappingTxSpec;
impl crate::RegisterSpec for AddressMappingTxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address_mapping_tx::R`](R) reader structure"]
impl crate::Readable for AddressMappingTxSpec {}
#[doc = "`write(|w| ..)` method takes [`address_mapping_tx::W`](W) writer structure"]
impl crate::Writable for AddressMappingTxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS_MAPPING_TX to value 0"]
impl crate::Resettable for AddressMappingTxSpec {
    const RESET_VALUE: u32 = 0;
}
