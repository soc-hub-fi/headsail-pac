#[doc = "Register `msip[%s]` reader"]
pub type R = crate::R<MsipSpec>;
#[doc = "Register `msip[%s]` writer"]
pub type W = crate::W<MsipSpec>;
#[doc = "Field `msip` reader - "]
pub type MsipR = crate::BitReader;
#[doc = "Field `msip` writer - "]
pub type MsipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn msip(&self) -> MsipR {
        MsipR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("msip").field("msip", &self.msip()).finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn msip(&mut self) -> MsipW<MsipSpec> {
        MsipW::new(self, 0)
    }
}
#[doc = "Array of machine mode software interrupts (IPI) for all Harts Machine-mode software interrupts are generated by writing to msip. Software interrupts are commonly usde for interprocessor communication. This is usually done by one Hart writing into another Harts msip register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsipSpec;
impl crate::RegisterSpec for MsipSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`msip::R`](R) reader structure"]
impl crate::Readable for MsipSpec {}
#[doc = "`write(|w| ..)` method takes [`msip::W`](W) writer structure"]
impl crate::Writable for MsipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets msip[%s]
to value 0"]
impl crate::Resettable for MsipSpec {
    const RESET_VALUE: u64 = 0;
}
