#[doc = "Register `buf_stride` reader"]
pub type R = crate::R<BufStrideSpec>;
#[doc = "Register `buf_stride` writer"]
pub type W = crate::W<BufStrideSpec>;
#[doc = "Field `x` reader - "]
pub type XR = crate::FieldReader;
#[doc = "Field `x` writer - "]
pub type XW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `y` reader - "]
pub type YR = crate::FieldReader;
#[doc = "Field `y` writer - "]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 16) & 0x0f) as u8)
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
    pub fn x(&mut self) -> XW<BufStrideSpec> {
        XW::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> YW<BufStrideSpec> {
        YW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_stride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_stride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufStrideSpec;
impl crate::RegisterSpec for BufStrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_stride::R`](R) reader structure"]
impl crate::Readable for BufStrideSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_stride::W`](W) writer structure"]
impl crate::Writable for BufStrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_stride to value 0"]
impl crate::Resettable for BufStrideSpec {
    const RESET_VALUE: u32 = 0;
}
