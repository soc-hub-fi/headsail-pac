#[doc = "Register `DIR` reader"]
pub type R = crate::R<DirSpec>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DirSpec>;
#[doc = "Field `DIR` reader - "]
pub type DirR = crate::FieldReader<u32>;
#[doc = "Field `DIR` writer - "]
pub type DirW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR").field("dir", &self.dir()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<DirSpec> {
        DirW::new(self, 0)
    }
}
#[doc = "GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DirSpec {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DirSpec {
    const RESET_VALUE: u32 = 0;
}
