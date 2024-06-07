#[doc = "Register `STATUS_CLR` reader"]
pub type R = crate::R<StatusClrSpec>;
#[doc = "Register `STATUS_CLR` writer"]
pub type W = crate::W<StatusClrSpec>;
#[doc = "Field `wr_complete` reader - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WrCompleteR = crate::BitReader;
#[doc = "Field `wr_complete` writer - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WrCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_complete` reader - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RdCompleteR = crate::BitReader;
#[doc = "Field `rd_complete` writer - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RdCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_error_0` reader - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WrError0R = crate::BitReader;
#[doc = "Field `wr_error_0` writer - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WrError0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_error` reader - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RdErrorR = crate::BitReader;
#[doc = "Field `rd_error` writer - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RdErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
    #[inline(always)]
    pub fn wr_complete(&self) -> WrCompleteR {
        WrCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
    #[inline(always)]
    pub fn rd_complete(&self) -> RdCompleteR {
        RdCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
    #[inline(always)]
    pub fn wr_error_0(&self) -> WrError0R {
        WrError0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
    #[inline(always)]
    pub fn rd_error(&self) -> RdErrorR {
        RdErrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_CLR")
            .field("wr_complete", &self.wr_complete())
            .field("rd_complete", &self.rd_complete())
            .field("wr_error_0", &self.wr_error_0())
            .field("rd_error", &self.rd_error())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_complete(&mut self) -> WrCompleteW<StatusClrSpec> {
        WrCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_complete(&mut self) -> RdCompleteW<StatusClrSpec> {
        RdCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_error_0(&mut self) -> WrError0W<StatusClrSpec> {
        WrError0W::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_error(&mut self) -> RdErrorW<StatusClrSpec> {
        RdErrorW::new(self, 3)
    }
}
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusClrSpec;
impl crate::RegisterSpec for StatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_clr::R`](R) reader structure"]
impl crate::Readable for StatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`status_clr::W`](W) writer structure"]
impl crate::Writable for StatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS_CLR to value 0"]
impl crate::Resettable for StatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
