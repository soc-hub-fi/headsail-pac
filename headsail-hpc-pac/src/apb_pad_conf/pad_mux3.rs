#[doc = "Register `pad_mux3` reader"]
pub type R = crate::R<PAD_MUX3_SPEC>;
#[doc = "Register `pad_mux3` writer"]
pub type W = crate::W<PAD_MUX3_SPEC>;
#[doc = "Field `mux_select` reader - "]
pub type MUX_SELECT_R = crate::FieldReader;
#[doc = "Field `mux_select` writer - "]
pub type MUX_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mux_select(&self) -> MUX_SELECT_R {
        MUX_SELECT_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pad_mux3")
            .field("mux_select", &self.mux_select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn mux_select(&mut self) -> MUX_SELECT_W<PAD_MUX3_SPEC> {
        MUX_SELECT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_mux3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_mux3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_MUX3_SPEC;
impl crate::RegisterSpec for PAD_MUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_mux3::R`](R) reader structure"]
impl crate::Readable for PAD_MUX3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_mux3::W`](W) writer structure"]
impl crate::Writable for PAD_MUX3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pad_mux3 to value 0"]
impl crate::Resettable for PAD_MUX3_SPEC {
    const RESET_VALUE: u32 = 0;
}
