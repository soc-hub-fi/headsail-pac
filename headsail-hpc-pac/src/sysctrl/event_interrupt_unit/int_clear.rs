#[doc = "Register `INT_clear` reader"]
pub type R = crate::R<INT_CLEAR_SPEC>;
#[doc = "Register `INT_clear` writer"]
pub type W = crate::W<INT_CLEAR_SPEC>;
#[doc = "Field `INT_clear` reader - "]
pub type INT_CLEAR_R = crate::FieldReader<u32>;
#[doc = "Field `INT_clear` writer - "]
pub type INT_CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_clear(&self) -> INT_CLEAR_R {
        INT_CLEAR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_clear")
            .field("int_clear", &self.int_clear())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn int_clear(&mut self) -> INT_CLEAR_W<INT_CLEAR_SPEC> {
        INT_CLEAR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLEAR_SPEC;
impl crate::RegisterSpec for INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clear::R`](R) reader structure"]
impl crate::Readable for INT_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for INT_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_clear to value 0"]
impl crate::Resettable for INT_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
