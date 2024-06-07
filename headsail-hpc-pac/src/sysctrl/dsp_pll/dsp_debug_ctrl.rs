#[doc = "Register `DSP_DEBUG_CTRL` reader"]
pub type R = crate::R<DspDebugCtrlSpec>;
#[doc = "Register `DSP_DEBUG_CTRL` writer"]
pub type W = crate::W<DspDebugCtrlSpec>;
#[doc = "Field `Debug_Ctrl` reader - "]
pub type DebugCtrlR = crate::FieldReader;
#[doc = "Field `Debug_Ctrl` writer - "]
pub type DebugCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn debug_ctrl(&self) -> DebugCtrlR {
        DebugCtrlR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSP_DEBUG_CTRL")
            .field("debug_ctrl", &self.debug_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ctrl(&mut self) -> DebugCtrlW<DspDebugCtrlSpec> {
        DebugCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_debug_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_debug_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspDebugCtrlSpec;
impl crate::RegisterSpec for DspDebugCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_debug_ctrl::R`](R) reader structure"]
impl crate::Readable for DspDebugCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_debug_ctrl::W`](W) writer structure"]
impl crate::Writable for DspDebugCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_DEBUG_CTRL to value 0"]
impl crate::Resettable for DspDebugCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
