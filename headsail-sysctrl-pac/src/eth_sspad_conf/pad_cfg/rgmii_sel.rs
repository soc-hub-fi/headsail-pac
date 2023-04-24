#[doc = "Register `RGMII_SEL` reader"]
pub struct R(crate::R<RGMII_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGMII_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGMII_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGMII_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGMII_SEL` writer"]
pub struct W(crate::W<RGMII_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGMII_SEL_SPEC>;
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
impl From<crate::W<RGMII_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGMII_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RGMII_SEL` reader - "]
pub type RGMII_SEL_R = crate::BitReader<RGMII_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGMII_SEL_A {
    #[doc = "0: `0`"]
    SEL_TSN_IP = 0,
    #[doc = "1: `1`"]
    SEL_ETH_IP = 1,
}
impl From<RGMII_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RGMII_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGMII_SEL_A {
        match self.bits {
            false => RGMII_SEL_A::SEL_TSN_IP,
            true => RGMII_SEL_A::SEL_ETH_IP,
        }
    }
    #[doc = "Checks if the value of the field is `SEL_TSN_IP`"]
    #[inline(always)]
    pub fn is_sel_tsn_ip(&self) -> bool {
        *self == RGMII_SEL_A::SEL_TSN_IP
    }
    #[doc = "Checks if the value of the field is `SEL_ETH_IP`"]
    #[inline(always)]
    pub fn is_sel_eth_ip(&self) -> bool {
        *self == RGMII_SEL_A::SEL_ETH_IP
    }
}
#[doc = "Field `RGMII_SEL` writer - "]
pub type RGMII_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGMII_SEL_SPEC, RGMII_SEL_A, O>;
impl<'a, const O: u8> RGMII_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sel_tsn_ip(self) -> &'a mut W {
        self.variant(RGMII_SEL_A::SEL_TSN_IP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sel_eth_ip(self) -> &'a mut W {
        self.variant(RGMII_SEL_A::SEL_ETH_IP)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rgmii_sel(&self) -> RGMII_SEL_R {
        RGMII_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_sel(&mut self) -> RGMII_SEL_W<0> {
        RGMII_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgmii_sel](index.html) module"]
pub struct RGMII_SEL_SPEC;
impl crate::RegisterSpec for RGMII_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgmii_sel::R](R) reader structure"]
impl crate::Readable for RGMII_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgmii_sel::W](W) writer structure"]
impl crate::Writable for RGMII_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGMII_SEL to value 0"]
impl crate::Resettable for RGMII_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
