#[doc = "Register `STATUS3` reader"]
pub struct R(crate::R<STATUS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `wr_transfer_completed` reader - "]
pub type WR_TRANSFER_COMPLETED_R = crate::BitReader<bool>;
#[doc = "Field `rd_transfer_completed` reader - "]
pub type RD_TRANSFER_COMPLETED_R = crate::BitReader<bool>;
#[doc = "Field `rb_error` reader - "]
pub type RB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `wb_error` reader - "]
pub type WB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `fifo_overflow` reader - "]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `fifo_underflow` reader - "]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `status_reg_group_0_write` reader - "]
pub type STATUS_REG_GROUP_0_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `status_reg_group_1_write` reader - "]
pub type STATUS_REG_GROUP_1_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `status_reg_group_0_read` reader - "]
pub type STATUS_REG_GROUP_0_READ_R = crate::BitReader<bool>;
#[doc = "Field `status_reg_group_1_read` reader - "]
pub type STATUS_REG_GROUP_1_READ_R = crate::BitReader<bool>;
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn status_reg_group_0_write(&self) -> STATUS_REG_GROUP_0_WRITE_R {
        STATUS_REG_GROUP_0_WRITE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn status_reg_group_1_write(&self) -> STATUS_REG_GROUP_1_WRITE_R {
        STATUS_REG_GROUP_1_WRITE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn status_reg_group_0_read(&self) -> STATUS_REG_GROUP_0_READ_R {
        STATUS_REG_GROUP_0_READ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn status_reg_group_1_read(&self) -> STATUS_REG_GROUP_1_READ_R {
        STATUS_REG_GROUP_1_READ_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status3](index.html) module"]
pub struct STATUS3_SPEC;
impl crate::RegisterSpec for STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status3::R](R) reader structure"]
impl crate::Readable for STATUS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS3 to value 0"]
impl crate::Resettable for STATUS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
