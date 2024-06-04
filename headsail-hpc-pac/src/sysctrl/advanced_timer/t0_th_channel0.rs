#[doc = "Register `T0_TH_CHANNEL0` reader"]
pub type R = crate::R<T0_TH_CHANNEL0_SPEC>;
#[doc = "Register `T0_TH_CHANNEL0` writer"]
pub type W = crate::W<T0_TH_CHANNEL0_SPEC>;
#[doc = "Field `TH` reader - ADV_TIMER0 channel 0 threshold configuration bitfield"]
pub type TH_R = crate::FieldReader<u16>;
#[doc = "Field `TH` writer - ADV_TIMER0 channel 0 threshold configuration bitfield"]
pub type TH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "ADV_TIMER0 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "1: `1`"]
    TOGGLE_CLEAR_NEXT = 1,
    #[doc = "2: `10`"]
    SET_CLEAR_NEXT = 2,
    #[doc = "3: `11`"]
    TOGGLE = 3,
    #[doc = "4: `100`"]
    CLEAR = 4,
    #[doc = "5: `101`"]
    TOGGLE_SET_NEXT = 5,
    #[doc = "6: `110`"]
    CLEAR_SET_NEXT = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE_A {}
#[doc = "Field `MODE` reader - ADV_TIMER0 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type MODE_R = crate::FieldReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::SET),
            1 => Some(MODE_A::TOGGLE_CLEAR_NEXT),
            2 => Some(MODE_A::SET_CLEAR_NEXT),
            3 => Some(MODE_A::TOGGLE),
            4 => Some(MODE_A::CLEAR),
            5 => Some(MODE_A::TOGGLE_SET_NEXT),
            6 => Some(MODE_A::CLEAR_SET_NEXT),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == MODE_A::SET
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_toggle_clear_next(&self) -> bool {
        *self == MODE_A::TOGGLE_CLEAR_NEXT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_set_clear_next(&self) -> bool {
        *self == MODE_A::SET_CLEAR_NEXT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == MODE_A::TOGGLE
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MODE_A::CLEAR
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_toggle_set_next(&self) -> bool {
        *self == MODE_A::TOGGLE_SET_NEXT
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_clear_set_next(&self) -> bool {
        *self == MODE_A::CLEAR_SET_NEXT
    }
}
#[doc = "Field `MODE` writer - ADV_TIMER0 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SET)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn toggle_clear_next(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::TOGGLE_CLEAR_NEXT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn set_clear_next(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SET_CLEAR_NEXT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::TOGGLE)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CLEAR)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn toggle_set_next(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::TOGGLE_SET_NEXT)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clear_set_next(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CLEAR_SET_NEXT)
    }
}
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 0 threshold configuration bitfield"]
    #[inline(always)]
    pub fn th(&self) -> TH_R {
        TH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_TH_CHANNEL0")
            .field("th", &self.th())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 0 threshold configuration bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn th(&mut self) -> TH_W<T0_TH_CHANNEL0_SPEC> {
        TH_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 0 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<T0_TH_CHANNEL0_SPEC> {
        MODE_W::new(self, 16)
    }
}
#[doc = "ADV_TIMER0 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0_TH_CHANNEL0_SPEC;
impl crate::RegisterSpec for T0_TH_CHANNEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_th_channel0::R`](R) reader structure"]
impl crate::Readable for T0_TH_CHANNEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0_th_channel0::W`](W) writer structure"]
impl crate::Writable for T0_TH_CHANNEL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0_TH_CHANNEL0 to value 0"]
impl crate::Resettable for T0_TH_CHANNEL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
