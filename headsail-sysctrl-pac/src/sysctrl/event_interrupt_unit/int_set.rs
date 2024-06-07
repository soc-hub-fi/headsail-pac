#[doc = "Register `INT_set` reader"]
pub type R = crate::R<IntSetSpec>;
#[doc = "Register `INT_set` writer"]
pub type W = crate::W<IntSetSpec>;
#[doc = "Field `INT_set` reader - "]
pub type IntSetR = crate::FieldReader<u32>;
#[doc = "Field `INT_set` writer - "]
pub type IntSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_set(&self) -> IntSetR {
        IntSetR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_set")
            .field("int_set", &self.int_set())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn int_set(&mut self) -> IntSetW<IntSetSpec> {
        IntSetW::new(self, 0)
    }
}
#[doc = "INT_read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSetSpec;
impl crate::RegisterSpec for IntSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_set::R`](R) reader structure"]
impl crate::Readable for IntSetSpec {}
#[doc = "`write(|w| ..)` method takes [`int_set::W`](W) writer structure"]
impl crate::Writable for IntSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_set to value 0"]
impl crate::Resettable for IntSetSpec {
    const RESET_VALUE: u32 = 0;
}
