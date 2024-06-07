#[doc = "Register `buf_input` reader"]
pub type R = crate::R<BufInputSpec>;
#[doc = "Register `buf_input` writer"]
pub type W = crate::W<BufInputSpec>;
#[doc = "Field `width` reader - "]
pub type WidthR = crate::FieldReader<u16>;
#[doc = "Field `width` writer - "]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `height` reader - "]
pub type HeightR = crate::FieldReader<u16>;
#[doc = "Field `height` writer - "]
pub type HeightW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `channels` reader - "]
pub type ChannelsR = crate::FieldReader<u16>;
#[doc = "Field `channels` writer - "]
pub type ChannelsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn height(&self) -> HeightR {
        HeightR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    pub fn channels(&self) -> ChannelsR {
        ChannelsR::new(((self.bits >> 18) & 0x0fff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_input")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("channels", &self.channels())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<BufInputSpec> {
        WidthW::new(self, 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HeightW<BufInputSpec> {
        HeightW::new(self, 9)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    #[must_use]
    pub fn channels(&mut self) -> ChannelsW<BufInputSpec> {
        ChannelsW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufInputSpec;
impl crate::RegisterSpec for BufInputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_input::R`](R) reader structure"]
impl crate::Readable for BufInputSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_input::W`](W) writer structure"]
impl crate::Writable for BufInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_input to value 0"]
impl crate::Resettable for BufInputSpec {
    const RESET_VALUE: u32 = 0;
}
