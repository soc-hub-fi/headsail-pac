#[doc = "Register `D_SOURCE_ADDR` reader"]
pub struct R(crate::R<D_SOURCE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SOURCE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SOURCE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SOURCE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_SOURCE_ADDR` writer"]
pub struct W(crate::W<D_SOURCE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SOURCE_ADDR_SPEC>;
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
impl From<crate::W<D_SOURCE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SOURCE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Source_Address_0` reader - "]
pub type SOURCE_ADDRESS_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Source_Address_0` writer - "]
pub type SOURCE_ADDRESS_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_SOURCE_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn source_address_0(&self) -> SOURCE_ADDRESS_0_R {
        SOURCE_ADDRESS_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_0(&mut self) -> SOURCE_ADDRESS_0_W<0> {
        SOURCE_ADDRESS_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start address for read\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_source_addr](index.html) module"]
pub struct D_SOURCE_ADDR_SPEC;
impl crate::RegisterSpec for D_SOURCE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_source_addr::R](R) reader structure"]
impl crate::Readable for D_SOURCE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_source_addr::W](W) writer structure"]
impl crate::Writable for D_SOURCE_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_SOURCE_ADDR to value 0"]
impl crate::Resettable for D_SOURCE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
