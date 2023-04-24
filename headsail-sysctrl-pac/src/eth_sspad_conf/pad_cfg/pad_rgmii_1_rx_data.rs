#[doc = "Register `PAD_RGMII_1_RX_DATA` reader"]
pub struct R(crate::R<PAD_RGMII_1_RX_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_RGMII_1_RX_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_RGMII_1_RX_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_RGMII_1_RX_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_RGMII_1_RX_DATA` writer"]
pub struct W(crate::W<PAD_RGMII_1_RX_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_RGMII_1_RX_DATA_SPEC>;
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
impl From<crate::W<PAD_RGMII_1_RX_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_RGMII_1_RX_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS0` reader - "]
pub type DS0_R = crate::BitReader<DS0_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS0_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<DS0_A> for bool {
    #[inline(always)]
    fn from(variant: DS0_A) -> Self {
        variant as u8 != 0
    }
}
impl DS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS0_A {
        match self.bits {
            true => DS0_A::ENABLE,
            false => DS0_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DS0_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DS0_A::DISABLE
    }
}
#[doc = "Field `DS0` writer - "]
pub type DS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, DS0_A, O>;
impl<'a, const O: u8> DS0_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DS0_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DS0_A::DISABLE)
    }
}
#[doc = "Field `DS1` reader - "]
pub type DS1_R = crate::BitReader<DS1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS1_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<DS1_A> for bool {
    #[inline(always)]
    fn from(variant: DS1_A) -> Self {
        variant as u8 != 0
    }
}
impl DS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS1_A {
        match self.bits {
            true => DS1_A::ENABLE,
            false => DS1_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DS1_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DS1_A::DISABLE
    }
}
#[doc = "Field `DS1` writer - "]
pub type DS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, DS1_A, O>;
impl<'a, const O: u8> DS1_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DS1_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DS1_A::DISABLE)
    }
}
#[doc = "Field `ST0` reader - "]
pub type ST0_R = crate::BitReader<ST0_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST0_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ST0_A> for bool {
    #[inline(always)]
    fn from(variant: ST0_A) -> Self {
        variant as u8 != 0
    }
}
impl ST0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST0_A {
        match self.bits {
            false => ST0_A::DISABLE,
            true => ST0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ST0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ST0_A::ENABLE
    }
}
#[doc = "Field `ST0` writer - "]
pub type ST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, ST0_A, O>;
impl<'a, const O: u8> ST0_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ST0_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ST0_A::ENABLE)
    }
}
#[doc = "Field `ST1` reader - "]
pub type ST1_R = crate::BitReader<ST1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST1_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<ST1_A> for bool {
    #[inline(always)]
    fn from(variant: ST1_A) -> Self {
        variant as u8 != 0
    }
}
impl ST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST1_A {
        match self.bits {
            true => ST1_A::ENABLE,
            false => ST1_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ST1_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ST1_A::DISABLE
    }
}
#[doc = "Field `ST1` writer - "]
pub type ST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, ST1_A, O>;
impl<'a, const O: u8> ST1_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ST1_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ST1_A::DISABLE)
    }
}
#[doc = "Field `RATE_CONTROL` reader - "]
pub type RATE_CONTROL_R = crate::BitReader<RATE_CONTROL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RATE_CONTROL_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<RATE_CONTROL_A> for bool {
    #[inline(always)]
    fn from(variant: RATE_CONTROL_A) -> Self {
        variant as u8 != 0
    }
}
impl RATE_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATE_CONTROL_A {
        match self.bits {
            true => RATE_CONTROL_A::ENABLE,
            false => RATE_CONTROL_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RATE_CONTROL_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RATE_CONTROL_A::DISABLE
    }
}
#[doc = "Field `RATE_CONTROL` writer - "]
pub type RATE_CONTROL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, RATE_CONTROL_A, O>;
impl<'a, const O: u8> RATE_CONTROL_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RATE_CONTROL_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RATE_CONTROL_A::DISABLE)
    }
}
#[doc = "Field `OUTPUT_EN` reader - "]
pub type OUTPUT_EN_R = crate::BitReader<OUTPUT_EN_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_EN_A {
    #[doc = "1: `1`"]
    DISABLE_OUTPUT = 1,
    #[doc = "0: `0`"]
    ENABLE_OUTPUT = 0,
}
impl From<OUTPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTPUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUT_EN_A {
        match self.bits {
            true => OUTPUT_EN_A::DISABLE_OUTPUT,
            false => OUTPUT_EN_A::ENABLE_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_disable_output(&self) -> bool {
        *self == OUTPUT_EN_A::DISABLE_OUTPUT
    }
    #[doc = "Checks if the value of the field is `ENABLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_enable_output(&self) -> bool {
        *self == OUTPUT_EN_A::ENABLE_OUTPUT
    }
}
#[doc = "Field `OUTPUT_EN` writer - "]
pub type OUTPUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, OUTPUT_EN_A, O>;
impl<'a, const O: u8> OUTPUT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable_output(self) -> &'a mut W {
        self.variant(OUTPUT_EN_A::DISABLE_OUTPUT)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable_output(self) -> &'a mut W {
        self.variant(OUTPUT_EN_A::ENABLE_OUTPUT)
    }
}
#[doc = "Field `HOLD` reader - "]
pub type HOLD_R = crate::BitReader<HOLD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOLD_A {
    #[doc = "1: `1`"]
    DISABLE_PAD = 1,
    #[doc = "0: `0`"]
    ENABLE_PAD = 0,
}
impl From<HOLD_A> for bool {
    #[inline(always)]
    fn from(variant: HOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl HOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOLD_A {
        match self.bits {
            true => HOLD_A::DISABLE_PAD,
            false => HOLD_A::ENABLE_PAD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_PAD`"]
    #[inline(always)]
    pub fn is_disable_pad(&self) -> bool {
        *self == HOLD_A::DISABLE_PAD
    }
    #[doc = "Checks if the value of the field is `ENABLE_PAD`"]
    #[inline(always)]
    pub fn is_enable_pad(&self) -> bool {
        *self == HOLD_A::ENABLE_PAD
    }
}
#[doc = "Field `HOLD` writer - "]
pub type HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, HOLD_A, O>;
impl<'a, const O: u8> HOLD_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable_pad(self) -> &'a mut W {
        self.variant(HOLD_A::DISABLE_PAD)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable_pad(self) -> &'a mut W {
        self.variant(HOLD_A::ENABLE_PAD)
    }
}
#[doc = "Field `PULL_ENABLE` reader - "]
pub type PULL_ENABLE_R = crate::BitReader<PULL_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULL_ENABLE_A {
    #[doc = "1: `1`"]
    ENABLE_PULL = 1,
    #[doc = "0: `0`"]
    DISABLE_PULL = 0,
}
impl From<PULL_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PULL_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PULL_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULL_ENABLE_A {
        match self.bits {
            true => PULL_ENABLE_A::ENABLE_PULL,
            false => PULL_ENABLE_A::DISABLE_PULL,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_PULL`"]
    #[inline(always)]
    pub fn is_enable_pull(&self) -> bool {
        *self == PULL_ENABLE_A::ENABLE_PULL
    }
    #[doc = "Checks if the value of the field is `DISABLE_PULL`"]
    #[inline(always)]
    pub fn is_disable_pull(&self) -> bool {
        *self == PULL_ENABLE_A::DISABLE_PULL
    }
}
#[doc = "Field `PULL_ENABLE` writer - "]
pub type PULL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, PULL_ENABLE_A, O>;
impl<'a, const O: u8> PULL_ENABLE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable_pull(self) -> &'a mut W {
        self.variant(PULL_ENABLE_A::ENABLE_PULL)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable_pull(self) -> &'a mut W {
        self.variant(PULL_ENABLE_A::DISABLE_PULL)
    }
}
#[doc = "Field `PULL_DIR` reader - "]
pub type PULL_DIR_R = crate::BitReader<PULL_DIR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PULL_DIR_A {
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "0: `0`"]
    PULL_DOWN = 0,
}
impl From<PULL_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: PULL_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl PULL_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULL_DIR_A {
        match self.bits {
            true => PULL_DIR_A::PULL_UP,
            false => PULL_DIR_A::PULL_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PULL_DIR_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PULL_DIR_A::PULL_DOWN
    }
}
#[doc = "Field `PULL_DIR` writer - "]
pub type PULL_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, PULL_DIR_A, O>;
impl<'a, const O: u8> PULL_DIR_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PULL_DIR_A::PULL_UP)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PULL_DIR_A::PULL_DOWN)
    }
}
#[doc = "Field `INPUT_EN` reader - "]
pub type INPUT_EN_R = crate::BitReader<INPUT_EN_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_EN_A {
    #[doc = "1: `1`"]
    ENABLE_INPUT = 1,
    #[doc = "0: `0`"]
    DISABLE_INPUT = 0,
}
impl From<INPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_EN_A {
        match self.bits {
            true => INPUT_EN_A::ENABLE_INPUT,
            false => INPUT_EN_A::DISABLE_INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_INPUT`"]
    #[inline(always)]
    pub fn is_enable_input(&self) -> bool {
        *self == INPUT_EN_A::ENABLE_INPUT
    }
    #[doc = "Checks if the value of the field is `DISABLE_INPUT`"]
    #[inline(always)]
    pub fn is_disable_input(&self) -> bool {
        *self == INPUT_EN_A::DISABLE_INPUT
    }
}
#[doc = "Field `INPUT_EN` writer - "]
pub type INPUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_RGMII_1_RX_DATA_SPEC, INPUT_EN_A, O>;
impl<'a, const O: u8> INPUT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable_input(self) -> &'a mut W {
        self.variant(INPUT_EN_A::ENABLE_INPUT)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable_input(self) -> &'a mut W {
        self.variant(INPUT_EN_A::DISABLE_INPUT)
    }
}
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_rgmii_1_rx_data](index.html) module"]
pub struct PAD_RGMII_1_RX_DATA_SPEC;
impl crate::RegisterSpec for PAD_RGMII_1_RX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_rgmii_1_rx_data::R](R) reader structure"]
impl crate::Readable for PAD_RGMII_1_RX_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_rgmii_1_rx_data::W](W) writer structure"]
impl crate::Writable for PAD_RGMII_1_RX_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_RGMII_1_RX_DATA to value 0x0224"]
impl crate::Resettable for PAD_RGMII_1_RX_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0224;
}
