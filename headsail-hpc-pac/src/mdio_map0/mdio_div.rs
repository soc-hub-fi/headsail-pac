#[doc = "Register `MDIO_DIV` reader"]
pub type R = crate::R<MdioDivSpec>;
#[doc = "Register `MDIO_DIV` writer"]
pub type W = crate::W<MdioDivSpec>;
#[doc = "Field `MDIO_DIV` reader - "]
pub type MdioDivR = crate::FieldReader;
#[doc = "Field `MDIO_DIV` writer - "]
pub type MdioDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mdio_div(&self) -> MdioDivR {
        MdioDivR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_DIV")
            .field("mdio_div", &self.mdio_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_div(&mut self) -> MdioDivW<MdioDivSpec> {
        MdioDivW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioDivSpec;
impl crate::RegisterSpec for MdioDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_div::R`](R) reader structure"]
impl crate::Readable for MdioDivSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_div::W`](W) writer structure"]
impl crate::Writable for MdioDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_DIV to value 0x0a"]
impl crate::Resettable for MdioDivSpec {
    const RESET_VALUE: u32 = 0x0a;
}
