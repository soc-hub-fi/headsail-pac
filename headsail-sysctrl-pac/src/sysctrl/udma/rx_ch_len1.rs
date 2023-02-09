#[doc = "Register `RX_CH_LEN1` reader"]
pub struct R(crate::R<RX_CH_LEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CH_LEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CH_LEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CH_LEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CH_LEN1` writer"]
pub struct W(crate::W<RX_CH_LEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CH_LEN1_SPEC>;
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
impl From<crate::W<RX_CH_LEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CH_LEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_RX_CH_LEN1` reader - "]
pub type REG_RX_CH_LEN1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG_RX_CH_LEN1` writer - "]
pub type REG_RX_CH_LEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CH_LEN1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_rx_ch_len1(&self) -> REG_RX_CH_LEN1_R {
        REG_RX_CH_LEN1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_ch_len1(&mut self) -> REG_RX_CH_LEN1_W<0> {
        REG_RX_CH_LEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER RX channel length1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ch_len1](index.html) module"]
pub struct RX_CH_LEN1_SPEC;
impl crate::RegisterSpec for RX_CH_LEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ch_len1::R](R) reader structure"]
impl crate::Readable for RX_CH_LEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ch_len1::W](W) writer structure"]
impl crate::Writable for RX_CH_LEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CH_LEN1 to value 0"]
impl crate::Resettable for RX_CH_LEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
