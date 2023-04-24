#[doc = "Register `IRQ_CONFIG` reader"]
pub struct R(crate::R<IRQ_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_CONFIG` writer"]
pub struct W(crate::W<IRQ_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_CONFIG_SPEC>;
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
impl From<crate::W<IRQ_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Clear` reader - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `Clear` writer - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Polarity` reader - Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `Polarity` writer - Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Polarity: '0' = IRQ active low '1' = IRQ active high"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<1> {
        POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_config](index.html) module"]
pub struct IRQ_CONFIG_SPEC;
impl crate::RegisterSpec for IRQ_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_config::R](R) reader structure"]
impl crate::Readable for IRQ_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_config::W](W) writer structure"]
impl crate::Writable for IRQ_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_CONFIG to value 0"]
impl crate::Resettable for IRQ_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
