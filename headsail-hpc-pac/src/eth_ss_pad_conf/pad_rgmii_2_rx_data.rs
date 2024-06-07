#[doc = "Register `PAD_RGMII_2_RX_DATA` reader"]
pub type R = crate::R<PadRgmii2RxDataSpec>;
#[doc = "Register `PAD_RGMII_2_RX_DATA` writer"]
pub type W = crate::W<PadRgmii2RxDataSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ds0 {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<Ds0> for bool {
    #[inline(always)]
    fn from(variant: Ds0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS0` reader - "]
pub type Ds0R = crate::BitReader<Ds0>;
impl Ds0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds0 {
        match self.bits {
            true => Ds0::Enable,
            false => Ds0::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ds0::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ds0::Disable
    }
}
#[doc = "Field `DS0` writer - "]
pub type Ds0W<'a, REG> = crate::BitWriter<'a, REG, Ds0>;
impl<'a, REG> Ds0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ds0::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ds0::Disable)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ds1 {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<Ds1> for bool {
    #[inline(always)]
    fn from(variant: Ds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS1` reader - "]
pub type Ds1R = crate::BitReader<Ds1>;
impl Ds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ds1 {
        match self.bits {
            true => Ds1::Enable,
            false => Ds1::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ds1::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ds1::Disable
    }
}
#[doc = "Field `DS1` writer - "]
pub type Ds1W<'a, REG> = crate::BitWriter<'a, REG, Ds1>;
impl<'a, REG> Ds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ds1::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ds1::Disable)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St0 {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Enable = 1,
}
impl From<St0> for bool {
    #[inline(always)]
    fn from(variant: St0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST0` reader - "]
pub type St0R = crate::BitReader<St0>;
impl St0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St0 {
        match self.bits {
            false => St0::Disable,
            true => St0::Enable,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == St0::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == St0::Enable
    }
}
#[doc = "Field `ST0` writer - "]
pub type St0W<'a, REG> = crate::BitWriter<'a, REG, St0>;
impl<'a, REG> St0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(St0::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(St0::Enable)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St1 {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<St1> for bool {
    #[inline(always)]
    fn from(variant: St1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST1` reader - "]
pub type St1R = crate::BitReader<St1>;
impl St1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St1 {
        match self.bits {
            true => St1::Enable,
            false => St1::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == St1::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == St1::Disable
    }
}
#[doc = "Field `ST1` writer - "]
pub type St1W<'a, REG> = crate::BitWriter<'a, REG, St1>;
impl<'a, REG> St1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(St1::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(St1::Disable)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateControl {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<RateControl> for bool {
    #[inline(always)]
    fn from(variant: RateControl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATE_CONTROL` reader - "]
pub type RateControlR = crate::BitReader<RateControl>;
impl RateControlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RateControl {
        match self.bits {
            true => RateControl::Enable,
            false => RateControl::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RateControl::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RateControl::Disable
    }
}
#[doc = "Field `RATE_CONTROL` writer - "]
pub type RateControlW<'a, REG> = crate::BitWriter<'a, REG, RateControl>;
impl<'a, REG> RateControlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RateControl::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RateControl::Disable)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputEn {
    #[doc = "1: `1`"]
    DisableOutput = 1,
    #[doc = "0: `0`"]
    EnableOutput = 0,
}
impl From<OutputEn> for bool {
    #[inline(always)]
    fn from(variant: OutputEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_EN` reader - "]
pub type OutputEnR = crate::BitReader<OutputEn>;
impl OutputEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutputEn {
        match self.bits {
            true => OutputEn::DisableOutput,
            false => OutputEn::EnableOutput,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_disable_output(&self) -> bool {
        *self == OutputEn::DisableOutput
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_enable_output(&self) -> bool {
        *self == OutputEn::EnableOutput
    }
}
#[doc = "Field `OUTPUT_EN` writer - "]
pub type OutputEnW<'a, REG> = crate::BitWriter<'a, REG, OutputEn>;
impl<'a, REG> OutputEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable_output(self) -> &'a mut crate::W<REG> {
        self.variant(OutputEn::DisableOutput)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable_output(self) -> &'a mut crate::W<REG> {
        self.variant(OutputEn::EnableOutput)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hold {
    #[doc = "1: `1`"]
    DisablePad = 1,
    #[doc = "0: `0`"]
    EnablePad = 0,
}
impl From<Hold> for bool {
    #[inline(always)]
    fn from(variant: Hold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOLD` reader - "]
pub type HoldR = crate::BitReader<Hold>;
impl HoldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hold {
        match self.bits {
            true => Hold::DisablePad,
            false => Hold::EnablePad,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_disable_pad(&self) -> bool {
        *self == Hold::DisablePad
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_enable_pad(&self) -> bool {
        *self == Hold::EnablePad
    }
}
#[doc = "Field `HOLD` writer - "]
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG, Hold>;
impl<'a, REG> HoldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable_pad(self) -> &'a mut crate::W<REG> {
        self.variant(Hold::DisablePad)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable_pad(self) -> &'a mut crate::W<REG> {
        self.variant(Hold::EnablePad)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PullEnable {
    #[doc = "1: `1`"]
    EnablePull = 1,
    #[doc = "0: `0`"]
    DisablePull = 0,
}
impl From<PullEnable> for bool {
    #[inline(always)]
    fn from(variant: PullEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULL_ENABLE` reader - "]
pub type PullEnableR = crate::BitReader<PullEnable>;
impl PullEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PullEnable {
        match self.bits {
            true => PullEnable::EnablePull,
            false => PullEnable::DisablePull,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable_pull(&self) -> bool {
        *self == PullEnable::EnablePull
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable_pull(&self) -> bool {
        *self == PullEnable::DisablePull
    }
}
#[doc = "Field `PULL_ENABLE` writer - "]
pub type PullEnableW<'a, REG> = crate::BitWriter<'a, REG, PullEnable>;
impl<'a, REG> PullEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable_pull(self) -> &'a mut crate::W<REG> {
        self.variant(PullEnable::EnablePull)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable_pull(self) -> &'a mut crate::W<REG> {
        self.variant(PullEnable::DisablePull)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PullDir {
    #[doc = "1: `1`"]
    PullUp = 1,
    #[doc = "0: `0`"]
    PullDown = 0,
}
impl From<PullDir> for bool {
    #[inline(always)]
    fn from(variant: PullDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULL_DIR` reader - "]
pub type PullDirR = crate::BitReader<PullDir>;
impl PullDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PullDir {
        match self.bits {
            true => PullDir::PullUp,
            false => PullDir::PullDown,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PullDir::PullUp
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PullDir::PullDown
    }
}
#[doc = "Field `PULL_DIR` writer - "]
pub type PullDirW<'a, REG> = crate::BitWriter<'a, REG, PullDir>;
impl<'a, REG> PullDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PullDir::PullUp)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PullDir::PullDown)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEn {
    #[doc = "1: `1`"]
    EnableInput = 1,
    #[doc = "0: `0`"]
    DisableInput = 0,
}
impl From<InputEn> for bool {
    #[inline(always)]
    fn from(variant: InputEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT_EN` reader - "]
pub type InputEnR = crate::BitReader<InputEn>;
impl InputEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputEn {
        match self.bits {
            true => InputEn::EnableInput,
            false => InputEn::DisableInput,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable_input(&self) -> bool {
        *self == InputEn::EnableInput
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable_input(&self) -> bool {
        *self == InputEn::DisableInput
    }
}
#[doc = "Field `INPUT_EN` writer - "]
pub type InputEnW<'a, REG> = crate::BitWriter<'a, REG, InputEn>;
impl<'a, REG> InputEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable_input(self) -> &'a mut crate::W<REG> {
        self.variant(InputEn::EnableInput)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable_input(self) -> &'a mut crate::W<REG> {
        self.variant(InputEn::DisableInput)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ds0(&self) -> Ds0R {
        Ds0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ds1(&self) -> Ds1R {
        Ds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn st0(&self) -> St0R {
        St0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn st1(&self) -> St1R {
        St1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rate_control(&self) -> RateControlR {
        RateControlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_en(&self) -> OutputEnR {
        OutputEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PullEnableR {
        PullEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_dir(&self) -> PullDirR {
        PullDirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_en(&self) -> InputEnR {
        InputEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_RGMII_2_RX_DATA")
            .field("ds0", &self.ds0())
            .field("ds1", &self.ds1())
            .field("st0", &self.st0())
            .field("st1", &self.st1())
            .field("rate_control", &self.rate_control())
            .field("output_en", &self.output_en())
            .field("hold", &self.hold())
            .field("pull_enable", &self.pull_enable())
            .field("pull_dir", &self.pull_dir())
            .field("input_en", &self.input_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ds0(&mut self) -> Ds0W<PadRgmii2RxDataSpec> {
        Ds0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ds1(&mut self) -> Ds1W<PadRgmii2RxDataSpec> {
        Ds1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> St0W<PadRgmii2RxDataSpec> {
        St0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> St1W<PadRgmii2RxDataSpec> {
        St1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rate_control(&mut self) -> RateControlW<PadRgmii2RxDataSpec> {
        RateControlW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn output_en(&mut self) -> OutputEnW<PadRgmii2RxDataSpec> {
        OutputEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HoldW<PadRgmii2RxDataSpec> {
        HoldW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PullEnableW<PadRgmii2RxDataSpec> {
        PullEnableW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pull_dir(&mut self) -> PullDirW<PadRgmii2RxDataSpec> {
        PullDirW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn input_en(&mut self) -> InputEnW<PadRgmii2RxDataSpec> {
        InputEnW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_rgmii_2_rx_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_rgmii_2_rx_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadRgmii2RxDataSpec;
impl crate::RegisterSpec for PadRgmii2RxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_rgmii_2_rx_data::R`](R) reader structure"]
impl crate::Readable for PadRgmii2RxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pad_rgmii_2_rx_data::W`](W) writer structure"]
impl crate::Writable for PadRgmii2RxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_RGMII_2_RX_DATA to value 0x0224"]
impl crate::Resettable for PadRgmii2RxDataSpec {
    const RESET_VALUE: u32 = 0x0224;
}
