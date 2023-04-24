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
#[doc = "Field `Clear0` reader - "]
pub type CLEAR0_R = crate::BitReader<bool>;
#[doc = "Field `Clear0` writer - "]
pub type CLEAR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Clear1` reader - "]
pub type CLEAR1_R = crate::BitReader<bool>;
#[doc = "Field `Clear1` writer - "]
pub type CLEAR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Clear2` reader - "]
pub type CLEAR2_R = crate::BitReader<bool>;
#[doc = "Field `Clear2` writer - "]
pub type CLEAR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Clear3` reader - "]
pub type CLEAR3_R = crate::BitReader<bool>;
#[doc = "Field `Clear3` writer - "]
pub type CLEAR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Polarity0` reader - "]
pub type POLARITY0_R = crate::BitReader<bool>;
#[doc = "Field `Polarity0` writer - "]
pub type POLARITY0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Polarity1` reader - "]
pub type POLARITY1_R = crate::BitReader<bool>;
#[doc = "Field `Polarity1` writer - "]
pub type POLARITY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Polarity2` reader - "]
pub type POLARITY2_R = crate::BitReader<bool>;
#[doc = "Field `Polarity2` writer - "]
pub type POLARITY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
#[doc = "Field `Polarity3` reader - "]
pub type POLARITY3_R = crate::BitReader<bool>;
#[doc = "Field `Polarity3` writer - "]
pub type POLARITY3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clear0(&self) -> CLEAR0_R {
        CLEAR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clear1(&self) -> CLEAR1_R {
        CLEAR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clear2(&self) -> CLEAR2_R {
        CLEAR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clear3(&self) -> CLEAR3_R {
        CLEAR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn polarity0(&self) -> POLARITY0_R {
        POLARITY0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn polarity1(&self) -> POLARITY1_R {
        POLARITY1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn polarity2(&self) -> POLARITY2_R {
        POLARITY2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn polarity3(&self) -> POLARITY3_R {
        POLARITY3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clear0(&mut self) -> CLEAR0_W<0> {
        CLEAR0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clear1(&mut self) -> CLEAR1_W<1> {
        CLEAR1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clear2(&mut self) -> CLEAR2_W<2> {
        CLEAR2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clear3(&mut self) -> CLEAR3_W<3> {
        CLEAR3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn polarity0(&mut self) -> POLARITY0_W<4> {
        POLARITY0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn polarity1(&mut self) -> POLARITY1_W<5> {
        POLARITY1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn polarity2(&mut self) -> POLARITY2_W<6> {
        POLARITY2_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn polarity3(&mut self) -> POLARITY3_W<7> {
        POLARITY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Cleared: '0' = Use STATUS_CLEAR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high Set all bits IRQ_CONFIG(7 downto 4) of polarity to \"1111\" for IRQ active high or \"0000\" for IRQ active low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_config](index.html) module"]
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
