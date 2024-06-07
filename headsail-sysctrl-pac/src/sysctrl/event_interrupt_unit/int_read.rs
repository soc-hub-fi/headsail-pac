#[doc = "Register `INT_read` reader"]
pub type R = crate::R<IntReadSpec>;
#[doc = "Register `INT_read` writer"]
pub type W = crate::W<IntReadSpec>;
#[doc = "Field `INT_read` reader - "]
pub type IntReadR = crate::FieldReader<u32>;
#[doc = "Field `INT_read` writer - "]
pub type IntReadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_read(&self) -> IntReadR {
        IntReadR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_read")
            .field("int_read", &self.int_read())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn int_read(&mut self) -> IntReadW<IntReadSpec> {
        IntReadW::new(self, 0)
    }
}
#[doc = "This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_read::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_read::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntReadSpec;
impl crate::RegisterSpec for IntReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_read::R`](R) reader structure"]
impl crate::Readable for IntReadSpec {}
#[doc = "`write(|w| ..)` method takes [`int_read::W`](W) writer structure"]
impl crate::Writable for IntReadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_read to value 0"]
impl crate::Resettable for IntReadSpec {
    const RESET_VALUE: u32 = 0;
}
