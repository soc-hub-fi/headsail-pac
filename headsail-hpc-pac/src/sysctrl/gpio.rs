#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "GPIO"]
#[doc(alias = "GPIO")]
pub struct Gpio {
    dir: Dir,
    en: En,
    in_: In,
    out: Out,
    _reserved4: [u8; 0x18],
    pad_cfg_0_3: PadCfg0_3,
    pad_cfg_4_7: PadCfg4_7,
    pad_cfg_8_11: PadCfg8_11,
    pad_cfg_12_15: PadCfg12_15,
}
impl Gpio {
    #[doc = "0x00 - GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x04 - GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x0c - GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pad_cfg_0_3(&self) -> &PadCfg0_3 {
        &self.pad_cfg_0_3
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pad_cfg_4_7(&self) -> &PadCfg4_7 {
        &self.pad_cfg_4_7
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn pad_cfg_8_11(&self) -> &PadCfg8_11 {
        &self.pad_cfg_8_11
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn pad_cfg_12_15(&self) -> &PadCfg12_15 {
        &self.pad_cfg_12_15
    }
}
#[doc = "DIR (rw) register accessor: GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
pub mod dir;
#[doc = "EN (rw) register accessor: GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
pub mod en;
#[doc = "IN (r) register accessor: GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
pub mod in_;
#[doc = "OUT (rw) register accessor: GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
pub mod out;
#[doc = "PAD_CFG_0_3 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg_0_3`]
module"]
#[doc(alias = "PAD_CFG_0_3")]
pub type PadCfg0_3 = crate::Reg<pad_cfg_0_3::PadCfg0_3Spec>;
#[doc = ""]
pub mod pad_cfg_0_3;
#[doc = "PAD_CFG_4_7 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_4_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_4_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg_4_7`]
module"]
#[doc(alias = "PAD_CFG_4_7")]
pub type PadCfg4_7 = crate::Reg<pad_cfg_4_7::PadCfg4_7Spec>;
#[doc = ""]
pub mod pad_cfg_4_7;
#[doc = "PAD_CFG_8_11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_8_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_8_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg_8_11`]
module"]
#[doc(alias = "PAD_CFG_8_11")]
pub type PadCfg8_11 = crate::Reg<pad_cfg_8_11::PadCfg8_11Spec>;
#[doc = ""]
pub mod pad_cfg_8_11;
#[doc = "PAD_CFG_12_15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_cfg_12_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_cfg_12_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_cfg_12_15`]
module"]
#[doc(alias = "PAD_CFG_12_15")]
pub type PadCfg12_15 = crate::Reg<pad_cfg_12_15::PadCfg12_15Spec>;
#[doc = ""]
pub mod pad_cfg_12_15;
