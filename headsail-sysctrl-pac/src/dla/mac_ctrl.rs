#[doc = "Register `mac_ctrl` reader"]
pub type R = crate::R<MacCtrlSpec>;
#[doc = "Register `mac_ctrl` writer"]
pub type W = crate::W<MacCtrlSpec>;
#[doc = "Field `simdSelect` reader - "]
pub type SimdSelectR = crate::FieldReader;
#[doc = "Field `simdSelect` writer - "]
pub type SimdSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clip` reader - "]
pub type ClipR = crate::FieldReader;
#[doc = "Field `clip` writer - "]
pub type ClipW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn simd_select(&self) -> SimdSelectR {
        SimdSelectR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn clip(&self) -> ClipR {
        ClipR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mac_ctrl")
            .field("simd_select", &self.simd_select())
            .field("clip", &self.clip())
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn simd_select(&mut self) -> SimdSelectW<MacCtrlSpec> {
        SimdSelectW::new(self, 1)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> ClipW<MacCtrlSpec> {
        ClipW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacCtrlSpec;
impl crate::RegisterSpec for MacCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_ctrl::R`](R) reader structure"]
impl crate::Readable for MacCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_ctrl::W`](W) writer structure"]
impl crate::Writable for MacCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mac_ctrl to value 0"]
impl crate::Resettable for MacCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
