#[doc = "Register `t_init` reader"]
pub type R = crate::R<TInitSpec>;
#[doc = "Register `t_init` writer"]
pub type W = crate::W<TInitSpec>;
#[doc = "Field `t_init` reader - "]
pub type TInitR = crate::FieldReader<u32>;
#[doc = "Field `t_init` writer - "]
pub type TInitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_init(&self) -> TInitR {
        TInitR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_init")
            .field("t_init", &self.t_init())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_init(&mut self) -> TInitW<TInitSpec> {
        TInitW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TInitSpec;
impl crate::RegisterSpec for TInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_init::R`](R) reader structure"]
impl crate::Readable for TInitSpec {}
#[doc = "`write(|w| ..)` method takes [`t_init::W`](W) writer structure"]
impl crate::Writable for TInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_init to value 0x2710"]
impl crate::Resettable for TInitSpec {
    const RESET_VALUE: u32 = 0x2710;
}
