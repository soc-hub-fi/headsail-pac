#[doc = "Register `buf_kernel1` reader"]
pub type R = crate::R<BUF_KERNEL1_SPEC>;
#[doc = "Register `buf_kernel1` writer"]
pub type W = crate::W<BUF_KERNEL1_SPEC>;
#[doc = "Field `num` reader - "]
pub type NUM_R = crate::FieldReader<u16>;
#[doc = "Field `num` writer - "]
pub type NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("buf_kernel1")
            .field("num", &self.num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NUM_W<BUF_KERNEL1_SPEC> {
        NUM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_KERNEL1_SPEC;
impl crate::RegisterSpec for BUF_KERNEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_kernel1::R`](R) reader structure"]
impl crate::Readable for BUF_KERNEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf_kernel1::W`](W) writer structure"]
impl crate::Writable for BUF_KERNEL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_kernel1 to value 0"]
impl crate::Resettable for BUF_KERNEL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
