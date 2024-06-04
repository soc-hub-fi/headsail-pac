#[doc = "Register `device_class` reader"]
pub type R = crate::R<DEVICE_CLASS_SPEC>;
#[doc = "Field `device_class` reader - "]
pub type DEVICE_CLASS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn device_class(&self) -> DEVICE_CLASS_R {
        DEVICE_CLASS_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("device_class")
            .field("device_class", &self.device_class())
            .finish()
    }
}
#[doc = "Device class (0x774)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_class::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICE_CLASS_SPEC;
impl crate::RegisterSpec for DEVICE_CLASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_class::R`](R) reader structure"]
impl crate::Readable for DEVICE_CLASS_SPEC {}
#[doc = "`reset()` method sets device_class to value 0x0774"]
impl crate::Resettable for DEVICE_CLASS_SPEC {
    const RESET_VALUE: u32 = 0x0774;
}
