#[doc = "Register `ddr_init_done` reader"]
pub type R = crate::R<DdrInitDoneSpec>;
#[doc = "Register `ddr_init_done` writer"]
pub type W = crate::W<DdrInitDoneSpec>;
#[doc = "Field `ddr_init_done` reader - "]
pub type DdrInitDoneR = crate::FieldReader<u32>;
#[doc = "Field `ddr_init_done` writer - "]
pub type DdrInitDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ddr_init_done(&self) -> DdrInitDoneR {
        DdrInitDoneR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ddr_init_done")
            .field("ddr_init_done", &self.ddr_init_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_init_done(&mut self) -> DdrInitDoneW<DdrInitDoneSpec> {
        DdrInitDoneW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_init_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_init_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrInitDoneSpec;
impl crate::RegisterSpec for DdrInitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_init_done::R`](R) reader structure"]
impl crate::Readable for DdrInitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`ddr_init_done::W`](W) writer structure"]
impl crate::Writable for DdrInitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ddr_init_done to value 0"]
impl crate::Resettable for DdrInitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
