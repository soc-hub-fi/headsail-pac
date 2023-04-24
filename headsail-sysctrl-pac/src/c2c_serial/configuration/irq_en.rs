#[doc = "Register `IRQ_EN` reader"]
pub struct R(crate::R<IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_EN` writer"]
pub struct W(crate::W<IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_EN_SPEC>;
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
impl From<crate::W<IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr_complete` reader - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `wr_complete` writer - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_EN_SPEC, bool, O>;
#[doc = "Field `rd_complete` reader - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `rd_complete` writer - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_EN_SPEC, bool, O>;
#[doc = "Field `wr_error_0` reader - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_ERROR_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_error_0` writer - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_ERROR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_EN_SPEC, bool, O>;
#[doc = "Field `rd_error` reader - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `rd_error` writer - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn wr_complete(&self) -> WR_COMPLETE_R {
        WR_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn rd_complete(&self) -> RD_COMPLETE_R {
        RD_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn wr_error_0(&self) -> WR_ERROR_0_R {
        WR_ERROR_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn rd_error(&self) -> RD_ERROR_R {
        RD_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_complete(&mut self) -> WR_COMPLETE_W<0> {
        WR_COMPLETE_W::new(self)
    }
    #[doc = "Bit 1 - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_complete(&mut self) -> RD_COMPLETE_W<1> {
        RD_COMPLETE_W::new(self)
    }
    #[doc = "Bit 2 - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_error_0(&mut self) -> WR_ERROR_0_W<2> {
        WR_ERROR_0_W::new(self)
    }
    #[doc = "Bit 3 - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_error(&mut self) -> RD_ERROR_W<3> {
        RD_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_en](index.html) module"]
pub struct IRQ_EN_SPEC;
impl crate::RegisterSpec for IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_en::R](R) reader structure"]
impl crate::Readable for IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_en::W](W) writer structure"]
impl crate::Writable for IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
