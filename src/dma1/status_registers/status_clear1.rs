#[doc = "Register `STATUS_CLEAR1` reader"]
pub struct R(crate::R<STATUS_CLEAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_CLEAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_CLEAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_CLEAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS_CLEAR1` writer"]
pub struct W(crate::W<STATUS_CLEAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_CLEAR1_SPEC>;
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
impl From<crate::W<STATUS_CLEAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_CLEAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr_transfer_completed` reader - "]
pub type WR_TRANSFER_COMPLETED_R = crate::BitReader<bool>;
#[doc = "Field `wr_transfer_completed` writer - "]
pub type WR_TRANSFER_COMPLETED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
#[doc = "Field `rd_transfer_completed` reader - "]
pub type RD_TRANSFER_COMPLETED_R = crate::BitReader<bool>;
#[doc = "Field `rd_transfer_completed` writer - "]
pub type RD_TRANSFER_COMPLETED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
#[doc = "Field `rb_error` reader - "]
pub type RB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `rb_error` writer - "]
pub type RB_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
#[doc = "Field `wb_error` reader - "]
pub type WB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `wb_error` writer - "]
pub type WB_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
#[doc = "Field `fifo_overflow` reader - "]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `fifo_overflow` writer - "]
pub type FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
#[doc = "Field `fifo_underflow` reader - "]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `fifo_underflow` writer - "]
pub type FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLEAR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wr_transfer_completed(&self) -> WR_TRANSFER_COMPLETED_R {
        WR_TRANSFER_COMPLETED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rd_transfer_completed(&self) -> RD_TRANSFER_COMPLETED_R {
        RD_TRANSFER_COMPLETED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rb_error(&self) -> RB_ERROR_R {
        RB_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wb_error(&self) -> WB_ERROR_R {
        WB_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wr_transfer_completed(&mut self) -> WR_TRANSFER_COMPLETED_W<0> {
        WR_TRANSFER_COMPLETED_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rd_transfer_completed(&mut self) -> RD_TRANSFER_COMPLETED_W<1> {
        RD_TRANSFER_COMPLETED_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rb_error(&mut self) -> RB_ERROR_W<2> {
        RB_ERROR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wb_error(&mut self) -> WB_ERROR_W<3> {
        WB_ERROR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow(&mut self) -> FIFO_OVERFLOW_W<4> {
        FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underflow(&mut self) -> FIFO_UNDERFLOW_W<5> {
        FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_clear1](index.html) module"]
pub struct STATUS_CLEAR1_SPEC;
impl crate::RegisterSpec for STATUS_CLEAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_clear1::R](R) reader structure"]
impl crate::Readable for STATUS_CLEAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status_clear1::W](W) writer structure"]
impl crate::Writable for STATUS_CLEAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS_CLEAR1 to value 0"]
impl crate::Resettable for STATUS_CLEAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
