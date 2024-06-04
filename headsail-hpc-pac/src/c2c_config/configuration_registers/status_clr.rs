#[doc = "Register `STATUS_CLR` reader"]
pub type R = crate::R<STATUS_CLR_SPEC>;
#[doc = "Register `STATUS_CLR` writer"]
pub type W = crate::W<STATUS_CLR_SPEC>;
#[doc = "Field `wr_complete` reader - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WR_COMPLETE_R = crate::BitReader;
#[doc = "Field `wr_complete` writer - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WR_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_complete` reader - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RD_COMPLETE_R = crate::BitReader;
#[doc = "Field `rd_complete` writer - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RD_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_error` reader - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WR_ERROR_R = crate::BitReader;
#[doc = "Field `wr_error` writer - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WR_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_error` reader - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RD_ERROR_R = crate::BitReader;
#[doc = "Field `rd_error` writer - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RD_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
    #[inline(always)]
    pub fn wr_complete(&self) -> WR_COMPLETE_R {
        WR_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
    #[inline(always)]
    pub fn rd_complete(&self) -> RD_COMPLETE_R {
        RD_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
    #[inline(always)]
    pub fn wr_error(&self) -> WR_ERROR_R {
        WR_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
    #[inline(always)]
    pub fn rd_error(&self) -> RD_ERROR_R {
        RD_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_CLR")
            .field("wr_complete", &self.wr_complete())
            .field("rd_complete", &self.rd_complete())
            .field("wr_error", &self.wr_error())
            .field("rd_error", &self.rd_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_complete(&mut self) -> WR_COMPLETE_W<STATUS_CLR_SPEC> {
        WR_COMPLETE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_complete(&mut self) -> RD_COMPLETE_W<STATUS_CLR_SPEC> {
        RD_COMPLETE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_error(&mut self) -> WR_ERROR_W<STATUS_CLR_SPEC> {
        WR_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_error(&mut self) -> RD_ERROR_W<STATUS_CLR_SPEC> {
        RD_ERROR_W::new(self, 3)
    }
}
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_CLR_SPEC;
impl crate::RegisterSpec for STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_clr::R`](R) reader structure"]
impl crate::Readable for STATUS_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status_clr::W`](W) writer structure"]
impl crate::Writable for STATUS_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS_CLR to value 0"]
impl crate::Resettable for STATUS_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
