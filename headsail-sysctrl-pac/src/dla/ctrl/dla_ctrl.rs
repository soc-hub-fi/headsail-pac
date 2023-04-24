#[doc = "Register `dla_ctrl` reader"]
pub struct R(crate::R<DLA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dla_ctrl` writer"]
pub struct W(crate::W<DLA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLA_CTRL_SPEC>;
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
impl From<crate::W<DLA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_fe` reader - "]
pub type CPU_FE_R = crate::BitReader<bool>;
#[doc = "Field `cpu_fe` writer - "]
pub type CPU_FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLA_CTRL_SPEC, bool, O>;
#[doc = "Field `hpRst` reader - "]
pub type HP_RST_R = crate::BitReader<bool>;
#[doc = "Field `hpRst` writer - "]
pub type HP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLA_CTRL_SPEC, bool, O>;
#[doc = "Field `sw_irq` reader - "]
pub type SW_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `sw_irq` writer - "]
pub type SW_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLA_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_fe(&self) -> CPU_FE_R {
        CPU_FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hp_rst(&self) -> HP_RST_R {
        HP_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_irq(&self) -> SW_IRQ_R {
        SW_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_fe(&mut self) -> CPU_FE_W<0> {
        CPU_FE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hp_rst(&mut self) -> HP_RST_W<4> {
        HP_RST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irq(&mut self) -> SW_IRQ_W<8> {
        SW_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dla_ctrl](index.html) module"]
pub struct DLA_CTRL_SPEC;
impl crate::RegisterSpec for DLA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dla_ctrl::R](R) reader structure"]
impl crate::Readable for DLA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dla_ctrl::W](W) writer structure"]
impl crate::Writable for DLA_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dla_ctrl to value 0"]
impl crate::Resettable for DLA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
