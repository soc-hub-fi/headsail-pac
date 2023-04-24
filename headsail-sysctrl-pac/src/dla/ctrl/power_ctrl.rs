#[doc = "Register `power_ctrl` reader"]
pub struct R(crate::R<POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `power_ctrl` writer"]
pub struct W(crate::W<POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CTRL_SPEC>;
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
impl From<crate::W<POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `down1` reader - "]
pub type DOWN1_R = crate::BitReader<bool>;
#[doc = "Field `down1` writer - "]
pub type DOWN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `down2` reader - "]
pub type DOWN2_R = crate::BitReader<bool>;
#[doc = "Field `down2` writer - "]
pub type DOWN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `iso` reader - "]
pub type ISO_R = crate::BitReader<bool>;
#[doc = "Field `iso` writer - "]
pub type ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn down1(&self) -> DOWN1_R {
        DOWN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn down2(&self) -> DOWN2_R {
        DOWN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn down1(&mut self) -> DOWN1_W<0> {
        DOWN1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn down2(&mut self) -> DOWN2_W<1> {
        DOWN2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<2> {
        ISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_ctrl](index.html) module"]
pub struct POWER_CTRL_SPEC;
impl crate::RegisterSpec for POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_ctrl::R](R) reader structure"]
impl crate::Readable for POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_ctrl::W](W) writer structure"]
impl crate::Writable for POWER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets power_ctrl to value 0"]
impl crate::Resettable for POWER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
