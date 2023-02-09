#[doc = "Register `t_ras` reader"]
pub struct R(crate::R<T_RAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T_RAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T_RAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T_RAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `t_ras` writer"]
pub struct W(crate::W<T_RAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T_RAS_SPEC>;
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
impl From<crate::W<T_RAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T_RAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `t_ras` reader - "]
pub type T_RAS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `t_ras` writer - "]
pub type T_RAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T_RAS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_ras(&self) -> T_RAS_R {
        T_RAS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_ras(&mut self) -> T_RAS_W<0> {
        T_RAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t_ras](index.html) module"]
pub struct T_RAS_SPEC;
impl crate::RegisterSpec for T_RAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t_ras::R](R) reader structure"]
impl crate::Readable for T_RAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t_ras::W](W) writer structure"]
impl crate::Writable for T_RAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets t_ras to value 0x03"]
impl crate::Resettable for T_RAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
