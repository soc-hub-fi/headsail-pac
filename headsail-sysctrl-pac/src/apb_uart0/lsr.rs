#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `LSR` reader - "]
pub type LsrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lsr(&self) -> LsrR {
        LsrR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSR").field("lsr", &self.lsr()).finish()
    }
}
#[doc = "LSR is the line status register => LSR\\[0\\]: rx fifo data valid => LSR\\[2\\]: parity error from the rx fifo => LSR\\[5\\]: the tx fifo is empty => LSR\\[6\\]: shift register and tx fifo are empty => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u8 = 0;
}
