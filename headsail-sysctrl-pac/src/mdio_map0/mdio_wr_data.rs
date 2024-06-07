#[doc = "Register `MDIO_WR_DATA` reader"]
pub type R = crate::R<MDIO_WR_DATA_SPEC>;
#[doc = "Register `MDIO_WR_DATA` writer"]
pub type W = crate::W<MDIO_WR_DATA_SPEC>;
#[doc = "Field `MDIO_WR_DATA` reader - "]
pub type MDIO_WR_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `MDIO_WR_DATA` writer - "]
pub type MDIO_WR_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_wr_data(&self) -> MDIO_WR_DATA_R {
        MDIO_WR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIO_WR_DATA")
            .field("mdio_wr_data", &self.mdio_wr_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_wr_data(&mut self) -> MDIO_WR_DATA_W<MDIO_WR_DATA_SPEC> {
        MDIO_WR_DATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdio_wr_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdio_wr_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIO_WR_DATA_SPEC;
impl crate::RegisterSpec for MDIO_WR_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_wr_data::R`](R) reader structure"]
impl crate::Readable for MDIO_WR_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdio_wr_data::W`](W) writer structure"]
impl crate::Writable for MDIO_WR_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_WR_DATA to value 0"]
impl crate::Resettable for MDIO_WR_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
