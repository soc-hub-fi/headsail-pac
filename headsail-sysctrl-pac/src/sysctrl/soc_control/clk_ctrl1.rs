#[doc = "Register `CLK_CTRL1` reader"]
pub struct R(crate::R<CLK_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL1` writer"]
pub struct W(crate::W<CLK_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL1_SPEC>;
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
impl From<crate::W<CLK_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETH_sel_cka` reader - Select CKA"]
pub type ETH_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ETH_sel_cka` writer - Select CKA"]
pub type ETH_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ETH_force_cka` reader - Force CKA"]
pub type ETH_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `ETH_force_cka` writer - Force CKA"]
pub type ETH_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ETH_force_ckb` reader - Force CKB"]
pub type ETH_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `ETH_force_ckb` writer - Force CKB"]
pub type ETH_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ETH_subsys_clkena` reader - Subsystem clock enable"]
pub type ETH_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `ETH_subsys_clkena` writer - Subsystem clock enable"]
pub type ETH_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `ETH_pll_ctrl_valid` reader - PLL Control valid"]
pub type ETH_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `ETH_pll_ctrl_valid` writer - PLL Control valid"]
pub type ETH_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DLA_sel_cka` reader - Select CKA"]
pub type DLA_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DLA_sel_cka` writer - Select CKA"]
pub type DLA_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DLA_force_cka` reader - Force CKA"]
pub type DLA_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DLA_force_cka` writer - Force CKA"]
pub type DLA_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DLA_force_ckb` reader - Force CKB"]
pub type DLA_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `DLA_force_ckb` writer - Force CKB"]
pub type DLA_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DLA_subsys_clkena` reader - Subsystem clock enable"]
pub type DLA_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `DLA_subsys_clkena` writer - Subsystem clock enable"]
pub type DLA_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DLA_pll_ctrl_valid` reader - PLL Control valid"]
pub type DLA_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DLA_pll_ctrl_valid` writer - PLL Control valid"]
pub type DLA_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `HPC_sel_cka` reader - Select CKA"]
pub type HPC_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `HPC_sel_cka` writer - Select CKA"]
pub type HPC_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `HPC_force_cka` reader - Force CKA"]
pub type HPC_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `HPC_force_cka` writer - Force CKA"]
pub type HPC_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `HPC_force_ckb` reader - Force CKB"]
pub type HPC_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `HPC_force_ckb` writer - Force CKB"]
pub type HPC_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `HPC_subsys_clkena` reader - Subsystem clock enable"]
pub type HPC_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `HPC_subsys_clkena` writer - Subsystem clock enable"]
pub type HPC_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `HPC_pll_ctrl_valid` reader - PLL Control valid"]
pub type HPC_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `HPC_pll_ctrl_valid` writer - PLL Control valid"]
pub type HPC_PLL_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DDR2_sel_cka` reader - Select CKA"]
pub type DDR2_SEL_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DDR2_sel_cka` writer - Select CKA"]
pub type DDR2_SEL_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DDR2_force_cka` reader - Force CKA"]
pub type DDR2_FORCE_CKA_R = crate::BitReader<bool>;
#[doc = "Field `DDR2_force_cka` writer - Force CKA"]
pub type DDR2_FORCE_CKA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DDR2_force_ckb` reader - Force CKB"]
pub type DDR2_FORCE_CKB_R = crate::BitReader<bool>;
#[doc = "Field `DDR2_force_ckb` writer - Force CKB"]
pub type DDR2_FORCE_CKB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DDR2_subsys_clkena` reader - Subsystem clock enable"]
pub type DDR2_SUBSYS_CLKENA_R = crate::BitReader<bool>;
#[doc = "Field `DDR2_subsys_clkena` writer - Subsystem clock enable"]
pub type DDR2_SUBSYS_CLKENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
#[doc = "Field `DDR2_pll_ctrl_valid` reader - PLL Control valid"]
pub type DDR2_PLL_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DDR2_pll_ctrl_valid` writer - PLL Control valid"]
pub type DDR2_PLL_CTRL_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn eth_sel_cka(&self) -> ETH_SEL_CKA_R {
        ETH_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn eth_force_cka(&self) -> ETH_FORCE_CKA_R {
        ETH_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn eth_force_ckb(&self) -> ETH_FORCE_CKB_R {
        ETH_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn eth_subsys_clkena(&self) -> ETH_SUBSYS_CLKENA_R {
        ETH_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn eth_pll_ctrl_valid(&self) -> ETH_PLL_CTRL_VALID_R {
        ETH_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    pub fn dla_sel_cka(&self) -> DLA_SEL_CKA_R {
        DLA_SEL_CKA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    pub fn dla_force_cka(&self) -> DLA_FORCE_CKA_R {
        DLA_FORCE_CKA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    pub fn dla_force_ckb(&self) -> DLA_FORCE_CKB_R {
        DLA_FORCE_CKB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    pub fn dla_subsys_clkena(&self) -> DLA_SUBSYS_CLKENA_R {
        DLA_SUBSYS_CLKENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    pub fn dla_pll_ctrl_valid(&self) -> DLA_PLL_CTRL_VALID_R {
        DLA_PLL_CTRL_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    pub fn hpc_sel_cka(&self) -> HPC_SEL_CKA_R {
        HPC_SEL_CKA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    pub fn hpc_force_cka(&self) -> HPC_FORCE_CKA_R {
        HPC_FORCE_CKA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    pub fn hpc_force_ckb(&self) -> HPC_FORCE_CKB_R {
        HPC_FORCE_CKB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    pub fn hpc_subsys_clkena(&self) -> HPC_SUBSYS_CLKENA_R {
        HPC_SUBSYS_CLKENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    pub fn hpc_pll_ctrl_valid(&self) -> HPC_PLL_CTRL_VALID_R {
        HPC_PLL_CTRL_VALID_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    pub fn ddr2_sel_cka(&self) -> DDR2_SEL_CKA_R {
        DDR2_SEL_CKA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    pub fn ddr2_force_cka(&self) -> DDR2_FORCE_CKA_R {
        DDR2_FORCE_CKA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    pub fn ddr2_force_ckb(&self) -> DDR2_FORCE_CKB_R {
        DDR2_FORCE_CKB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    pub fn ddr2_subsys_clkena(&self) -> DDR2_SUBSYS_CLKENA_R {
        DDR2_SUBSYS_CLKENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    pub fn ddr2_pll_ctrl_valid(&self) -> DDR2_PLL_CTRL_VALID_R {
        DDR2_PLL_CTRL_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn eth_sel_cka(&mut self) -> ETH_SEL_CKA_W<0> {
        ETH_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn eth_force_cka(&mut self) -> ETH_FORCE_CKA_W<1> {
        ETH_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn eth_force_ckb(&mut self) -> ETH_FORCE_CKB_W<2> {
        ETH_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth_subsys_clkena(&mut self) -> ETH_SUBSYS_CLKENA_W<3> {
        ETH_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pll_ctrl_valid(&mut self) -> ETH_PLL_CTRL_VALID_W<7> {
        ETH_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 8 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn dla_sel_cka(&mut self) -> DLA_SEL_CKA_W<8> {
        DLA_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 9 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn dla_force_cka(&mut self) -> DLA_FORCE_CKA_W<9> {
        DLA_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 10 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn dla_force_ckb(&mut self) -> DLA_FORCE_CKB_W<10> {
        DLA_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 11 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dla_subsys_clkena(&mut self) -> DLA_SUBSYS_CLKENA_W<11> {
        DLA_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 15 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn dla_pll_ctrl_valid(&mut self) -> DLA_PLL_CTRL_VALID_W<15> {
        DLA_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 16 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_sel_cka(&mut self) -> HPC_SEL_CKA_W<16> {
        HPC_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 17 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_force_cka(&mut self) -> HPC_FORCE_CKA_W<17> {
        HPC_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 18 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_force_ckb(&mut self) -> HPC_FORCE_CKB_W<18> {
        HPC_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 19 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_subsys_clkena(&mut self) -> HPC_SUBSYS_CLKENA_W<19> {
        HPC_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 23 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_pll_ctrl_valid(&mut self) -> HPC_PLL_CTRL_VALID_W<23> {
        HPC_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Bit 24 - Select CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2_sel_cka(&mut self) -> DDR2_SEL_CKA_W<24> {
        DDR2_SEL_CKA_W::new(self)
    }
    #[doc = "Bit 25 - Force CKA"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2_force_cka(&mut self) -> DDR2_FORCE_CKA_W<25> {
        DDR2_FORCE_CKA_W::new(self)
    }
    #[doc = "Bit 26 - Force CKB"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2_force_ckb(&mut self) -> DDR2_FORCE_CKB_W<26> {
        DDR2_FORCE_CKB_W::new(self)
    }
    #[doc = "Bit 27 - Subsystem clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2_subsys_clkena(&mut self) -> DDR2_SUBSYS_CLKENA_W<27> {
        DDR2_SUBSYS_CLKENA_W::new(self)
    }
    #[doc = "Bit 31 - PLL Control valid"]
    #[inline(always)]
    #[must_use]
    pub fn ddr2_pll_ctrl_valid(&mut self) -> DDR2_PLL_CTRL_VALID_W<31> {
        DDR2_PLL_CTRL_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for Ethernet, DLA; HPC, DDR2 *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl1](index.html) module"]
pub struct CLK_CTRL1_SPEC;
impl crate::RegisterSpec for CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl1::R](R) reader structure"]
impl crate::Readable for CLK_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl1::W](W) writer structure"]
impl crate::Writable for CLK_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CTRL1 to value 0"]
impl crate::Resettable for CLK_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
