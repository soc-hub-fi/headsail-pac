#[doc = "Register `IER_DLM` reader"]
pub type R = crate::R<IerDlmSpec>;
#[doc = "Register `IER_DLM` writer"]
pub type W = crate::W<IerDlmSpec>;
#[doc = "Field `IER_DLM` reader - "]
pub type IerDlmR = crate::FieldReader;
#[doc = "Field `IER_DLM` writer - "]
pub type IerDlmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ier_dlm(&self) -> IerDlmR {
        IerDlmR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER_DLM")
            .field("ier_dlm", &self.ier_dlm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ier_dlm(&mut self) -> IerDlmW<IerDlmSpec> {
        IerDlmW::new(self, 0)
    }
}
#[doc = "IER_DLM is a multi-purpose register address. you can access 2 different registers using the same address. IF LCR\\[7\\]
is 0 IER is accessable. O.W DLM is accessable. IER writes/reads into/from position IER => IER\\[2:0\\]: The interrupt enable bits. => xx1: Interrupt is raised when Received data available or trigger level reached in FIFO mode if non of these it checks Character timeout indication. => x10: Interrupt is raised when Transmitter holding register empty. => 100: Interrupt is raised when Overrun error, parity error, framing error or break interrupt. => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier_dlm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier_dlm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerDlmSpec;
impl crate::RegisterSpec for IerDlmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ier_dlm::R`](R) reader structure"]
impl crate::Readable for IerDlmSpec {}
#[doc = "`write(|w| ..)` method takes [`ier_dlm::W`](W) writer structure"]
impl crate::Writable for IerDlmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IER_DLM to value 0"]
impl crate::Resettable for IerDlmSpec {
    const RESET_VALUE: u8 = 0;
}
