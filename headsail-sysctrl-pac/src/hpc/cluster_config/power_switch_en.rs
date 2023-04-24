#[doc = "Register `power_switch_en` reader"]
pub struct R(crate::R<POWER_SWITCH_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SWITCH_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SWITCH_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SWITCH_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `power_switch_en` writer"]
pub struct W(crate::W<POWER_SWITCH_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_SWITCH_EN_SPEC>;
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
impl From<crate::W<POWER_SWITCH_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_SWITCH_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: `1`"]
    ENABLED = 1,
    #[doc = "0: `0`"]
    DISABLED = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::ENABLED,
            false => EN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
}
#[doc = "Field `en` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u64, POWER_SWITCH_EN_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_switch_en](index.html) module"]
pub struct POWER_SWITCH_EN_SPEC;
impl crate::RegisterSpec for POWER_SWITCH_EN_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [power_switch_en::R](R) reader structure"]
impl crate::Readable for POWER_SWITCH_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_switch_en::W](W) writer structure"]
impl crate::Writable for POWER_SWITCH_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets power_switch_en to value 0"]
impl crate::Resettable for POWER_SWITCH_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
