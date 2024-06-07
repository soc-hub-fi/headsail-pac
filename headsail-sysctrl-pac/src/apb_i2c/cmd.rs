#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `IA` reader - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IA_R = crate::BitReader;
#[doc = "Field `IA` writer - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge received data"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge received data"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Write to bus"]
pub type WR_R = crate::BitReader;
#[doc = "Field `WR` writer - Write to bus"]
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read from bus."]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - Read from bus."]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STO` reader - Send stop bit"]
pub type STO_R = crate::BitReader;
#[doc = "Field `STO` writer - Send stop bit"]
pub type STO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA` reader - Send start bit"]
pub type STA_R = crate::BitReader;
#[doc = "Field `STA` writer - Send start bit"]
pub type STA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("ia", &self.ia())
            .field("ack", &self.ack())
            .field("wr", &self.wr())
            .field("rd", &self.rd())
            .field("sto", &self.sto())
            .field("sta", &self.sta())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
    #[inline(always)]
    #[must_use]
    pub fn ia(&mut self) -> IA_W<CMD_SPEC> {
        IA_W::new(self, 0)
    }
    #[doc = "Bit 3 - Acknowledge received data"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CMD_SPEC> {
        ACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write to bus"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<CMD_SPEC> {
        WR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Read from bus."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<CMD_SPEC> {
        RD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Send stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<CMD_SPEC> {
        STO_W::new(self, 6)
    }
    #[doc = "Bit 7 - Send start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<CMD_SPEC> {
        STA_W::new(self, 7)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
