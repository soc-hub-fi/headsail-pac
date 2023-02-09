#[doc = "Register `PAD_CONFIG` reader"]
pub struct R(crate::R<PAD_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CONFIG` writer"]
pub struct W(crate::W<PAD_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CONFIG_SPEC>;
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
impl From<crate::W<PAD_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Write_Pad_Configuration` reader - "]
pub type WRITE_PAD_CONFIGURATION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Write_Pad_Configuration` writer - "]
pub type WRITE_PAD_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CONFIG_SPEC, u16, u16, 10, O>;
#[doc = "Field `Read_Pad_Configuration` reader - "]
pub type READ_PAD_CONFIGURATION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Read_Pad_Configuration` writer - "]
pub type READ_PAD_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CONFIG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn write_pad_configuration(&self) -> WRITE_PAD_CONFIGURATION_R {
        WRITE_PAD_CONFIGURATION_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn read_pad_configuration(&self) -> READ_PAD_CONFIGURATION_R {
        READ_PAD_CONFIGURATION_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn write_pad_configuration(&mut self) -> WRITE_PAD_CONFIGURATION_W<0> {
        WRITE_PAD_CONFIGURATION_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn read_pad_configuration(&mut self) -> READ_PAD_CONFIGURATION_W<10> {
        READ_PAD_CONFIGURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures Pad, 10 Bits for read pad configuration 10 bits for write pad configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_config](index.html) module"]
pub struct PAD_CONFIG_SPEC;
impl crate::RegisterSpec for PAD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_config::R](R) reader structure"]
impl crate::Readable for PAD_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_config::W](W) writer structure"]
impl crate::Writable for PAD_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CONFIG to value 0"]
impl crate::Resettable for PAD_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
