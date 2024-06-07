#[doc = "Register `t_wr` reader"]
pub type R = crate::R<TWrSpec>;
#[doc = "Register `t_wr` writer"]
pub type W = crate::W<TWrSpec>;
#[doc = "Field `t_wr` reader - "]
pub type TWrR = crate::FieldReader<u32>;
#[doc = "Field `t_wr` writer - "]
pub type TWrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t_wr(&self) -> TWrR {
        TWrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("t_wr").field("t_wr", &self.t_wr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn t_wr(&mut self) -> TWrW<TWrSpec> {
        TWrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t_wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWrSpec;
impl crate::RegisterSpec for TWrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_wr::R`](R) reader structure"]
impl crate::Readable for TWrSpec {}
#[doc = "`write(|w| ..)` method takes [`t_wr::W`](W) writer structure"]
impl crate::Writable for TWrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets t_wr to value 0x01"]
impl crate::Resettable for TWrSpec {
    const RESET_VALUE: u32 = 0x01;
}
