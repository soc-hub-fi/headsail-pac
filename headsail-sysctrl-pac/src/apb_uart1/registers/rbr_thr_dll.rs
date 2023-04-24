#[doc = "Register `RBR_THR_DLL` reader"]
pub struct R(crate::R<RBR_THR_DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_THR_DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_THR_DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_THR_DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBR_THR_DLL` writer"]
pub struct W(crate::W<RBR_THR_DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBR_THR_DLL_SPEC>;
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
impl From<crate::W<RBR_THR_DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBR_THR_DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBR_THR_DLL` reader - "]
pub type RBR_THR_DLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBR_THR_DLL` writer - "]
pub type RBR_THR_DLL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, RBR_THR_DLL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr_thr_dll(&self) -> RBR_THR_DLL_R {
        RBR_THR_DLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rbr_thr_dll(&mut self) -> RBR_THR_DLL_W<0> {
        RBR_THR_DLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RBR_THR_DLL is a multi-purpose register address. you can access 3 different registers using the same address. IF LCR\\[7\\]
is 0 RBR and THR are accessable. OW DLL is accessable. RBR read only. Reads from rx fifo THR write only. Writes into a tx fifo DLL writes/reads into/from the 8 LSBs in the divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr_thr_dll](index.html) module"]
pub struct RBR_THR_DLL_SPEC;
impl crate::RegisterSpec for RBR_THR_DLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rbr_thr_dll::R](R) reader structure"]
impl crate::Readable for RBR_THR_DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbr_thr_dll::W](W) writer structure"]
impl crate::Writable for RBR_THR_DLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBR_THR_DLL to value 0"]
impl crate::Resettable for RBR_THR_DLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
