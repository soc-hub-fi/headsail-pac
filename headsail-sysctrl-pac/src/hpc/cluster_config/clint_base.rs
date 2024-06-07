#[doc = "Register `clint_base` reader"]
pub type R = crate::R<ClintBaseSpec>;
#[doc = "Register `clint_base` writer"]
pub type W = crate::W<ClintBaseSpec>;
#[doc = "Field `base` reader - "]
pub type BaseR = crate::FieldReader<u64>;
#[doc = "Field `base` writer - "]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("clint_base")
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<ClintBaseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clint_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clint_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClintBaseSpec;
impl crate::RegisterSpec for ClintBaseSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`clint_base::R`](R) reader structure"]
impl crate::Readable for ClintBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`clint_base::W`](W) writer structure"]
impl crate::Writable for ClintBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets clint_base to value 0x0006_0000"]
impl crate::Resettable for ClintBaseSpec {
    const RESET_VALUE: u64 = 0x0006_0000;
}
