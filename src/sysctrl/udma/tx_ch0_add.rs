#[doc = "Register `TX_CH0_ADD` reader"]
pub struct R(crate::R<TX_CH0_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CH0_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CH0_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CH0_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CH0_ADD` writer"]
pub struct W(crate::W<TX_CH0_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CH0_ADD_SPEC>;
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
impl From<crate::W<TX_CH0_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CH0_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_TX_CH0_ADD` reader - "]
pub type REG_TX_CH0_ADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG_TX_CH0_ADD` writer - "]
pub type REG_TX_CH0_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CH0_ADD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_tx_ch0_add(&self) -> REG_TX_CH0_ADD_R {
        REG_TX_CH0_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tx_ch0_add(&mut self) -> REG_TX_CH0_ADD_W<0> {
        REG_TX_CH0_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER tx channel address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ch0_add](index.html) module"]
pub struct TX_CH0_ADD_SPEC;
impl crate::RegisterSpec for TX_CH0_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ch0_add::R](R) reader structure"]
impl crate::Readable for TX_CH0_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ch0_add::W](W) writer structure"]
impl crate::Writable for TX_CH0_ADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CH0_ADD to value 0"]
impl crate::Resettable for TX_CH0_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
