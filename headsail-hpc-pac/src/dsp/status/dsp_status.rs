#[doc = "Register `dsp_status` reader"]
pub type R = crate::R<DSP_STATUS_SPEC>;
#[doc = "Field `single_stepping_bp` reader - "]
pub type SINGLE_STEPPING_BP_R = crate::BitReader;
#[doc = "Field `bp1` reader - "]
pub type BP1_R = crate::BitReader;
#[doc = "Field `manual_bp` reader - "]
pub type MANUAL_BP_R = crate::BitReader;
#[doc = "Field `bp2` reader - "]
pub type BP2_R = crate::BitReader;
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
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dsp_status")
            .field("single_stepping_bp", &self.single_stepping_bp())
            .field("bp1", &self.bp1())
            .field("manual_bp", &self.manual_bp())
            .field("bp2", &self.bp2())
            .finish()
    }
}
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSP_STATUS_SPEC;
impl crate::RegisterSpec for DSP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_status::R`](R) reader structure"]
impl crate::Readable for DSP_STATUS_SPEC {}
#[doc = "`reset()` method sets dsp_status to value 0"]
impl crate::Resettable for DSP_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
