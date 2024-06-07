#[doc = "Register `T0_TH_CHANNEL1` reader"]
pub type R = crate::R<T0ThChannel1Spec>;
#[doc = "Register `T0_TH_CHANNEL1` writer"]
pub type W = crate::W<T0ThChannel1Spec>;
#[doc = "Field `TH` reader - ADV_TIMER0 channel 1 threshold configuration bitfield"]
pub type ThR = crate::FieldReader<u16>;
#[doc = "Field `TH` writer - ADV_TIMER0 channel 1 threshold configuration bitfield"]
pub type ThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "5: `101`"]
    ToggleSetNext = 5,
    #[doc = "1: `1`"]
    ToggleClearNext = 1,
    #[doc = "3: `11`"]
    Toggle = 3,
    #[doc = "2: `10`"]
    SetClearNext = 2,
    #[doc = "0: `0`"]
    Set = 0,
    #[doc = "6: `110`"]
    ClearSetNext = 6,
    #[doc = "4: `100`"]
    Clear = 4,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            5 => Some(Mode::ToggleSetNext),
            1 => Some(Mode::ToggleClearNext),
            3 => Some(Mode::Toggle),
            2 => Some(Mode::SetClearNext),
            0 => Some(Mode::Set),
            6 => Some(Mode::ClearSetNext),
            4 => Some(Mode::Clear),
            _ => None,
        }
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_toggle_set_next(&self) -> bool {
        *self == Mode::ToggleSetNext
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_toggle_clear_next(&self) -> bool {
        *self == Mode::ToggleClearNext
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Mode::Toggle
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_set_clear_next(&self) -> bool {
        *self == Mode::SetClearNext
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mode::Set
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_clear_set_next(&self) -> bool {
        *self == Mode::ClearSetNext
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Mode::Clear
    }
}
#[doc = "Field `MODE` writer - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`101`"]
    #[inline(always)]
    pub fn toggle_set_next(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ToggleSetNext)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn toggle_clear_next(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ToggleClearNext)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Toggle)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn set_clear_next(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SetClearNext)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Set)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clear_set_next(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ClearSetNext)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Clear)
    }
}
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 1 threshold configuration bitfield"]
    #[inline(always)]
    pub fn th(&self) -> ThR {
        ThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_TH_CHANNEL1")
            .field("th", &self.th())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 1 threshold configuration bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn th(&mut self) -> ThW<T0ThChannel1Spec> {
        ThW::new(self, 0)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<T0ThChannel1Spec> {
        ModeW::new(self, 16)
    }
}
#[doc = "ADV_TIMER0 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0ThChannel1Spec;
impl crate::RegisterSpec for T0ThChannel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_th_channel1::R`](R) reader structure"]
impl crate::Readable for T0ThChannel1Spec {}
#[doc = "`write(|w| ..)` method takes [`t0_th_channel1::W`](W) writer structure"]
impl crate::Writable for T0ThChannel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0_TH_CHANNEL1 to value 0"]
impl crate::Resettable for T0ThChannel1Spec {
    const RESET_VALUE: u32 = 0;
}
