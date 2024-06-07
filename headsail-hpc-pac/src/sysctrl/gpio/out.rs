#[doc = "Register `OUT` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "Field `DATA_OUT` reader - "]
pub type DataOutR = crate::FieldReader<u32>;
#[doc = "Field `DATA_OUT` writer - "]
pub type DataOutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR {
        DataOutR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT")
            .field("data_out", &self.data_out())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data_out(&mut self) -> DataOutW<OutSpec> {
        DataOutW::new(self, 0)
    }
}
#[doc = "GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OutSpec {
    const RESET_VALUE: u32 = 0;
}
