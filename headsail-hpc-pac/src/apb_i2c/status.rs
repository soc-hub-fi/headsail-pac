#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `IRQ` reader - Interrupt received. This flag is always set when transmission has finished or bus arpitration was lostm, regardless of whether interrupts are enabled or not. This flag can possibly polled and is cleared by writing 1 to the IA command register."]
pub type IrqR = crate::BitReader;
#[doc = "Field `TIP` reader - Transfer in progress"]
pub type TipR = crate::BitReader;
#[doc = "Field `AL` reader - Arbitration lost."]
pub type AlR = crate::BitReader;
#[doc = "Field `BUS` reader - Bus is busy"]
pub type BusR = crate::BitReader;
#[doc = "Field `RXA` reader - Acknowledge from sent data"]
pub type RxaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt received. This flag is always set when transmission has finished or bus arpitration was lostm, regardless of whether interrupts are enabled or not. This flag can possibly polled and is cleared by writing 1 to the IA command register."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer in progress"]
    #[inline(always)]
    pub fn tip(&self) -> TipR {
        TipR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration lost."]
    #[inline(always)]
    pub fn al(&self) -> AlR {
        AlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus is busy"]
    #[inline(always)]
    pub fn bus(&self) -> BusR {
        BusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledge from sent data"]
    #[inline(always)]
    pub fn rxa(&self) -> RxaR {
        RxaR::new(((self.bits >> 7) & 1) != 0)
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
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
