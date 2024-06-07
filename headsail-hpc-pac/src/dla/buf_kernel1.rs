#[doc = "Register `buf_kernel1` reader"]
pub type R = crate::R<BufKernel1Spec>;
#[doc = "Register `buf_kernel1` writer"]
pub type W = crate::W<BufKernel1Spec>;
#[doc = "Field `num` reader - "]
pub type NumR = crate::FieldReader<u16>;
#[doc = "Field `num` writer - "]
pub type NumW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn num(&self) -> NumR {
        NumR::new((self.bits & 0x0fff) as u16)
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
    pub fn num(&mut self) -> NumW<BufKernel1Spec> {
        NumW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufKernel1Spec;
impl crate::RegisterSpec for BufKernel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_kernel1::R`](R) reader structure"]
impl crate::Readable for BufKernel1Spec {}
#[doc = "`write(|w| ..)` method takes [`buf_kernel1::W`](W) writer structure"]
impl crate::Writable for BufKernel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets buf_kernel1 to value 0"]
impl crate::Resettable for BufKernel1Spec {
    const RESET_VALUE: u32 = 0;
}
