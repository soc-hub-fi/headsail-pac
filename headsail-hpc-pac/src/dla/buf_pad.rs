#[doc = "Register `buf_pad` reader"]
pub type R = crate::R<BUF_PAD_SPEC>;
#[doc = "Register `buf_pad` writer"]
pub type W = crate::W<BUF_PAD_SPEC>;
#[doc = "Field `top` reader - "]
pub type TOP_R = crate::FieldReader;
#[doc = "Field `top` writer - "]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `right` reader - "]
pub type RIGHT_R = crate::FieldReader;
#[doc = "Field `right` writer - "]
pub type RIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bottom` reader - "]
pub type BOTTOM_R = crate::FieldReader;
#[doc = "Field `bottom` writer - "]
pub type BOTTOM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `left` reader - "]
pub type LEFT_R = crate::FieldReader;
#[doc = "Field `left` writer - "]
pub type LEFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn right(&self) -> RIGHT_R {
        RIGHT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bottom(&self) -> BOTTOM_R {
        BOTTOM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn left(&self) -> LEFT_R {
        LEFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_pad")
            .field("top", &self.top())
            .field("right", &self.right())
            .field("bottom", &self.bottom())
            .field("left", &self.left())
            .field("value", &self.value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<BUF_PAD_SPEC> {
        TOP_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn right(&mut self) -> RIGHT_W<BUF_PAD_SPEC> {
        RIGHT_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn bottom(&mut self) -> BOTTOM_W<BUF_PAD_SPEC> {
        BOTTOM_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn left(&mut self) -> LEFT_W<BUF_PAD_SPEC> {
        LEFT_W::new(self, 12)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<BUF_PAD_SPEC> {
        VALUE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_pad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_PAD_SPEC;
impl crate::RegisterSpec for BUF_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_pad::R`](R) reader structure"]
impl crate::Readable for BUF_PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_pad::W`](W) writer structure"]
impl crate::Writable for BUF_PAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_pad to value 0"]
impl crate::Resettable for BUF_PAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
