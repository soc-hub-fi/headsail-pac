#[doc = "Register `CLK_DIVIDER_CONF` reader"]
pub type R = crate::R<ClkDividerConfSpec>;
#[doc = "Register `CLK_DIVIDER_CONF` writer"]
pub type W = crate::W<ClkDividerConfSpec>;
#[doc = "Field `DIVIDER` reader - "]
pub type DividerR = crate::FieldReader;
#[doc = "Field `DIVIDER` writer - "]
pub type DividerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn divider(&self) -> DividerR {
        DividerR::new((self.bits & 0x0f) as u8)
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
    pub fn divider(&mut self) -> DividerW<ClkDividerConfSpec> {
        DividerW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_divider_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_divider_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDividerConfSpec;
impl crate::RegisterSpec for ClkDividerConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_divider_conf::R`](R) reader structure"]
impl crate::Readable for ClkDividerConfSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_divider_conf::W`](W) writer structure"]
impl crate::Writable for ClkDividerConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DIVIDER_CONF to value 0"]
impl crate::Resettable for ClkDividerConfSpec {
    const RESET_VALUE: u32 = 0;
}
