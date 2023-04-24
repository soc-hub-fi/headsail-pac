#[doc = "Register `external_memory_len` reader"]
pub struct R(crate::R<EXTERNAL_MEMORY_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTERNAL_MEMORY_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTERNAL_MEMORY_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTERNAL_MEMORY_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `external_memory_len` writer"]
pub struct W(crate::W<EXTERNAL_MEMORY_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTERNAL_MEMORY_LEN_SPEC>;
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
impl From<crate::W<EXTERNAL_MEMORY_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTERNAL_MEMORY_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `len` reader - "]
pub type LEN_R = crate::FieldReader<u64, u64>;
#[doc = "Field `len` writer - "]
pub type LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, EXTERNAL_MEMORY_LEN_SPEC, u64, u64, 64, O>;
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
#[doc = "Original memory map calls this dram_addr_length but the register has been repurposed\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [external_memory_len](index.html) module"]
pub struct EXTERNAL_MEMORY_LEN_SPEC;
impl crate::RegisterSpec for EXTERNAL_MEMORY_LEN_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [external_memory_len::R](R) reader structure"]
impl crate::Readable for EXTERNAL_MEMORY_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [external_memory_len::W](W) writer structure"]
impl crate::Writable for EXTERNAL_MEMORY_LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets external_memory_len to value 0x0010_0000_0000"]
impl crate::Resettable for EXTERNAL_MEMORY_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000_0000;
}
