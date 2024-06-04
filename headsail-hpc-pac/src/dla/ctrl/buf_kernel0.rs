#[doc = "Register `buf_kernel0` reader"]
pub type R = crate::R<BUF_KERNEL0_SPEC>;
#[doc = "Register `buf_kernel0` writer"]
pub type W = crate::W<BUF_KERNEL0_SPEC>;
#[doc = "Field `width` reader - "]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `width` writer - "]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `height` reader - "]
pub type HEIGHT_R = crate::FieldReader;
#[doc = "Field `height` writer - "]
pub type HEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `s_channels` reader - "]
pub type S_CHANNELS_R = crate::FieldReader<u16>;
#[doc = "Field `s_channels` writer - "]
pub type S_CHANNELS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    pub fn s_channels(&self) -> S_CHANNELS_R {
        S_CHANNELS_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_kernel0")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("s_channels", &self.s_channels())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<BUF_KERNEL0_SPEC> {
        WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<BUF_KERNEL0_SPEC> {
        HEIGHT_W::new(self, 4)
    }
    #[doc = "Bits 8:19"]
    #[inline(always)]
    #[must_use]
    pub fn s_channels(&mut self) -> S_CHANNELS_W<BUF_KERNEL0_SPEC> {
        S_CHANNELS_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_KERNEL0_SPEC;
impl crate::RegisterSpec for BUF_KERNEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_kernel0::R`](R) reader structure"]
impl crate::Readable for BUF_KERNEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_kernel0::W`](W) writer structure"]
impl crate::Writable for BUF_KERNEL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_kernel0 to value 0"]
impl crate::Resettable for BUF_KERNEL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
