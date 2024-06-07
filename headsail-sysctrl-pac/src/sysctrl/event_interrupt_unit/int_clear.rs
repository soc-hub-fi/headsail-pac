#[doc = "Register `INT_clear` reader"]
pub type R = crate::R<IntClearSpec>;
#[doc = "Register `INT_clear` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `INT_clear` reader - "]
pub type IntClearR = crate::FieldReader<u32>;
#[doc = "Field `INT_clear` writer - "]
pub type IntClearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_clear(&self) -> IntClearR {
        IntClearR::new(self.bits)
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
    pub fn int_clear(&mut self) -> IntClearW<IntClearSpec> {
        IntClearW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clear::R`](R) reader structure"]
impl crate::Readable for IntClearSpec {}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_clear to value 0"]
impl crate::Resettable for IntClearSpec {
    const RESET_VALUE: u32 = 0;
}
