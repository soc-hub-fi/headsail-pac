#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `wr_complete` reader - Status flag for write transfer completion"]
pub type WR_COMPLETE_R = crate::BitReader;
#[doc = "Field `rd_complete` reader - Status flag for read transfer completion"]
pub type RD_COMPLETE_R = crate::BitReader;
#[doc = "Field `wr_error` reader - Status flag for write transfer error"]
pub type WR_ERROR_R = crate::BitReader;
#[doc = "Field `rd_error` reader - Status flag for read transfer error"]
pub type RD_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status flag for write transfer completion"]
    #[inline(always)]
    pub fn wr_complete(&self) -> WR_COMPLETE_R {
        WR_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status flag for read transfer completion"]
    #[inline(always)]
    pub fn rd_complete(&self) -> RD_COMPLETE_R {
        RD_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status flag for write transfer error"]
    #[inline(always)]
    pub fn wr_error(&self) -> WR_ERROR_R {
        WR_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status flag for read transfer error"]
    #[inline(always)]
    pub fn rd_error(&self) -> RD_ERROR_R {
        RD_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("wr_complete", &self.wr_complete())
            .field("rd_complete", &self.rd_complete())
            .field("wr_error", &self.wr_error())
            .field("rd_error", &self.rd_error())
            .finish()
    }
}
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
