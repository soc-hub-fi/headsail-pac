#[doc = "Register `t_rcd` reader"]
pub struct R(crate::R<T_RCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T_RCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T_RCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T_RCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `t_rcd` writer"]
pub struct W(crate::W<T_RCD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T_RCD_SPEC>;
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
impl From<crate::W<T_RCD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T_RCD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `t_rcd` reader - "]
pub type T_RCD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `t_rcd` writer - "]
pub type T_RCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T_RCD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_rcd(&self) -> T_RCD_R {
        T_RCD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_rcd(&mut self) -> T_RCD_W<0> {
        T_RCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t_rcd](index.html) module"]
pub struct T_RCD_SPEC;
impl crate::RegisterSpec for T_RCD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t_rcd::R](R) reader structure"]
impl crate::Readable for T_RCD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t_rcd::W](W) writer structure"]
impl crate::Writable for T_RCD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets t_rcd to value 0x02"]
impl crate::Resettable for T_RCD_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
