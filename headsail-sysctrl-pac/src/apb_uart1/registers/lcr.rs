#[doc = "Register `LCR` reader"]
pub type R = crate::R<LCR_SPEC>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LCR_SPEC>;
#[doc = "Field `LCR` reader - "]
pub type LCR_R = crate::FieldReader;
#[doc = "Field `LCR` writer - "]
pub type LCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lcr(&self) -> LCR_R {
        LCR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR").field("lcr", &self.lcr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn lcr(&mut self) -> LCR_W<LCR_SPEC> {
        LCR_W::new(self, 0)
    }
}
#[doc = "LCR is a register configures the main operation of the uart. It configures the width of the received data, stop bit, parity, and DLAB bit. => LCR\\[1:0\\]: configuration bits =>00: data is configured to be 5 bits =>01: data is configured to be 6 bits =>10: data is configured to be 7 bits =>11: data is configured to be 8 bits => LCR\\[2\\]: stop bit configuration =>0: 1 stop bit =>1: 1.5 stop bits for 5 bits data word OR 2 stop bits 6, 7 or 8 bits data word => LCR\\[3\\]: parity enable bit => LCR\\[7\\]: divisor latch access bit (DLAB) =>0: RBR, THR and IER accessible =>1: DLL and DLM accessible => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
