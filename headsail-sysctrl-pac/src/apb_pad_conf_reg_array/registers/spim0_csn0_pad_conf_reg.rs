#[doc = "Register `spim0_csn0_pad_conf_reg` reader"]
pub struct R(crate::R<SPIM0_CSN0_PAD_CONF_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM0_CSN0_PAD_CONF_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM0_CSN0_PAD_CONF_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM0_CSN0_PAD_CONF_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spim0_csn0_pad_conf_reg` writer"]
pub struct W(crate::W<SPIM0_CSN0_PAD_CONF_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM0_CSN0_PAD_CONF_REG_SPEC>;
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
impl From<crate::W<SPIM0_CSN0_PAD_CONF_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM0_CSN0_PAD_CONF_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS0` reader - "]
pub type DS0_R = crate::BitReader<bool>;
#[doc = "Field `DS0` writer - "]
pub type DS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `DS1` reader - "]
pub type DS1_R = crate::BitReader<bool>;
#[doc = "Field `DS1` writer - "]
pub type DS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `ST0` reader - "]
pub type ST0_R = crate::BitReader<bool>;
#[doc = "Field `ST0` writer - "]
pub type ST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `ST1` reader - "]
pub type ST1_R = crate::BitReader<bool>;
#[doc = "Field `ST1` writer - "]
pub type ST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `RATE_CONTROL` reader - "]
pub type RATE_CONTROL_R = crate::BitReader<bool>;
#[doc = "Field `RATE_CONTROL` writer - "]
pub type RATE_CONTROL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `OUTPUT_EN` reader - "]
pub type OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUTPUT_EN` writer - "]
pub type OUTPUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `HOLD` reader - "]
pub type HOLD_R = crate::BitReader<bool>;
#[doc = "Field `HOLD` writer - "]
pub type HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `PULL_ENABLE` reader - "]
pub type PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PULL_ENABLE` writer - "]
pub type PULL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `PULL_DIR` reader - "]
pub type PULL_DIR_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DIR` writer - "]
pub type PULL_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
#[doc = "Field `INPUT_EN` reader - "]
pub type INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INPUT_EN` writer - "]
pub type INPUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPIM0_CSN0_PAD_CONF_REG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rate_control(&self) -> RATE_CONTROL_R {
        RATE_CONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_en(&self) -> OUTPUT_EN_R {
        OUTPUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_dir(&self) -> PULL_DIR_R {
        PULL_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_en(&self) -> INPUT_EN_R {
        INPUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ds0(&mut self) -> DS0_W<0> {
        DS0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ds1(&mut self) -> DS1_W<1> {
        DS1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<2> {
        ST0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<3> {
        ST1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rate_control(&mut self) -> RATE_CONTROL_W<4> {
        RATE_CONTROL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_en(&mut self) -> OUTPUT_EN_W<5> {
        OUTPUT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HOLD_W<6> {
        HOLD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W<7> {
        PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pull_dir(&mut self) -> PULL_DIR_W<8> {
        PULL_DIR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn input_en(&mut self) -> INPUT_EN_W<9> {
        INPUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim0_csn0_pad_conf_reg](index.html) module"]
pub struct SPIM0_CSN0_PAD_CONF_REG_SPEC;
impl crate::RegisterSpec for SPIM0_CSN0_PAD_CONF_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim0_csn0_pad_conf_reg::R](R) reader structure"]
impl crate::Readable for SPIM0_CSN0_PAD_CONF_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim0_csn0_pad_conf_reg::W](W) writer structure"]
impl crate::Writable for SPIM0_CSN0_PAD_CONF_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spim0_csn0_pad_conf_reg to value 0"]
impl crate::Resettable for SPIM0_CSN0_PAD_CONF_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
