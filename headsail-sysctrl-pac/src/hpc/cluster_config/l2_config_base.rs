#[doc = "Register `l2_config_base` reader"]
pub type R = crate::R<L2ConfigBaseSpec>;
#[doc = "Register `l2_config_base` writer"]
pub type W = crate::W<L2ConfigBaseSpec>;
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
        f.debug_struct("l2_config_base")
            .field("base", &self.base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<L2ConfigBaseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_config_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_config_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2ConfigBaseSpec;
impl crate::RegisterSpec for L2ConfigBaseSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`l2_config_base::R`](R) reader structure"]
impl crate::Readable for L2ConfigBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_config_base::W`](W) writer structure"]
impl crate::Writable for L2ConfigBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets l2_config_base to value 0x0005_2000"]
impl crate::Resettable for L2ConfigBaseSpec {
    const RESET_VALUE: u64 = 0x0005_2000;
}
