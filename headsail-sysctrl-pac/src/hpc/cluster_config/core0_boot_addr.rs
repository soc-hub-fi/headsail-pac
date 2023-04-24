#[doc = "Register `core0_boot_addr` reader"]
pub struct R(crate::R<CORE0_BOOT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_BOOT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_BOOT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_BOOT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core0_boot_addr` writer"]
pub struct W(crate::W<CORE0_BOOT_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_BOOT_ADDR_SPEC>;
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
impl From<crate::W<CORE0_BOOT_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_BOOT_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `boot_addr` reader - "]
pub type BOOT_ADDR_R = crate::FieldReader<u64, u64>;
#[doc = "Field `boot_addr` writer - "]
pub type BOOT_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, CORE0_BOOT_ADDR_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn boot_addr(&self) -> BOOT_ADDR_R {
        BOOT_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn boot_addr(&mut self) -> BOOT_ADDR_W<0> {
        BOOT_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core #0 boot address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_boot_addr](index.html) module"]
pub struct CORE0_BOOT_ADDR_SPEC;
impl crate::RegisterSpec for CORE0_BOOT_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core0_boot_addr::R](R) reader structure"]
impl crate::Readable for CORE0_BOOT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core0_boot_addr::W](W) writer structure"]
impl crate::Writable for CORE0_BOOT_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core0_boot_addr to value 0x0001_0000"]
impl crate::Resettable for CORE0_BOOT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
