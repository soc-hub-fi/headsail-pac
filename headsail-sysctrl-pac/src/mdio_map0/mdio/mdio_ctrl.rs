#[doc = "Register `MDIO_CTRL` reader"]
pub type R = crate::R<MDIO_CTRL_SPEC>;
#[doc = "Register `MDIO_CTRL` writer"]
pub type W = crate::W<MDIO_CTRL_SPEC>;
#[doc = "Field `MDIO_CTRL` reader - "]
pub type MDIO_CTRL_R = crate::FieldReader;
#[doc = "Field `MDIO_CTRL` writer - "]
pub type MDIO_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn mdio_ctrl(&self) -> MDIO_CTRL_R {
        MDIO_CTRL_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_CTRL")
            .field("mdio_ctrl", &self.mdio_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_ctrl(&mut self) -> MDIO_CTRL_W<MDIO_CTRL_SPEC> {
        MDIO_CTRL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIO_CTRL_SPEC;
impl crate::RegisterSpec for MDIO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_ctrl::R`](R) reader structure"]
impl crate::Readable for MDIO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdio_ctrl::W`](W) writer structure"]
impl crate::Writable for MDIO_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_CTRL to value 0"]
impl crate::Resettable for MDIO_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
