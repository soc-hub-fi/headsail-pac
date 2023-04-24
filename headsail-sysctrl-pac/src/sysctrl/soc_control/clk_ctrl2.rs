#[doc = "Register `CLK_CTRL2` reader"]
pub struct R(crate::R<CLK_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL2` writer"]
pub struct W(crate::W<CLK_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL2_SPEC>;
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
impl From<crate::W<CLK_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSP_sel_cka` reader - Select CKA"]
pub type DSP_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DSP_sel_cka` writer - Select CKA"]
pub type DSP_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `DSP_force_cka` reader - Force CKA"]
pub type DSP_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DSP_force_cka` writer - Force CKA"]
pub type DSP_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `DSP_force_ckb` reader - Force CKB"]
pub type DSP_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `DSP_force_ckb` writer - Force CKB"]
pub type DSP_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `DSP_subsys_clkena` reader - Subsystem clock enable"]
pub type DSP_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `DSP_subsys_clkena` writer - Subsystem clock enable"]
pub type DSP_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `DSP_pll_ctrl_valid` reader - PLL Control valid"]
pub type DSP_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DSP_pll_ctrl_valid` writer - PLL Control valid"]
pub type DSP_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `ICN_sel_cka` reader - Select CKA"]
pub type ICN_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ICN_sel_cka` writer - Select CKA"]
pub type ICN_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `ICN_force_cka` reader - Force CKA"]
pub type ICN_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ICN_force_cka` writer - Force CKA"]
pub type ICN_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `ICN_force_ckb` reader - Force CKB"]
pub type ICN_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `ICN_force_ckb` writer - Force CKB"]
pub type ICN_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `ICN_subsys_clkena` reader - Subsystem clock enable"]
pub type ICN_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `ICN_subsys_clkena` writer - Subsystem clock enable"]
pub type ICN_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `ICN_pll_ctrl_valid` reader - PLL Control valid"]
pub type ICN_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `ICN_pll_ctrl_valid` writer - PLL Control valid"]
pub type ICN_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_SER_sel_cka` reader - Select CKA"]
pub type C2C_SER_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_SER_sel_cka` writer - Select CKA"]
pub type C2C_SER_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_SER_force_cka` reader - Force CKA"]
pub type C2C_SER_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_SER_force_cka` writer - Force CKA"]
pub type C2C_SER_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_SER_force_ckb` reader - Force CKB"]
pub type C2C_SER_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `C2C_SER_force_ckb` writer - Force CKB"]
pub type C2C_SER_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_SER_subsys_clkena` reader - Subsystem clock enable"]
pub type C2C_SER_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_SER_subsys_clkena` writer - Subsystem clock enable"]
pub type C2C_SER_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_SER_pll_ctrl_valid` reader - PLL Control valid"]
pub type C2C_SER_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `C2C_SER_pll_ctrl_valid` writer - PLL Control valid"]
pub type C2C_SER_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_PAR_sel_cka` reader - Select CKA"]
pub type C2C_PAR_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_PAR_sel_cka` writer - Select CKA"]
pub type C2C_PAR_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_PAR_force_cka` reader - Force CKA"]
pub type C2C_PAR_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_PAR_force_cka` writer - Force CKA"]
pub type C2C_PAR_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_PAR_force_ckb` reader - Force CKB"]
pub type C2C_PAR_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `C2C_PAR_force_ckb` writer - Force CKB"]
pub type C2C_PAR_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_PAR_subsys_clkena` reader - Subsystem clock enable"]
pub type C2C_PAR_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `C2C_PAR_subsys_clkena` writer - Subsystem clock enable"]
pub type C2C_PAR_SUBSYS_CLKENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
#[doc = "Field `C2C_PAR_pll_ctrl_valid` reader - PLL Control valid"]
pub type C2C_PAR_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `C2C_PAR_pll_ctrl_valid` writer - PLL Control valid"]
pub type C2C_PAR_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn dsp_sel_cka(&self) -> DSP_SEL_CKA_R {
        DSP_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn dsp_force_cka(&self) -> DSP_FORCE_CKA_R {
        DSP_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn dsp_force_ckb(&self) -> DSP_FORCE_CKB_R {
        DSP_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn dsp_subsys_clkena(&self) -> DSP_SUBSYS_CLKENA_R {
        DSP_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn dsp_pll_ctrl_valid(&self) -> DSP_PLL_CTRL_VALID_R {
        DSP_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn icn_sel_cka(&self) -> ICN_SEL_CKA_R {
        ICN_SEL_CKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn icn_force_cka(&self) -> ICN_FORCE_CKA_R {
        ICN_FORCE_CKA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn icn_force_ckb(&self) -> ICN_FORCE_CKB_R {
        ICN_FORCE_CKB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn icn_subsys_clkena(&self) -> ICN_SUBSYS_CLKENA_R {
        ICN_SUBSYS_CLKENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn icn_pll_ctrl_valid(&self) -> ICN_PLL_CTRL_VALID_R {
        ICN_PLL_CTRL_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn c2c_ser_sel_cka(&self) -> C2C_SER_SEL_CKA_R {
        C2C_SER_SEL_CKA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn c2c_ser_force_cka(&self) -> C2C_SER_FORCE_CKA_R {
        C2C_SER_FORCE_CKA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn c2c_ser_force_ckb(&self) -> C2C_SER_FORCE_CKB_R {
        C2C_SER_FORCE_CKB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn c2c_ser_subsys_clkena(&self) -> C2C_SER_SUBSYS_CLKENA_R {
        C2C_SER_SUBSYS_CLKENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn c2c_ser_pll_ctrl_valid(&self) -> C2C_SER_PLL_CTRL_VALID_R {
        C2C_SER_PLL_CTRL_VALID_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn c2c_par_sel_cka(&self) -> C2C_PAR_SEL_CKA_R {
        C2C_PAR_SEL_CKA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn c2c_par_force_cka(&self) -> C2C_PAR_FORCE_CKA_R {
        C2C_PAR_FORCE_CKA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn c2c_par_force_ckb(&self) -> C2C_PAR_FORCE_CKB_R {
        C2C_PAR_FORCE_CKB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn c2c_par_subsys_clkena(&self) -> C2C_PAR_SUBSYS_CLKENA_R {
        C2C_PAR_SUBSYS_CLKENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn c2c_par_pll_ctrl_valid(&self) -> C2C_PAR_PLL_CTRL_VALID_R {
        C2C_PAR_PLL_CTRL_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_sel_cka(&mut self) -> DSP_SEL_CKA_W<0> {
        DSP_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_force_cka(&mut self) -> DSP_FORCE_CKA_W<1> {
        DSP_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_force_ckb(&mut self) -> DSP_FORCE_CKB_W<2> {
        DSP_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_subsys_clkena(&mut self) -> DSP_SUBSYS_CLKENA_W<3> {
        DSP_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn dsp_pll_ctrl_valid(&mut self) -> DSP_PLL_CTRL_VALID_W<7> {
        DSP_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn icn_sel_cka(&mut self) -> ICN_SEL_CKA_W<8> {
        ICN_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn icn_force_cka(&mut self) -> ICN_FORCE_CKA_W<9> {
        ICN_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn icn_force_ckb(&mut self) -> ICN_FORCE_CKB_W<10> {
        ICN_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn icn_subsys_clkena(&mut self) -> ICN_SUBSYS_CLKENA_W<11> {
        ICN_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn icn_pll_ctrl_valid(&mut self) -> ICN_PLL_CTRL_VALID_W<15> {
        ICN_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_ser_sel_cka(&mut self) -> C2C_SER_SEL_CKA_W<16> {
        C2C_SER_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_ser_force_cka(&mut self) -> C2C_SER_FORCE_CKA_W<17> {
        C2C_SER_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_ser_force_ckb(&mut self) -> C2C_SER_FORCE_CKB_W<18> {
        C2C_SER_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_ser_subsys_clkena(&mut self) -> C2C_SER_SUBSYS_CLKENA_W<19> {
        C2C_SER_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_ser_pll_ctrl_valid(&mut self) -> C2C_SER_PLL_CTRL_VALID_W<23> {
        C2C_SER_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_par_sel_cka(&mut self) -> C2C_PAR_SEL_CKA_W<24> {
        C2C_PAR_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_par_force_cka(&mut self) -> C2C_PAR_FORCE_CKA_W<25> {
        C2C_PAR_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_par_force_ckb(&mut self) -> C2C_PAR_FORCE_CKB_W<26> {
        C2C_PAR_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_par_subsys_clkena(&mut self) -> C2C_PAR_SUBSYS_CLKENA_W<27> {
        C2C_PAR_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_par_pll_ctrl_valid(&mut self) -> C2C_PAR_PLL_CTRL_VALID_W<31> {
        C2C_PAR_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for DSP, Interconnect, C2C Serial and C2C Parallel. *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl2](index.html) module"]
pub struct CLK_CTRL2_SPEC;
impl crate::RegisterSpec for CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl2::R](R) reader structure"]
impl crate::Readable for CLK_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl2::W](W) writer structure"]
impl crate::Writable for CLK_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL2 to value 0"]
impl crate::Resettable for CLK_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
