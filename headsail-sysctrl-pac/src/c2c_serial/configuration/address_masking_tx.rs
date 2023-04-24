#[doc = "Register `ADDRESS_MASKING_TX` reader"]
pub struct R(crate::R<ADDRESS_MASKING_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS_MASKING_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRESS_MASKING_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRESS_MASKING_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS_MASKING_TX` writer"]
pub struct W(crate::W<ADDRESS_MASKING_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS_MASKING_TX_SPEC>;
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
impl From<crate::W<ADDRESS_MASKING_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRESS_MASKING_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_MASKING_TX` reader - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub type ADDRESS_MASKING_TX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_MASKING_TX` writer - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub type ADDRESS_MASKING_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDRESS_MASKING_TX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    pub fn address_masking_tx(&self) -> ADDRESS_MASKING_TX_R {
        ADDRESS_MASKING_TX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    #[inline(always)]
    #[must_use]
    pub fn address_masking_tx(&mut self) -> ADDRESS_MASKING_TX_W<0> {
        ADDRESS_MASKING_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address_masking_tx](index.html) module"]
pub struct ADDRESS_MASKING_TX_SPEC;
impl crate::RegisterSpec for ADDRESS_MASKING_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address_masking_tx::R](R) reader structure"]
impl crate::Readable for ADDRESS_MASKING_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address_masking_tx::W](W) writer structure"]
impl crate::Writable for ADDRESS_MASKING_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRESS_MASKING_TX to value 0xffff_ffff"]
impl crate::Resettable for ADDRESS_MASKING_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
