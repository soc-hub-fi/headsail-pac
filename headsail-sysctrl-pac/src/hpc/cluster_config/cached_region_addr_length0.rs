#[doc = "Register `cached_region_addr_length0` reader"]
pub struct R(crate::R<CACHED_REGION_ADDR_LENGTH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHED_REGION_ADDR_LENGTH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHED_REGION_ADDR_LENGTH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHED_REGION_ADDR_LENGTH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cached_region_addr_length0` writer"]
pub struct W(crate::W<CACHED_REGION_ADDR_LENGTH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHED_REGION_ADDR_LENGTH0_SPEC>;
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
impl From<crate::W<CACHED_REGION_ADDR_LENGTH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHED_REGION_ADDR_LENGTH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `len` reader - "]
pub type LEN_R = crate::FieldReader<u64, u64>;
#[doc = "Field `len` writer - "]
pub type LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, CACHED_REGION_ADDR_LENGTH0_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cached_region_addr_length0](index.html) module"]
pub struct CACHED_REGION_ADDR_LENGTH0_SPEC;
impl crate::RegisterSpec for CACHED_REGION_ADDR_LENGTH0_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [cached_region_addr_length0::R](R) reader structure"]
impl crate::Readable for CACHED_REGION_ADDR_LENGTH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cached_region_addr_length0::W](W) writer structure"]
impl crate::Writable for CACHED_REGION_ADDR_LENGTH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cached_region_addr_length0 to value 0"]
impl crate::Resettable for CACHED_REGION_ADDR_LENGTH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
