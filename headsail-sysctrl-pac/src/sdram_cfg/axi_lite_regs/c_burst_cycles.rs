#[doc = "Register `c_burst_cycles` reader"]
pub struct R(crate::R<C_BURST_CYCLES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C_BURST_CYCLES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C_BURST_CYCLES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C_BURST_CYCLES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c_burst_cycles` writer"]
pub struct W(crate::W<C_BURST_CYCLES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C_BURST_CYCLES_SPEC>;
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
impl From<crate::W<C_BURST_CYCLES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C_BURST_CYCLES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c_burst_cycles` reader - "]
pub type C_BURST_CYCLES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `c_burst_cycles` writer - "]
pub type C_BURST_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, C_BURST_CYCLES_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_burst_cycles(&self) -> C_BURST_CYCLES_R {
        C_BURST_CYCLES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_burst_cycles(&mut self) -> C_BURST_CYCLES_W<0> {
        C_BURST_CYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_burst_cycles](index.html) module"]
pub struct C_BURST_CYCLES_SPEC;
impl crate::RegisterSpec for C_BURST_CYCLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c_burst_cycles::R](R) reader structure"]
impl crate::Readable for C_BURST_CYCLES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c_burst_cycles::W](W) writer structure"]
impl crate::Writable for C_BURST_CYCLES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets c_burst_cycles to value 0x01"]
impl crate::Resettable for C_BURST_CYCLES_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
