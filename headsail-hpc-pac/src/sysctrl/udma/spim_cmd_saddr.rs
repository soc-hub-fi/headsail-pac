#[doc = "Register `SPIM_CMD_SADDR` reader"]
pub type R = crate::R<SPIM_CMD_SADDR_SPEC>;
#[doc = "Register `SPIM_CMD_SADDR` writer"]
pub type W = crate::W<SPIM_CMD_SADDR_SPEC>;
#[doc = "Field `CMD_SADDR` reader - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub type CMD_SADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CMD_SADDR` writer - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub type CMD_SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn cmd_saddr(&self) -> CMD_SADDR_R {
        CMD_SADDR_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIM_CMD_SADDR")
            .field("cmd_saddr", &self.cmd_saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_saddr(&mut self) -> CMD_SADDR_W<SPIM_CMD_SADDR_SPEC> {
        CMD_SADDR_W::new(self, 0)
    }
}
#[doc = "CMD SPI uDMA transfer address of associated buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spim_cmd_saddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spim_cmd_saddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPIM_CMD_SADDR_SPEC;
impl crate::RegisterSpec for SPIM_CMD_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spim_cmd_saddr::R`](R) reader structure"]
impl crate::Readable for SPIM_CMD_SADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spim_cmd_saddr::W`](W) writer structure"]
impl crate::Writable for SPIM_CMD_SADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIM_CMD_SADDR to value 0"]
impl crate::Resettable for SPIM_CMD_SADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
