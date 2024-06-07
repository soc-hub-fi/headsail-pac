#[doc = "Register `dsp_status` reader"]
pub type R = crate::R<DspStatusSpec>;
#[doc = "Field `single_stepping_bp` reader - "]
pub type SingleSteppingBpR = crate::BitReader;
#[doc = "Field `bp1` reader - "]
pub type Bp1R = crate::BitReader;
#[doc = "Field `manual_bp` reader - "]
pub type ManualBpR = crate::BitReader;
#[doc = "Field `bp2` reader - "]
pub type Bp2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn single_stepping_bp(&self) -> SingleSteppingBpR {
        SingleSteppingBpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bp1(&self) -> Bp1R {
        Bp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn manual_bp(&self) -> ManualBpR {
        ManualBpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bp2(&self) -> Bp2R {
        Bp2R::new(((self.bits >> 3) & 1) != 0)
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
pub struct DspStatusSpec;
impl crate::RegisterSpec for DspStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_status::R`](R) reader structure"]
impl crate::Readable for DspStatusSpec {}
#[doc = "`reset()` method sets dsp_status to value 0"]
impl crate::Resettable for DspStatusSpec {
    const RESET_VALUE: u32 = 0;
}
