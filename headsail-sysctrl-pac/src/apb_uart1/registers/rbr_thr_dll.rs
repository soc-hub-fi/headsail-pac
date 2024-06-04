#[doc = "Register `RBR_THR_DLL` reader"]
pub type R = crate::R<RBR_THR_DLL_SPEC>;
#[doc = "Register `RBR_THR_DLL` writer"]
pub type W = crate::W<RBR_THR_DLL_SPEC>;
#[doc = "Field `RBR_THR_DLL` reader - "]
pub type RBR_THR_DLL_R = crate::FieldReader;
#[doc = "Field `RBR_THR_DLL` writer - "]
pub type RBR_THR_DLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr_thr_dll(&self) -> RBR_THR_DLL_R {
        RBR_THR_DLL_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBR_THR_DLL")
            .field("rbr_thr_dll", &self.rbr_thr_dll())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rbr_thr_dll(&mut self) -> RBR_THR_DLL_W<RBR_THR_DLL_SPEC> {
        RBR_THR_DLL_W::new(self, 0)
    }
}
#[doc = "RBR_THR_DLL is a multi-purpose register address. you can access 3 different registers using the same address. IF LCR\\[7\\]
is 0 RBR and THR are accessable. OW DLL is accessable. RBR read only. Reads from rx fifo THR write only. Writes into a tx fifo DLL writes/reads into/from the 8 LSBs in the divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr_thr_dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr_thr_dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBR_THR_DLL_SPEC;
impl crate::RegisterSpec for RBR_THR_DLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rbr_thr_dll::R`](R) reader structure"]
impl crate::Readable for RBR_THR_DLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rbr_thr_dll::W`](W) writer structure"]
impl crate::Writable for RBR_THR_DLL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RBR_THR_DLL to value 0"]
impl crate::Resettable for RBR_THR_DLL_SPEC {
    const RESET_VALUE: u8 = 0;
}
