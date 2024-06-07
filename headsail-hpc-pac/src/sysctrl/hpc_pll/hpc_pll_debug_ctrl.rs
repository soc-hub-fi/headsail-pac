#[doc = "Register `HPC_PLL_DEBUG_CTRL` reader"]
pub type R = crate::R<HpcPllDebugCtrlSpec>;
#[doc = "Register `HPC_PLL_DEBUG_CTRL` writer"]
pub type W = crate::W<HpcPllDebugCtrlSpec>;
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
        f.debug_struct("HPC_PLL_DEBUG_CTRL")
            .field("debug_ctrl", &self.debug_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn debug_ctrl(&mut self) -> DebugCtrlW<HpcPllDebugCtrlSpec> {
        DebugCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpc_pll_debug_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpc_pll_debug_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpcPllDebugCtrlSpec;
impl crate::RegisterSpec for HpcPllDebugCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpc_pll_debug_ctrl::R`](R) reader structure"]
impl crate::Readable for HpcPllDebugCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hpc_pll_debug_ctrl::W`](W) writer structure"]
impl crate::Writable for HpcPllDebugCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPC_PLL_DEBUG_CTRL to value 0"]
impl crate::Resettable for HpcPllDebugCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
