#[doc = "Register `c_col_shift` reader"]
pub type R = crate::R<C_COL_SHIFT_SPEC>;
#[doc = "Register `c_col_shift` writer"]
pub type W = crate::W<C_COL_SHIFT_SPEC>;
#[doc = "Field `c_col_shift` reader - "]
pub type C_COL_SHIFT_R = crate::FieldReader<u32>;
#[doc = "Field `c_col_shift` writer - "]
pub type C_COL_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn c_col_shift(&self) -> C_COL_SHIFT_R {
        C_COL_SHIFT_R::new(self.bits)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("c_col_shift")
            .field("c_col_shift", &self.c_col_shift())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn c_col_shift(&mut self) -> C_COL_SHIFT_W<C_COL_SHIFT_SPEC> {
        C_COL_SHIFT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c_col_shift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c_col_shift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_COL_SHIFT_SPEC;
impl crate::RegisterSpec for C_COL_SHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c_col_shift::R`](R) reader structure"]
impl crate::Readable for C_COL_SHIFT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c_col_shift::W`](W) writer structure"]
impl crate::Writable for C_COL_SHIFT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets c_col_shift to value 0x01"]
impl crate::Resettable for C_COL_SHIFT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
