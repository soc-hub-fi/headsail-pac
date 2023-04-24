#[doc = "Register `bp_en` reader"]
pub struct R(crate::R<BP_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BP_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BP_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bp_en` writer"]
pub struct W(crate::W<BP_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BP_EN_SPEC>;
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
impl From<crate::W<BP_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BP_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `single_step_bp_en` reader - "]
pub type SINGLE_STEP_BP_EN_R = crate::BitReader<bool>;
#[doc = "Field `single_step_bp_en` writer - "]
pub type SINGLE_STEP_BP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BP_EN_SPEC, bool, O>;
#[doc = "Field `enable_breakpoint_1` reader - "]
pub type ENABLE_BREAKPOINT_1_R = crate::BitReader<bool>;
#[doc = "Field `enable_breakpoint_1` writer - "]
pub type ENABLE_BREAKPOINT_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BP_EN_SPEC, bool, O>;
#[doc = "Field `enable_breakpoint_2` reader - "]
pub type ENABLE_BREAKPOINT_2_R = crate::BitReader<bool>;
#[doc = "Field `enable_breakpoint_2` writer - "]
pub type ENABLE_BREAKPOINT_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BP_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn single_step_bp_en(&self) -> SINGLE_STEP_BP_EN_R {
        SINGLE_STEP_BP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_breakpoint_1(&self) -> ENABLE_BREAKPOINT_1_R {
        ENABLE_BREAKPOINT_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn enable_breakpoint_2(&self) -> ENABLE_BREAKPOINT_2_R {
        ENABLE_BREAKPOINT_2_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn single_step_bp_en(&mut self) -> SINGLE_STEP_BP_EN_W<2> {
        SINGLE_STEP_BP_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_1(&mut self) -> ENABLE_BREAKPOINT_1_W<3> {
        ENABLE_BREAKPOINT_1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_2(&mut self) -> ENABLE_BREAKPOINT_2_W<4> {
        ENABLE_BREAKPOINT_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bp_en](index.html) module"]
pub struct BP_EN_SPEC;
impl crate::RegisterSpec for BP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bp_en::R](R) reader structure"]
impl crate::Readable for BP_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bp_en::W](W) writer structure"]
impl crate::Writable for BP_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bp_en to value 0"]
impl crate::Resettable for BP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
