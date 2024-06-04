#[doc = "Register `CTRL_CFG_RST` reader"]
pub type R = crate::R<CTRL_CFG_RST_SPEC>;
#[doc = "Register `CTRL_CFG_RST` writer"]
pub type W = crate::W<CTRL_CFG_RST_SPEC>;
#[doc = "Field `CTRL_CFG_RST` reader - uDMA peripherals reset trigger (unimplemented)"]
pub type CTRL_CFG_RST_R = crate::FieldReader<u32>;
#[doc = "Field `CTRL_CFG_RST` writer - uDMA peripherals reset trigger (unimplemented)"]
pub type CTRL_CFG_RST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    pub fn ctrl_cfg_rst(&self) -> CTRL_CFG_RST_R {
        CTRL_CFG_RST_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CFG_RST")
            .field("ctrl_cfg_rst", &self.ctrl_cfg_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - uDMA peripherals reset trigger (unimplemented)"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_cfg_rst(&mut self) -> CTRL_CFG_RST_W<CTRL_CFG_RST_SPEC> {
        CTRL_CFG_RST_W::new(self, 0)
    }
}
#[doc = "uDMA peripherals reset trigger (unimplemented)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_CFG_RST_SPEC;
impl crate::RegisterSpec for CTRL_CFG_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_cfg_rst::R`](R) reader structure"]
impl crate::Readable for CTRL_CFG_RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_cfg_rst::W`](W) writer structure"]
impl crate::Writable for CTRL_CFG_RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_RST to value 0"]
impl crate::Resettable for CTRL_CFG_RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
