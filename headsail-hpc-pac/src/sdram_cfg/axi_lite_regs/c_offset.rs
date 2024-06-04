#[doc = "Register `c_offset` reader"]
pub type R = crate::R<C_OFFSET_SPEC>;
#[doc = "Register `c_offset` writer"]
pub type W = crate::W<C_OFFSET_SPEC>;
#[doc = "Field `c_offset` reader - "]
pub type C_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `c_offset` writer - "]
pub type C_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_offset(&self) -> C_OFFSET_R {
        C_OFFSET_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_offset")
            .field("c_offset", &self.c_offset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_offset(&mut self) -> C_OFFSET_W<C_OFFSET_SPEC> {
        C_OFFSET_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_OFFSET_SPEC;
impl crate::RegisterSpec for C_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_offset::R`](R) reader structure"]
impl crate::Readable for C_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c_offset::W`](W) writer structure"]
impl crate::Writable for C_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_offset to value 0"]
impl crate::Resettable for C_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
