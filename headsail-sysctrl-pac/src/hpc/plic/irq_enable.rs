#[doc = "Register `irq_enable[%s]` reader"]
pub struct R(crate::R<IRQ_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irq_enable[%s]` writer"]
pub struct W(crate::W<IRQ_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENABLE_SPEC>;
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
impl From<crate::W<IRQ_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer0_int0` reader - "]
pub type TIMER0_INT0_R = crate::BitReader<bool>;
#[doc = "Field `timer0_int0` writer - "]
pub type TIMER0_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer0_int1` reader - "]
pub type TIMER0_INT1_R = crate::BitReader<bool>;
#[doc = "Field `timer0_int1` writer - "]
pub type TIMER0_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer1_int0` reader - "]
pub type TIMER1_INT0_R = crate::BitReader<bool>;
#[doc = "Field `timer1_int0` writer - "]
pub type TIMER1_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer1_int1` reader - "]
pub type TIMER1_INT1_R = crate::BitReader<bool>;
#[doc = "Field `timer1_int1` writer - "]
pub type TIMER1_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer2_int0` reader - "]
pub type TIMER2_INT0_R = crate::BitReader<bool>;
#[doc = "Field `timer2_int0` writer - "]
pub type TIMER2_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer2_int1` reader - "]
pub type TIMER2_INT1_R = crate::BitReader<bool>;
#[doc = "Field `timer2_int1` writer - "]
pub type TIMER2_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer3_int0` reader - "]
pub type TIMER3_INT0_R = crate::BitReader<bool>;
#[doc = "Field `timer3_int0` writer - "]
pub type TIMER3_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `timer3_int1` reader - "]
pub type TIMER3_INT1_R = crate::BitReader<bool>;
#[doc = "Field `timer3_int1` writer - "]
pub type TIMER3_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int0` reader - "]
pub type EXT_INT0_R = crate::BitReader<bool>;
#[doc = "Field `ext_int0` writer - "]
pub type EXT_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int1` reader - "]
pub type EXT_INT1_R = crate::BitReader<bool>;
#[doc = "Field `ext_int1` writer - "]
pub type EXT_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int2` reader - "]
pub type EXT_INT2_R = crate::BitReader<bool>;
#[doc = "Field `ext_int2` writer - "]
pub type EXT_INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int3` reader - "]
pub type EXT_INT3_R = crate::BitReader<bool>;
#[doc = "Field `ext_int3` writer - "]
pub type EXT_INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int4` reader - "]
pub type EXT_INT4_R = crate::BitReader<bool>;
#[doc = "Field `ext_int4` writer - "]
pub type EXT_INT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int5` reader - "]
pub type EXT_INT5_R = crate::BitReader<bool>;
#[doc = "Field `ext_int5` writer - "]
pub type EXT_INT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int6` reader - "]
pub type EXT_INT6_R = crate::BitReader<bool>;
#[doc = "Field `ext_int6` writer - "]
pub type EXT_INT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int7` reader - "]
pub type EXT_INT7_R = crate::BitReader<bool>;
#[doc = "Field `ext_int7` writer - "]
pub type EXT_INT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int8` reader - "]
pub type EXT_INT8_R = crate::BitReader<bool>;
#[doc = "Field `ext_int8` writer - "]
pub type EXT_INT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int9` reader - "]
pub type EXT_INT9_R = crate::BitReader<bool>;
#[doc = "Field `ext_int9` writer - "]
pub type EXT_INT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int10` reader - "]
pub type EXT_INT10_R = crate::BitReader<bool>;
#[doc = "Field `ext_int10` writer - "]
pub type EXT_INT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int11` reader - "]
pub type EXT_INT11_R = crate::BitReader<bool>;
#[doc = "Field `ext_int11` writer - "]
pub type EXT_INT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int12` reader - "]
pub type EXT_INT12_R = crate::BitReader<bool>;
#[doc = "Field `ext_int12` writer - "]
pub type EXT_INT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int13` reader - "]
pub type EXT_INT13_R = crate::BitReader<bool>;
#[doc = "Field `ext_int13` writer - "]
pub type EXT_INT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int14` reader - "]
pub type EXT_INT14_R = crate::BitReader<bool>;
#[doc = "Field `ext_int14` writer - "]
pub type EXT_INT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int15` reader - "]
pub type EXT_INT15_R = crate::BitReader<bool>;
#[doc = "Field `ext_int15` writer - "]
pub type EXT_INT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int16` reader - "]
pub type EXT_INT16_R = crate::BitReader<bool>;
#[doc = "Field `ext_int16` writer - "]
pub type EXT_INT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int17` reader - "]
pub type EXT_INT17_R = crate::BitReader<bool>;
#[doc = "Field `ext_int17` writer - "]
pub type EXT_INT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int18` reader - "]
pub type EXT_INT18_R = crate::BitReader<bool>;
#[doc = "Field `ext_int18` writer - "]
pub type EXT_INT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int19` reader - "]
pub type EXT_INT19_R = crate::BitReader<bool>;
#[doc = "Field `ext_int19` writer - "]
pub type EXT_INT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int20` reader - "]
pub type EXT_INT20_R = crate::BitReader<bool>;
#[doc = "Field `ext_int20` writer - "]
pub type EXT_INT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int21` reader - "]
pub type EXT_INT21_R = crate::BitReader<bool>;
#[doc = "Field `ext_int21` writer - "]
pub type EXT_INT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
#[doc = "Field `ext_int22` reader - "]
pub type EXT_INT22_R = crate::BitReader<bool>;
#[doc = "Field `ext_int22` writer - "]
pub type EXT_INT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer0_int0(&self) -> TIMER0_INT0_R {
        TIMER0_INT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer0_int1(&self) -> TIMER0_INT1_R {
        TIMER0_INT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer1_int0(&self) -> TIMER1_INT0_R {
        TIMER1_INT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_int1(&self) -> TIMER1_INT1_R {
        TIMER1_INT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_int0(&self) -> TIMER2_INT0_R {
        TIMER2_INT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer2_int1(&self) -> TIMER2_INT1_R {
        TIMER2_INT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer3_int0(&self) -> TIMER3_INT0_R {
        TIMER3_INT0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer3_int1(&self) -> TIMER3_INT1_R {
        TIMER3_INT1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ext_int0(&self) -> EXT_INT0_R {
        EXT_INT0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ext_int1(&self) -> EXT_INT1_R {
        EXT_INT1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ext_int2(&self) -> EXT_INT2_R {
        EXT_INT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ext_int3(&self) -> EXT_INT3_R {
        EXT_INT3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ext_int4(&self) -> EXT_INT4_R {
        EXT_INT4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ext_int5(&self) -> EXT_INT5_R {
        EXT_INT5_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ext_int6(&self) -> EXT_INT6_R {
        EXT_INT6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ext_int7(&self) -> EXT_INT7_R {
        EXT_INT7_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ext_int8(&self) -> EXT_INT8_R {
        EXT_INT8_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ext_int9(&self) -> EXT_INT9_R {
        EXT_INT9_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ext_int10(&self) -> EXT_INT10_R {
        EXT_INT10_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ext_int11(&self) -> EXT_INT11_R {
        EXT_INT11_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ext_int12(&self) -> EXT_INT12_R {
        EXT_INT12_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ext_int13(&self) -> EXT_INT13_R {
        EXT_INT13_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ext_int14(&self) -> EXT_INT14_R {
        EXT_INT14_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ext_int15(&self) -> EXT_INT15_R {
        EXT_INT15_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ext_int16(&self) -> EXT_INT16_R {
        EXT_INT16_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ext_int17(&self) -> EXT_INT17_R {
        EXT_INT17_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ext_int18(&self) -> EXT_INT18_R {
        EXT_INT18_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ext_int19(&self) -> EXT_INT19_R {
        EXT_INT19_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ext_int20(&self) -> EXT_INT20_R {
        EXT_INT20_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ext_int21(&self) -> EXT_INT21_R {
        EXT_INT21_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ext_int22(&self) -> EXT_INT22_R {
        EXT_INT22_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_int0(&mut self) -> TIMER0_INT0_W<1> {
        TIMER0_INT0_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_int1(&mut self) -> TIMER0_INT1_W<2> {
        TIMER0_INT1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_int0(&mut self) -> TIMER1_INT0_W<3> {
        TIMER1_INT0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_int1(&mut self) -> TIMER1_INT1_W<4> {
        TIMER1_INT1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_int0(&mut self) -> TIMER2_INT0_W<5> {
        TIMER2_INT0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_int1(&mut self) -> TIMER2_INT1_W<6> {
        TIMER2_INT1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_int0(&mut self) -> TIMER3_INT0_W<7> {
        TIMER3_INT0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn timer3_int1(&mut self) -> TIMER3_INT1_W<8> {
        TIMER3_INT1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int0(&mut self) -> EXT_INT0_W<9> {
        EXT_INT0_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int1(&mut self) -> EXT_INT1_W<10> {
        EXT_INT1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int2(&mut self) -> EXT_INT2_W<11> {
        EXT_INT2_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int3(&mut self) -> EXT_INT3_W<12> {
        EXT_INT3_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int4(&mut self) -> EXT_INT4_W<13> {
        EXT_INT4_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int5(&mut self) -> EXT_INT5_W<14> {
        EXT_INT5_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int6(&mut self) -> EXT_INT6_W<15> {
        EXT_INT6_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int7(&mut self) -> EXT_INT7_W<16> {
        EXT_INT7_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int8(&mut self) -> EXT_INT8_W<17> {
        EXT_INT8_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int9(&mut self) -> EXT_INT9_W<18> {
        EXT_INT9_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int10(&mut self) -> EXT_INT10_W<19> {
        EXT_INT10_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int11(&mut self) -> EXT_INT11_W<20> {
        EXT_INT11_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int12(&mut self) -> EXT_INT12_W<21> {
        EXT_INT12_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int13(&mut self) -> EXT_INT13_W<22> {
        EXT_INT13_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int14(&mut self) -> EXT_INT14_W<23> {
        EXT_INT14_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int15(&mut self) -> EXT_INT15_W<24> {
        EXT_INT15_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int16(&mut self) -> EXT_INT16_W<25> {
        EXT_INT16_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int17(&mut self) -> EXT_INT17_W<26> {
        EXT_INT17_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int18(&mut self) -> EXT_INT18_W<27> {
        EXT_INT18_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int19(&mut self) -> EXT_INT19_W<28> {
        EXT_INT19_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int20(&mut self) -> EXT_INT20_W<29> {
        EXT_INT20_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int21(&mut self) -> EXT_INT21_W<30> {
        EXT_INT21_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ext_int22(&mut self) -> EXT_INT22_W<31> {
        EXT_INT22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ enable bits for Hart context #\\[%s\\]. `idx / 2` is core-#. Even indices are M-mode, odd indices are S-mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enable](index.html) module"]
pub struct IRQ_ENABLE_SPEC;
impl crate::RegisterSpec for IRQ_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enable::R](R) reader structure"]
impl crate::Readable for IRQ_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enable::W](W) writer structure"]
impl crate::Writable for IRQ_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irq_enable[%s]
to value 0"]
impl crate::Resettable for IRQ_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
