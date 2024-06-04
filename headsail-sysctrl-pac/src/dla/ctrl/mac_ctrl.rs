#[doc = "Register `mac_ctrl` reader"]
pub type R = crate::R<MAC_CTRL_SPEC>;
#[doc = "Register `mac_ctrl` writer"]
pub type W = crate::W<MAC_CTRL_SPEC>;
#[doc = "Field `simdSelect` reader - "]
pub type SIMD_SELECT_R = crate::FieldReader;
#[doc = "Field `simdSelect` writer - "]
pub type SIMD_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clip` reader - "]
pub type CLIP_R = crate::FieldReader;
#[doc = "Field `clip` writer - "]
pub type CLIP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn simd_select(&self) -> SIMD_SELECT_R {
        SIMD_SELECT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn clip(&self) -> CLIP_R {
        CLIP_R::new(((self.bits >> 8) & 0x1f) as u8)
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
    pub fn simd_select(&mut self) -> SIMD_SELECT_W<MAC_CTRL_SPEC> {
        SIMD_SELECT_W::new(self, 1)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn clip(&mut self) -> CLIP_W<MAC_CTRL_SPEC> {
        CLIP_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_CTRL_SPEC;
impl crate::RegisterSpec for MAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_ctrl::R`](R) reader structure"]
impl crate::Readable for MAC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_ctrl::W`](W) writer structure"]
impl crate::Writable for MAC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mac_ctrl to value 0"]
impl crate::Resettable for MAC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
