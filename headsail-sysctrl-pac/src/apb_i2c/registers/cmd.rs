#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IA` reader - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IA_R = crate::BitReader<bool>;
#[doc = "Field `IA` writer - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ACK` reader - Acknowledge received data"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - Acknowledge received data"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `WR` reader - Write to bus"]
pub type WR_R = crate::BitReader<bool>;
#[doc = "Field `WR` writer - Write to bus"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RD` reader - Read from bus."]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Read from bus."]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STO` reader - Send stop bit"]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `STO` writer - Send stop bit"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STA` reader - Send start bit"]
pub type STA_R = crate::BitReader<bool>;
#[doc = "Field `STA` writer - Send start bit"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
    #[inline(always)]
    pub fn ia(&self) -> IA_R {
        IA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge received data"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write to bus"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read from bus."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send stop bit"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Send start bit"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
    #[inline(always)]
    #[must_use]
    pub fn ia(&mut self) -> IA_W<0> {
        IA_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledge received data"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<3> {
        ACK_W::new(self)
    }
    #[doc = "Bit 4 - Write to bus"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<4> {
        WR_W::new(self)
    }
    #[doc = "Bit 5 - Read from bus."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<5> {
        RD_W::new(self)
    }
    #[doc = "Bit 6 - Send stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<6> {
        STO_W::new(self)
    }
    #[doc = "Bit 7 - Send start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<7> {
        STA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
