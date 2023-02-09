#[doc = "Register `ADDRESS_MAPPING_TX` reader"]
pub struct R(crate::R<ADDRESS_MAPPING_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS_MAPPING_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRESS_MAPPING_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRESS_MAPPING_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS_MAPPING_TX` writer"]
pub struct W(crate::W<ADDRESS_MAPPING_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS_MAPPING_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADDRESS_MAPPING_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRESS_MAPPING_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_MAPPING_TX` reader - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub type ADDRESS_MAPPING_TX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_MAPPING_TX` writer - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub type ADDRESS_MAPPING_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDRESS_MAPPING_TX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    pub fn address_mapping_tx(&self) -> ADDRESS_MAPPING_TX_R {
        ADDRESS_MAPPING_TX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    #[inline(always)]
    #[must_use]
    pub fn address_mapping_tx(&mut self) -> ADDRESS_MAPPING_TX_W<0> {
        ADDRESS_MAPPING_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address_mapping_tx](index.html) module"]
pub struct ADDRESS_MAPPING_TX_SPEC;
impl crate::RegisterSpec for ADDRESS_MAPPING_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address_mapping_tx::R](R) reader structure"]
impl crate::Readable for ADDRESS_MAPPING_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address_mapping_tx::W](W) writer structure"]
impl crate::Writable for ADDRESS_MAPPING_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRESS_MAPPING_TX to value 0"]
impl crate::Resettable for ADDRESS_MAPPING_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
