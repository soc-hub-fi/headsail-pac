#[doc = "Register `pad_mux2` reader"]
pub type R = crate::R<PadMux2Spec>;
#[doc = "Register `pad_mux2` writer"]
pub type W = crate::W<PadMux2Spec>;
#[doc = "Field `mux_select` reader - "]
pub type MuxSelectR = crate::FieldReader;
#[doc = "Field `mux_select` writer - "]
pub type MuxSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mux_select(&self) -> MuxSelectR {
        MuxSelectR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pad_mux2")
            .field("mux_select", &self.mux_select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn mux_select(&mut self) -> MuxSelectW<PadMux2Spec> {
        MuxSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadMux2Spec;
impl crate::RegisterSpec for PadMux2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_mux2::R`](R) reader structure"]
impl crate::Readable for PadMux2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_mux2::W`](W) writer structure"]
impl crate::Writable for PadMux2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pad_mux2 to value 0"]
impl crate::Resettable for PadMux2Spec {
    const RESET_VALUE: u32 = 0;
}
