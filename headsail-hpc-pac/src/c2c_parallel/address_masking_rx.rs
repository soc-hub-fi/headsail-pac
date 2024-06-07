#[doc = "Register `ADDRESS_MASKING_RX` reader"]
pub type R = crate::R<AddressMaskingRxSpec>;
#[doc = "Register `ADDRESS_MASKING_RX` writer"]
pub type W = crate::W<AddressMaskingRxSpec>;
#[doc = "Field `ADDRESS_MASKING_RX` reader - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub type AddressMaskingRxR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS_MASKING_RX` writer - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub type AddressMaskingRxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub fn address_masking_rx(&self) -> AddressMaskingRxR {
        AddressMaskingRxR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRESS_MASKING_RX")
            .field("address_masking_rx", &self.address_masking_rx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    #[must_use]
    pub fn address_masking_rx(&mut self) -> AddressMaskingRxW<AddressMaskingRxSpec> {
        AddressMaskingRxW::new(self, 0)
    }
}
#[doc = "Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) &lt;=address_in(i) address_masking (i) = 0 : address_out(i) &lt;= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address_masking_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address_masking_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressMaskingRxSpec;
impl crate::RegisterSpec for AddressMaskingRxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address_masking_rx::R`](R) reader structure"]
impl crate::Readable for AddressMaskingRxSpec {}
#[doc = "`write(|w| ..)` method takes [`address_masking_rx::W`](W) writer structure"]
impl crate::Writable for AddressMaskingRxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS_MASKING_RX to value 0xffff_ffff"]
impl crate::Resettable for AddressMaskingRxSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
