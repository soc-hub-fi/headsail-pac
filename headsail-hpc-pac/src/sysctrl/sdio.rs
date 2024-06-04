#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "SDIO"]
pub struct SDIO {
    _reserved0: [u8; 0x20],
    cmd_op: CMD_OP,
    cmd_arg: CMD_ARG,
    data_setup: DATA_SETUP,
    start: START,
    rsp0: RSP0,
    rsp1: RSP1,
    rsp2: RSP2,
    rsp3: RSP3,
    clk_div_0: CLK_DIV_0,
    status: STATUS,
    cid0: CID0,
    cid1: CID1,
    cid2: CID2,
    cid3: CID3,
    rca: RCA,
    stop: STOP,
}
impl SDIO {
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cmd_op(&self) -> &CMD_OP {
        &self.cmd_op
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cmd_arg(&self) -> &CMD_ARG {
        &self.cmd_arg
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn data_setup(&self) -> &DATA_SETUP {
        &self.data_setup
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn rsp0(&self) -> &RSP0 {
        &self.rsp0
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn rsp1(&self) -> &RSP1 {
        &self.rsp1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn rsp2(&self) -> &RSP2 {
        &self.rsp2
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn rsp3(&self) -> &RSP3 {
        &self.rsp3
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn clk_div_0(&self) -> &CLK_DIV_0 {
        &self.clk_div_0
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x48 - Card Identification Word0"]
    #[inline(always)]
    pub const fn cid0(&self) -> &CID0 {
        &self.cid0
    }
    #[doc = "0x4c - Card Identification Word 1"]
    #[inline(always)]
    pub const fn cid1(&self) -> &CID1 {
        &self.cid1
    }
    #[doc = "0x50 - Card Identification Word 2"]
    #[inline(always)]
    pub const fn cid2(&self) -> &CID2 {
        &self.cid2
    }
    #[doc = "0x54 - Card Identification Word 3"]
    #[inline(always)]
    pub const fn cid3(&self) -> &CID3 {
        &self.cid3
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn rca(&self) -> &RCA {
        &self.rca
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn stop(&self) -> &STOP {
        &self.stop
    }
}
#[doc = "CMD_OP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_op::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_op`]
module"]
pub type CMD_OP = crate::Reg<cmd_op::CMD_OP_SPEC>;
#[doc = ""]
pub mod cmd_op;
#[doc = "CMD_ARG (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_arg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_arg`]
module"]
pub type CMD_ARG = crate::Reg<cmd_arg::CMD_ARG_SPEC>;
#[doc = ""]
pub mod cmd_arg;
#[doc = "DATA_SETUP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_setup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_setup`]
module"]
pub type DATA_SETUP = crate::Reg<data_setup::DATA_SETUP_SPEC>;
#[doc = ""]
pub mod data_setup;
#[doc = "START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "STOP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = ""]
pub mod stop;
#[doc = "RSP0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp0`]
module"]
pub type RSP0 = crate::Reg<rsp0::RSP0_SPEC>;
#[doc = ""]
pub mod rsp0;
#[doc = "CID3 (r) register accessor: Card Identification Word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Card Identification Word 3"]
pub mod cid3;
#[doc = "CID1 (r) register accessor: Card Identification Word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Card Identification Word 1"]
pub mod cid1;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "RSP3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp3`]
module"]
pub type RSP3 = crate::Reg<rsp3::RSP3_SPEC>;
#[doc = ""]
pub mod rsp3;
#[doc = "RSP1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp1`]
module"]
pub type RSP1 = crate::Reg<rsp1::RSP1_SPEC>;
#[doc = ""]
pub mod rsp1;
#[doc = "RCA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rca::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rca`]
module"]
pub type RCA = crate::Reg<rca::RCA_SPEC>;
#[doc = ""]
pub mod rca;
#[doc = "CID2 (r) register accessor: Card Identification Word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Card Identification Word 2"]
pub mod cid2;
#[doc = "CID0 (r) register accessor: Card Identification Word0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Card Identification Word0"]
pub mod cid0;
#[doc = "CLK_DIV_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_div_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_div_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_div_0`]
module"]
pub type CLK_DIV_0 = crate::Reg<clk_div_0::CLK_DIV_0_SPEC>;
#[doc = ""]
pub mod clk_div_0;
#[doc = "RSP2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp2`]
module"]
pub type RSP2 = crate::Reg<rsp2::RSP2_SPEC>;
#[doc = ""]
pub mod rsp2;
