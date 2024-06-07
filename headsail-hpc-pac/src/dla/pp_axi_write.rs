#[doc = "Register `pp_axi_write` reader"]
pub type R = crate::R<PpAxiWriteSpec>;
#[doc = "Register `pp_axi_write` writer"]
pub type W = crate::W<PpAxiWriteSpec>;
#[doc = "Field `addr` reader - "]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - "]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pp_axi_write")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<PpAxiWriteSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_write::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_write::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpAxiWriteSpec;
impl crate::RegisterSpec for PpAxiWriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp_axi_write::R`](R) reader structure"]
impl crate::Readable for PpAxiWriteSpec {}
#[doc = "`write(|w| ..)` method takes [`pp_axi_write::W`](W) writer structure"]
impl crate::Writable for PpAxiWriteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pp_axi_write to value 0"]
impl crate::Resettable for PpAxiWriteSpec {
    const RESET_VALUE: u32 = 0;
}
