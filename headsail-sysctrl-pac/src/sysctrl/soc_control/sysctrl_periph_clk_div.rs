#[doc = "Register `SYSCTRL_PERIPH_CLK_DIV` reader"]
pub struct R(crate::R<SYSCTRL_PERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTRL_PERIPH_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTRL_PERIPH_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTRL_PERIPH_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTRL_PERIPH_CLK_DIV` writer"]
pub struct W(crate::W<SYSCTRL_PERIPH_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL_PERIPH_CLK_DIV_SPEC>;
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
impl From<crate::W<SYSCTRL_PERIPH_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL_PERIPH_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTRL_PERIPH_CLK_DIV` reader - "]
pub type SYSCTRL_PERIPH_CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTRL_PERIPH_CLK_DIV` writer - "]
pub type SYSCTRL_PERIPH_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCTRL_PERIPH_CLK_DIV_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sysctrl_periph_clk_div(&self) -> SYSCTRL_PERIPH_CLK_DIV_R {
        SYSCTRL_PERIPH_CLK_DIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn sysctrl_periph_clk_div(&mut self) -> SYSCTRL_PERIPH_CLK_DIV_W<0> {
        SYSCTRL_PERIPH_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls for the peripheral clock divider within SysCtrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_periph_clk_div](index.html) module"]
pub struct SYSCTRL_PERIPH_CLK_DIV_SPEC;
impl crate::RegisterSpec for SYSCTRL_PERIPH_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysctrl_periph_clk_div::R](R) reader structure"]
impl crate::Readable for SYSCTRL_PERIPH_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctrl_periph_clk_div::W](W) writer structure"]
impl crate::Writable for SYSCTRL_PERIPH_CLK_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCTRL_PERIPH_CLK_DIV to value 0"]
impl crate::Resettable for SYSCTRL_PERIPH_CLK_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
