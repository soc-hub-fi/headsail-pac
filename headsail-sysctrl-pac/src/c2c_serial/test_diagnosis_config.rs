#[doc = "Register `TEST_DIAGNOSIS_CONFIG[%s]` reader"]
pub type R = crate::R<TestDiagnosisConfigSpec>;
#[doc = "Register `TEST_DIAGNOSIS_CONFIG[%s]` writer"]
pub type W = crate::W<TestDiagnosisConfigSpec>;
#[doc = "Field `CONFIG` reader - "]
pub type ConfigR = crate::FieldReader<u32>;
#[doc = "Field `CONFIG` writer - "]
pub type ConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_DIAGNOSIS_CONFIG")
            .field("config", &self.config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> ConfigW<TestDiagnosisConfigSpec> {
        ConfigW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test_diagnosis_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_diagnosis_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestDiagnosisConfigSpec;
impl crate::RegisterSpec for TestDiagnosisConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_diagnosis_config::R`](R) reader structure"]
impl crate::Readable for TestDiagnosisConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`test_diagnosis_config::W`](W) writer structure"]
impl crate::Writable for TestDiagnosisConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST_DIAGNOSIS_CONFIG[%s]
to value 0"]
impl crate::Resettable for TestDiagnosisConfigSpec {
    const RESET_VALUE: u32 = 0;
}
