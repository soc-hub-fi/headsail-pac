#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `IRQ` reader - Interrupt received. This flag is always set when transmission has finished or bus arpitration was lostm, regardless of whether interrupts are enabled or not. This flag can possibly polled and is cleared by writing 1 to the IA command register."]
pub type IRQ_R = crate::BitReader;
#[doc = "Field `TIP` reader - Transfer in progress"]
pub type TIP_R = crate::BitReader;
#[doc = "Field `AL` reader - Arbitration lost."]
pub type AL_R = crate::BitReader;
#[doc = "Field `BUS` reader - Bus is busy"]
pub type BUS_R = crate::BitReader;
#[doc = "Field `RXA` reader - Acknowledge from sent data"]
pub type RXA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt received. This flag is always set when transmission has finished or bus arpitration was lostm, regardless of whether interrupts are enabled or not. This flag can possibly polled and is cleared by writing 1 to the IA command register."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer in progress"]
    #[inline(always)]
    pub fn tip(&self) -> TIP_R {
        TIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration lost."]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus is busy"]
    #[inline(always)]
    pub fn bus(&self) -> BUS_R {
        BUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge from sent data"]
    #[inline(always)]
    pub fn rxa(&self) -> RXA_R {
        RXA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("irq", &self.irq())
            .field("tip", &self.tip())
            .field("al", &self.al())
            .field("bus", &self.bus())
            .field("rxa", &self.rxa())
            .finish()
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
