#[doc = "Register `buf_input` reader"]
pub type R = crate::R<BUF_INPUT_SPEC>;
#[doc = "Register `buf_input` writer"]
pub type W = crate::W<BUF_INPUT_SPEC>;
#[doc = "Field `width` reader - "]
pub type WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `width` writer - "]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `height` reader - "]
pub type HEIGHT_R = crate::FieldReader<u16>;
#[doc = "Field `height` writer - "]
pub type HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `channels` reader - "]
pub type CHANNELS_R = crate::FieldReader<u16>;
#[doc = "Field `channels` writer - "]
pub type CHANNELS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new(((self.bits >> 18) & 0x0fff) as u16)
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
    pub fn width(&mut self) -> WIDTH_W<BUF_INPUT_SPEC> {
        WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<BUF_INPUT_SPEC> {
        HEIGHT_W::new(self, 9)
    }
    #[doc = "Bits 18:29"]
    #[inline(always)]
    #[must_use]
    pub fn channels(&mut self) -> CHANNELS_W<BUF_INPUT_SPEC> {
        CHANNELS_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_INPUT_SPEC;
impl crate::RegisterSpec for BUF_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_input::R`](R) reader structure"]
impl crate::Readable for BUF_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_input::W`](W) writer structure"]
impl crate::Writable for BUF_INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_input to value 0"]
impl crate::Resettable for BUF_INPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
