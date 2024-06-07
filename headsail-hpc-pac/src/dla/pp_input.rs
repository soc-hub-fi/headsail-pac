#[doc = "Register `pp_input` reader"]
pub type R = crate::R<PpInputSpec>;
#[doc = "Register `pp_input` writer"]
pub type W = crate::W<PpInputSpec>;
#[doc = "Field `width` reader - "]
pub type WidthR = crate::FieldReader<u16>;
#[doc = "Field `width` writer - "]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `height` reader - "]
pub type HeightR = crate::FieldReader<u16>;
#[doc = "Field `height` writer - "]
pub type HeightW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn height(&self) -> HeightR {
        HeightR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pp_input")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<PpInputSpec> {
        WidthW::new(self, 0)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HeightW<PpInputSpec> {
        HeightW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpInputSpec;
impl crate::RegisterSpec for PpInputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp_input::R`](R) reader structure"]
impl crate::Readable for PpInputSpec {}
#[doc = "`write(|w| ..)` method takes [`pp_input::W`](W) writer structure"]
impl crate::Writable for PpInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pp_input to value 0"]
impl crate::Resettable for PpInputSpec {
    const RESET_VALUE: u32 = 0;
}
