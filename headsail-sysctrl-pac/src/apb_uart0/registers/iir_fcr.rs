#[doc = "Register `IIR_FCR` reader"]
pub struct R(crate::R<IIR_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIR_FCR` writer"]
pub struct W(crate::W<IIR_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIR_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IIR_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIR_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IIR_FCR` reader - "]
pub type IIR_FCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IIR_FCR` writer - "]
pub type IIR_FCR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, IIR_FCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn iir_fcr(&self) -> IIR_FCR_R {
        IIR_FCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn iir_fcr(&mut self) -> IIR_FCR_W<0> {
        IIR_FCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IIR_FCR is a multi-purpose register address. you can access 2 different registers using the same address. However LCR\\[7\\]
isn't needed in this case because one of them is only written and the other is only read. FCR (fifo control register) write only => FCR\\[1\\]: clears the rx fifo if high => FCR\\[2\\]: clears the tx fifo if high => FCR\\[7:6\\]: sets the trigger level =>00: trigger level is high when there is 1 element in the fifo =>01: trigger level is high when there are 4 elements in the fifo =>10: trigger level is high when there are 8 elements in the fifo =>11: trigger level is high when there are 14 elements in the fifo => other bits aren't used IIR (Interrupt Identification Register) read only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir_fcr](index.html) module"]
pub struct IIR_FCR_SPEC;
impl crate::RegisterSpec for IIR_FCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iir_fcr::R](R) reader structure"]
impl crate::Readable for IIR_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iir_fcr::W](W) writer structure"]
impl crate::Writable for IIR_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIR_FCR to value 0"]
impl crate::Resettable for IIR_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
