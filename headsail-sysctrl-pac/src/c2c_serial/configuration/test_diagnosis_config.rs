#[doc = "Register `TEST_DIAGNOSIS_CONFIG[%s]` reader"]
pub struct R(crate::R<TEST_DIAGNOSIS_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_DIAGNOSIS_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_DIAGNOSIS_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_DIAGNOSIS_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_DIAGNOSIS_CONFIG[%s]` writer"]
pub struct W(crate::W<TEST_DIAGNOSIS_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_DIAGNOSIS_CONFIG_SPEC>;
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
impl From<crate::W<TEST_DIAGNOSIS_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_DIAGNOSIS_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG` reader - "]
pub type CONFIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONFIG` writer - "]
pub type CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEST_DIAGNOSIS_CONFIG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> CONFIG_W<0> {
        CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_diagnosis_config](index.html) module"]
pub struct TEST_DIAGNOSIS_CONFIG_SPEC;
impl crate::RegisterSpec for TEST_DIAGNOSIS_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_diagnosis_config::R](R) reader structure"]
impl crate::Readable for TEST_DIAGNOSIS_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_diagnosis_config::W](W) writer structure"]
impl crate::Writable for TEST_DIAGNOSIS_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_DIAGNOSIS_CONFIG[%s]
to value 0"]
impl crate::Resettable for TEST_DIAGNOSIS_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
