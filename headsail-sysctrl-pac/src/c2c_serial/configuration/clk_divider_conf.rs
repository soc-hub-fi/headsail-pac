#[doc = "Register `CLK_DIVIDER_CONF` reader"]
pub struct R(crate::R<CLK_DIVIDER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIVIDER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIVIDER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIVIDER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIVIDER_CONF` writer"]
pub struct W(crate::W<CLK_DIVIDER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIVIDER_CONF_SPEC>;
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
impl From<crate::W<CLK_DIVIDER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIVIDER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVIDER` reader - "]
pub type DIVIDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVIDER` writer - "]
pub type DIVIDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_DIVIDER_CONF_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<0> {
        DIVIDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_divider_conf](index.html) module"]
pub struct CLK_DIVIDER_CONF_SPEC;
impl crate::RegisterSpec for CLK_DIVIDER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_divider_conf::R](R) reader structure"]
impl crate::Readable for CLK_DIVIDER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_divider_conf::W](W) writer structure"]
impl crate::Writable for CLK_DIVIDER_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_DIVIDER_CONF to value 0"]
impl crate::Resettable for CLK_DIVIDER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
