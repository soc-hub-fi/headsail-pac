#[doc = "Register `dsp_status` reader"]
pub struct R(crate::R<DSP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `single_stepping_bp` reader - "]
pub type SINGLE_STEPPING_BP_R = crate::BitReader<bool>;
#[doc = "Field `bp1` reader - "]
pub type BP1_R = crate::BitReader<bool>;
#[doc = "Field `manual_bp` reader - "]
pub type MANUAL_BP_R = crate::BitReader<bool>;
#[doc = "Field `bp2` reader - "]
pub type BP2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn single_stepping_bp(&self) -> SINGLE_STEPPING_BP_R {
        SINGLE_STEPPING_BP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bp1(&self) -> BP1_R {
        BP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn manual_bp(&self) -> MANUAL_BP_R {
        MANUAL_BP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bp2(&self) -> BP2_R {
        BP2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_status](index.html) module"]
pub struct DSP_STATUS_SPEC;
impl crate::RegisterSpec for DSP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_status::R](R) reader structure"]
impl crate::Readable for DSP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dsp_status to value 0"]
impl crate::Resettable for DSP_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
