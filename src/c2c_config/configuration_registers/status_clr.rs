#[doc = "Register `STATUS_CLR` reader"]
pub struct R(crate::R<STATUS_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS_CLR` writer"]
pub struct W(crate::W<STATUS_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_CLR_SPEC>;
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
impl From<crate::W<STATUS_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr_complete` reader - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WR_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `wr_complete` writer - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
pub type WR_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLR_SPEC, bool, O>;
#[doc = "Field `rd_complete` reader - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RD_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `rd_complete` writer - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
pub type RD_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLR_SPEC, bool, O>;
#[doc = "Field `wr_error` reader - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WR_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `wr_error` writer - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
pub type WR_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLR_SPEC, bool, O>;
#[doc = "Field `rd_error` reader - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RD_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `rd_error` writer - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
pub type RD_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_CLR_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - Acknowledging wr_complete status bit. Clearing wr_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_complete(&mut self) -> WR_COMPLETE_W<0> {
        WR_COMPLETE_W::new(self)
    }
    #[doc = "Bit 1 - Acknowledging rd_complete status bit. Clearing rd_complete status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_complete(&mut self) -> RD_COMPLETE_W<1> {
        RD_COMPLETE_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledging wr_error status bit. Clearing wr_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn wr_error(&mut self) -> WR_ERROR_W<2> {
        WR_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledging rd_error status bit. Clearing rd_error status flag by setting '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rd_error(&mut self) -> RD_ERROR_W<3> {
        RD_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_clr](index.html) module"]
pub struct STATUS_CLR_SPEC;
impl crate::RegisterSpec for STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status_clr::R](R) reader structure"]
impl crate::Readable for STATUS_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status_clr::W](W) writer structure"]
impl crate::Writable for STATUS_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS_CLR to value 0"]
impl crate::Resettable for STATUS_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
