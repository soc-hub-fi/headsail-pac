#[doc = "Register `IIR_FCR` reader"]
pub type R = crate::R<IirFcrSpec>;
#[doc = "Register `IIR_FCR` writer"]
pub type W = crate::W<IirFcrSpec>;
#[doc = "Field `IIR_FCR` reader - "]
pub type IirFcrR = crate::FieldReader;
#[doc = "Field `IIR_FCR` writer - "]
pub type IirFcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn iir_fcr(&self) -> IirFcrR {
        IirFcrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IIR_FCR")
            .field("iir_fcr", &self.iir_fcr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn iir_fcr(&mut self) -> IirFcrW<IirFcrSpec> {
        IirFcrW::new(self, 0)
    }
}
#[doc = "IIR_FCR is a multi-purpose register address. you can access 2 different registers using the same address. However LCR\\[7\\]
isn't needed in this case because one of them is only written and the other is only read. FCR (fifo control register) write only => FCR\\[1\\]: clears the rx fifo if high => FCR\\[2\\]: clears the tx fifo if high => FCR\\[7:6\\]: sets the trigger level =>00: trigger level is high when there is 1 element in the fifo =>01: trigger level is high when there are 4 elements in the fifo =>10: trigger level is high when there are 8 elements in the fifo =>11: trigger level is high when there are 14 elements in the fifo => other bits aren't used IIR (Interrupt Identification Register) read only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirFcrSpec;
impl crate::RegisterSpec for IirFcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iir_fcr::R`](R) reader structure"]
impl crate::Readable for IirFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`iir_fcr::W`](W) writer structure"]
impl crate::Writable for IirFcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IIR_FCR to value 0"]
impl crate::Resettable for IirFcrSpec {
    const RESET_VALUE: u8 = 0;
}
