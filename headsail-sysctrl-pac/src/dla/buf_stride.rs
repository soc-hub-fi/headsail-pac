#[doc = "Register `buf_stride` reader"]
pub type R = crate::R<BUF_STRIDE_SPEC>;
#[doc = "Register `buf_stride` writer"]
pub type W = crate::W<BUF_STRIDE_SPEC>;
#[doc = "Field `x` reader - "]
pub type X_R = crate::FieldReader;
#[doc = "Field `x` writer - "]
pub type X_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `y` reader - "]
pub type Y_R = crate::FieldReader;
#[doc = "Field `y` writer - "]
pub type Y_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_stride")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<BUF_STRIDE_SPEC> {
        X_W::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<BUF_STRIDE_SPEC> {
        Y_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_stride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_stride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_STRIDE_SPEC;
impl crate::RegisterSpec for BUF_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_stride::R`](R) reader structure"]
impl crate::Readable for BUF_STRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_stride::W`](W) writer structure"]
impl crate::Writable for BUF_STRIDE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_stride to value 0"]
impl crate::Resettable for BUF_STRIDE_SPEC {
    const RESET_VALUE: u32 = 0;
}
