#[doc = "Register `interface_version` reader"]
pub type R = crate::R<InterfaceVersionSpec>;
#[doc = "Field `interface_version` reader - "]
pub type InterfaceVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interface_version(&self) -> InterfaceVersionR {
        InterfaceVersionR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("interface_version")
            .field("interface_version", &self.interface_version())
            .finish()
    }
}
#[doc = "Interface version (0x3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterfaceVersionSpec;
impl crate::RegisterSpec for InterfaceVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interface_version::R`](R) reader structure"]
impl crate::Readable for InterfaceVersionSpec {}
#[doc = "`reset()` method sets interface_version to value 0"]
impl crate::Resettable for InterfaceVersionSpec {
    const RESET_VALUE: u32 = 0;
}
