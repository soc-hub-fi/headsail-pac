#[doc = "Register `T0_TH_CHANNEL1` reader"]
pub struct R(crate::R<T0_TH_CHANNEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_TH_CHANNEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_TH_CHANNEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_TH_CHANNEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0_TH_CHANNEL1` writer"]
pub struct W(crate::W<T0_TH_CHANNEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0_TH_CHANNEL1_SPEC>;
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
impl From<crate::W<T0_TH_CHANNEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0_TH_CHANNEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH` reader - ADV_TIMER0 channel 1 threshold configuration bitfield"]
pub type TH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH` writer - ADV_TIMER0 channel 1 threshold configuration bitfield"]
pub type TH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_TH_CHANNEL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `MODE` reader - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "5: `101`"]
    TOGGLE_SET_NEXT = 5,
    #[doc = "1: `1`"]
    TOGGLE_CLEAR_NEXT = 1,
    #[doc = "3: `11`"]
    TOGGLE = 3,
    #[doc = "2: `10`"]
    SET_CLEAR_NEXT = 2,
    #[doc = "0: `0`"]
    SET = 0,
    #[doc = "6: `110`"]
    CLEAR_SET_NEXT = 6,
    #[doc = "4: `100`"]
    CLEAR = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            5 => Some(MODE_A::TOGGLE_SET_NEXT),
            1 => Some(MODE_A::TOGGLE_CLEAR_NEXT),
            3 => Some(MODE_A::TOGGLE),
            2 => Some(MODE_A::SET_CLEAR_NEXT),
            0 => Some(MODE_A::SET),
            6 => Some(MODE_A::CLEAR_SET_NEXT),
            4 => Some(MODE_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE_SET_NEXT`"]
    #[inline(always)]
    pub fn is_toggle_set_next(&self) -> bool {
        *self == MODE_A::TOGGLE_SET_NEXT
    }
    #[doc = "Checks if the value of the field is `TOGGLE_CLEAR_NEXT`"]
    #[inline(always)]
    pub fn is_toggle_clear_next(&self) -> bool {
        *self == MODE_A::TOGGLE_CLEAR_NEXT
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == MODE_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `SET_CLEAR_NEXT`"]
    #[inline(always)]
    pub fn is_set_clear_next(&self) -> bool {
        *self == MODE_A::SET_CLEAR_NEXT
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR_SET_NEXT`"]
    #[inline(always)]
    pub fn is_clear_set_next(&self) -> bool {
        *self == MODE_A::CLEAR_SET_NEXT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MODE_A::CLEAR
    }
}
#[doc = "Field `MODE` writer - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T0_TH_CHANNEL1_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`101`"]
    #[inline(always)]
    pub fn toggle_set_next(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE_SET_NEXT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn toggle_clear_next(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE_CLEAR_NEXT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(MODE_A::TOGGLE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn set_clear_next(self) -> &'a mut W {
        self.variant(MODE_A::SET_CLEAR_NEXT)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(MODE_A::SET)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn clear_set_next(self) -> &'a mut W {
        self.variant(MODE_A::CLEAR_SET_NEXT)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MODE_A::CLEAR)
    }
}
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 1 threshold configuration bitfield"]
    #[inline(always)]
    pub fn th(&self) -> TH_R {
        TH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER0 channel 1 threshold configuration bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn th(&mut self) -> TH_W<0> {
        TH_W::new(self)
    }
    #[doc = "Bits 16:18 - ADV_TIMER0 channel 1 threshold match action on channel output signal configuration bitfield: - 3'h0: set. - 3'h1: toggle then next threshold match action is clear. - 3'h2: set then next threshold match action is clear. - 3'h3: toggle. - 3'h4: clear. - 3'h5: toggle then next threshold match action is set. - 3'h6: clear then next threshold match action is set."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER0 channel 1 threshold configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0_th_channel1](index.html) module"]
pub struct T0_TH_CHANNEL1_SPEC;
impl crate::RegisterSpec for T0_TH_CHANNEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0_th_channel1::R](R) reader structure"]
impl crate::Readable for T0_TH_CHANNEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0_th_channel1::W](W) writer structure"]
impl crate::Writable for T0_TH_CHANNEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0_TH_CHANNEL1 to value 0"]
impl crate::Resettable for T0_TH_CHANNEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
