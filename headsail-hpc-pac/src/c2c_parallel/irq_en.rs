#[doc = "Register `IRQ_EN` reader"]
pub type R = crate::R<IRQ_EN_SPEC>;
#[doc = "Register `IRQ_EN` writer"]
pub type W = crate::W<IRQ_EN_SPEC>;
#[doc = "Field `wr_complete` reader - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_COMPLETE_R = crate::BitReader;
#[doc = "Field `wr_complete` writer - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_complete` reader - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_COMPLETE_R = crate::BitReader;
#[doc = "Field `rd_complete` writer - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_error` reader - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_ERROR_R = crate::BitReader;
#[doc = "Field `wr_error` writer - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
pub type WR_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_error` reader - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_ERROR_R = crate::BitReader;
#[doc = "Field `rd_error` writer - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
pub type RD_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn wr_complete(&self) -> WR_COMPLETE_R {
        WR_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn rd_complete(&self) -> RD_COMPLETE_R {
        RD_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn wr_error(&self) -> WR_ERROR_R {
        WR_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    pub fn rd_error(&self) -> RD_ERROR_R {
        RD_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_EN")
            .field("wr_complete", &self.wr_complete())
            .field("rd_complete", &self.rd_complete())
            .field("wr_error", &self.wr_error())
            .field("rd_error", &self.rd_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configuring wr_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_complete(&mut self) -> WR_COMPLETE_W<IRQ_EN_SPEC> {
        WR_COMPLETE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuring rd_complete interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_complete(&mut self) -> RD_COMPLETE_W<IRQ_EN_SPEC> {
        RD_COMPLETE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configuring wr_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_error(&mut self) -> WR_ERROR_W<IRQ_EN_SPEC> {
        WR_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configuring rd_error interrupt is routed to the IRQ pin by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_error(&mut self) -> RD_ERROR_W<IRQ_EN_SPEC> {
        RD_ERROR_W::new(self, 3)
    }
}
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_EN_SPEC;
impl crate::RegisterSpec for IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_en::R`](R) reader structure"]
impl crate::Readable for IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_en::W`](W) writer structure"]
impl crate::Writable for IRQ_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IRQ_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
