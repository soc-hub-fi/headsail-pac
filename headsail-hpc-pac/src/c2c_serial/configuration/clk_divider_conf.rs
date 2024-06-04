#[doc = "Register `CLK_DIVIDER_CONF` reader"]
pub type R = crate::R<CLK_DIVIDER_CONF_SPEC>;
#[doc = "Register `CLK_DIVIDER_CONF` writer"]
pub type W = crate::W<CLK_DIVIDER_CONF_SPEC>;
#[doc = "Field `DIVIDER` reader - "]
pub type DIVIDER_R = crate::FieldReader;
#[doc = "Field `DIVIDER` writer - "]
pub type DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_DIVIDER_CONF")
            .field("divider", &self.divider())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<CLK_DIVIDER_CONF_SPEC> {
        DIVIDER_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_divider_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_divider_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_DIVIDER_CONF_SPEC;
impl crate::RegisterSpec for CLK_DIVIDER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_divider_conf::R`](R) reader structure"]
impl crate::Readable for CLK_DIVIDER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_divider_conf::W`](W) writer structure"]
impl crate::Writable for CLK_DIVIDER_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DIVIDER_CONF to value 0"]
impl crate::Resettable for CLK_DIVIDER_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
