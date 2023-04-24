#[doc = "Register `int_ack_reg` reader"]
pub struct R(crate::R<INT_ACK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ACK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ACK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ACK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `int_ack_reg` writer"]
pub struct W(crate::W<INT_ACK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ACK_REG_SPEC>;
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
impl From<crate::W<INT_ACK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ACK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spim0_int0_ack` reader - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type SPIM0_INT0_ACK_R = crate::BitReader<bool>;
#[doc = "Field `spim0_int0_ack` writer - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
pub type SPIM0_INT0_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ACK_REG_SPEC, bool, O>;
#[doc = "Field `spim0_int1_ack` reader - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type SPIM0_INT1_ACK_R = crate::BitReader<bool>;
#[doc = "Field `spim0_int1_ack` writer - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
pub type SPIM0_INT1_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ACK_REG_SPEC, bool, O>;
#[doc = "Field `spim1_int0_ack` reader - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type SPIM1_INT0_ACK_R = crate::BitReader<bool>;
#[doc = "Field `spim1_int0_ack` writer - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
pub type SPIM1_INT0_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ACK_REG_SPEC, bool, O>;
#[doc = "Field `spim1_int1_ack` reader - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type SPIM1_INT1_ACK_R = crate::BitReader<bool>;
#[doc = "Field `spim1_int1_ack` writer - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
pub type SPIM1_INT1_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ACK_REG_SPEC, bool, O>;
#[doc = "Field `gpio_int_ack` reader - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GPIO_INT_ACK_R = crate::BitReader<bool>;
#[doc = "Field `gpio_int_ack` writer - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
pub type GPIO_INT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ACK_REG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int0_ack(&self) -> SPIM0_INT0_ACK_R {
        SPIM0_INT0_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim0_int1_ack(&self) -> SPIM0_INT1_ACK_R {
        SPIM0_INT1_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int0_ack(&self) -> SPIM1_INT0_ACK_R {
        SPIM1_INT0_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn spim1_int1_ack(&self) -> SPIM1_INT1_ACK_R {
        SPIM1_INT1_ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    pub fn gpio_int_ack(&self) -> GPIO_INT_ACK_R {
        GPIO_INT_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge that the spim0_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim0_int0_ack(&mut self) -> SPIM0_INT0_ACK_W<0> {
        SPIM0_INT0_ACK_W::new(self)
    }
    #[doc = "Bit 1 - Acknowledge that the spim0_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim0_int1_ack(&mut self) -> SPIM0_INT1_ACK_W<1> {
        SPIM0_INT1_ACK_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge that the spim1_events0 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int0_ack(&mut self) -> SPIM1_INT0_ACK_W<2> {
        SPIM1_INT0_ACK_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledge that the spim1_events1 is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spim1_int1_ack(&mut self) -> SPIM1_INT1_ACK_W<3> {
        SPIM1_INT1_ACK_W::new(self)
    }
    #[doc = "Bit 4 - Acknowledge that the gpio irq is handled to de-assert the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int_ack(&mut self) -> GPIO_INT_ACK_W<4> {
        GPIO_INT_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ack_reg](index.html) module"]
pub struct INT_ACK_REG_SPEC;
impl crate::RegisterSpec for INT_ACK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ack_reg::R](R) reader structure"]
impl crate::Readable for INT_ACK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ack_reg::W](W) writer structure"]
impl crate::Writable for INT_ACK_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets int_ack_reg to value 0"]
impl crate::Resettable for INT_ACK_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
