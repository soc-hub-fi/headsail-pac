#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `IE` reader - Enable interrupts"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Enable interrupts"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enable the I2C peripheral."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable the I2C peripheral."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Enable interrupts"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the I2C peripheral."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ie", &self.ie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - Enable interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CtrlSpec> {
        IeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the I2C peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 7)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
