#[doc = "Register `CLK_CTRL3` reader"]
pub struct R(crate::R<CLK_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL3` writer"]
pub struct W(crate::W<CLK_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIPH_sel_cka` reader - Select CKA"]
pub type PERIPH_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_sel_cka` writer - Select CKA"]
pub type PERIPH_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL3_SPEC, bool, O>;
#[doc = "Field `PERIPH_force_cka` reader - Force CKA"]
pub type PERIPH_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_force_cka` writer - Force CKA"]
pub type PERIPH_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL3_SPEC, bool, O>;
#[doc = "Field `PERIPH_force_ckb` reader - Force CKB"]
pub type PERIPH_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_force_ckb` writer - Force CKB"]
pub type PERIPH_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL3_SPEC, bool, O>;
#[doc = "Field `PERIPH_subsys_clkena` reader - Subsystem clock enable"]
pub type PERIPH_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_subsys_clkena` writer - Subsystem clock enable"]
pub type PERIPH_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL3_SPEC, bool, O>;
#[doc = "Field `PERIPH_pll_ctrl_valid` reader - PLL Control valid"]
pub type PERIPH_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `PERIPH_pll_ctrl_valid` writer - PLL Control valid"]
pub type PERIPH_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn periph_sel_cka(&self) -> PERIPH_SEL_CKA_R {
        PERIPH_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn periph_force_cka(&self) -> PERIPH_FORCE_CKA_R {
        PERIPH_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn periph_force_ckb(&self) -> PERIPH_FORCE_CKB_R {
        PERIPH_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn periph_subsys_clkena(&self) -> PERIPH_SUBSYS_CLKENA_R {
        PERIPH_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn periph_pll_ctrl_valid(&self) -> PERIPH_PLL_CTRL_VALID_R {
        PERIPH_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn periph_sel_cka(&mut self) -> PERIPH_SEL_CKA_W<0> {
        PERIPH_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn periph_force_cka(&mut self) -> PERIPH_FORCE_CKA_W<1> {
        PERIPH_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn periph_force_ckb(&mut self) -> PERIPH_FORCE_CKB_W<2> {
        PERIPH_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn periph_subsys_clkena(&mut self) -> PERIPH_SUBSYS_CLKENA_W<3> {
        PERIPH_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn periph_pll_ctrl_valid(&mut self) -> PERIPH_PLL_CTRL_VALID_W<7> {
        PERIPH_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection for PERIPH *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl3](index.html) module"]
pub struct CLK_CTRL3_SPEC;
impl crate::RegisterSpec for CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl3::R](R) reader structure"]
impl crate::Readable for CLK_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl3::W](W) writer structure"]
impl crate::Writable for CLK_CTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL3 to value 0"]
impl crate::Resettable for CLK_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
