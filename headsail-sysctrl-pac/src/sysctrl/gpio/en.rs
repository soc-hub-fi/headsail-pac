#[doc = "Register `EN` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "Field `GPIOEN` reader - "]
pub type GpioenR = crate::FieldReader<u32>;
#[doc = "Field `GPIOEN` writer - "]
pub type GpioenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpioen(&self) -> GpioenR {
        GpioenR::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EN")
            .field("gpioen", &self.gpioen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpioen(&mut self) -> GpioenW<EnSpec> {
        GpioenW::new(self, 0)
    }
}
#[doc = "GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EnSpec {
    const RESET_VALUE: u32 = 0;
}
