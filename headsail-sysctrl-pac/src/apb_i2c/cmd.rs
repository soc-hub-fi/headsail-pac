#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `IA` reader - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IaR = crate::BitReader;
#[doc = "Field `IA` writer - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
pub type IaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge received data"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge received data"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Write to bus"]
pub type WrR = crate::BitReader;
#[doc = "Field `WR` writer - Write to bus"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read from bus."]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Read from bus."]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STO` reader - Send stop bit"]
pub type StoR = crate::BitReader;
#[doc = "Field `STO` writer - Send stop bit"]
pub type StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA` reader - Send start bit"]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - Send start bit"]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Acknowldge. Set to one to acknowledge interrupt. Cleared when transmission is done or arbitration is lost."]
    #[inline(always)]
    pub fn ia(&self) -> IaR {
        IaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge received data"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write to bus"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read from bus."]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Send stop bit"]
    #[inline(always)]
    pub fn sto(&self) -> StoR {
        StoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Send start bit"]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn ia(&mut self) -> IaW<CmdSpec> {
        IaW::new(self, 0)
    }
    #[doc = "Bit 3 - Acknowledge received data"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<CmdSpec> {
        AckW::new(self, 3)
    }
    #[doc = "Bit 4 - Write to bus"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<CmdSpec> {
        WrW::new(self, 4)
    }
    #[doc = "Bit 5 - Read from bus."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<CmdSpec> {
        RdW::new(self, 5)
    }
    #[doc = "Bit 6 - Send stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> StoW<CmdSpec> {
        StoW::new(self, 6)
    }
    #[doc = "Bit 7 - Send start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<CmdSpec> {
        StaW::new(self, 7)
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
