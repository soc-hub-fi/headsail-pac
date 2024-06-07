#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "SDIO"]
#[doc(alias = "SDIO")]
pub struct Sdio {
    _reserved0: [u8; 0x20],
    cmd_op: CmdOp,
    cmd_arg: CmdArg,
    data_setup: DataSetup,
    start: Start,
    rsp0: Rsp0,
    rsp1: Rsp1,
    rsp2: Rsp2,
    rsp3: Rsp3,
    clk_div_0: ClkDiv0,
    status: Status,
    cid0: Cid0,
    cid1: Cid1,
    cid2: Cid2,
    cid3: Cid3,
    rca: Rca,
    stop: Stop,
}
impl Sdio {
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn cmd_op(&self) -> &CmdOp {
        &self.cmd_op
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn cmd_arg(&self) -> &CmdArg {
        &self.cmd_arg
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn data_setup(&self) -> &DataSetup {
        &self.data_setup
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn rsp0(&self) -> &Rsp0 {
        &self.rsp0
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn rsp1(&self) -> &Rsp1 {
        &self.rsp1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn rsp2(&self) -> &Rsp2 {
        &self.rsp2
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn rsp3(&self) -> &Rsp3 {
        &self.rsp3
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn clk_div_0(&self) -> &ClkDiv0 {
        &self.clk_div_0
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x48 - Card Identification Word0"]
    #[inline(always)]
    pub const fn cid0(&self) -> &Cid0 {
        &self.cid0
    }
    #[doc = "0x4c - Card Identification Word 1"]
    #[inline(always)]
    pub const fn cid1(&self) -> &Cid1 {
        &self.cid1
    }
    #[doc = "0x50 - Card Identification Word 2"]
    #[inline(always)]
    pub const fn cid2(&self) -> &Cid2 {
        &self.cid2
    }
    #[doc = "0x54 - Card Identification Word 3"]
    #[inline(always)]
    pub const fn cid3(&self) -> &Cid3 {
        &self.cid3
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn rca(&self) -> &Rca {
        &self.rca
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
}
#[doc = "CMD_OP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_op::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_op`]
module"]
#[doc(alias = "CMD_OP")]
pub type CmdOp = crate::Reg<cmd_op::CmdOpSpec>;
#[doc = ""]
pub mod cmd_op;
#[doc = "CMD_ARG (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_arg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_arg`]
module"]
#[doc(alias = "CMD_ARG")]
pub type CmdArg = crate::Reg<cmd_arg::CmdArgSpec>;
#[doc = ""]
pub mod cmd_arg;
#[doc = "DATA_SETUP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_setup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_setup`]
module"]
#[doc(alias = "DATA_SETUP")]
pub type DataSetup = crate::Reg<data_setup::DataSetupSpec>;
#[doc = ""]
pub mod data_setup;
#[doc = "START (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = ""]
pub mod start;
#[doc = "STOP (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = ""]
pub mod stop;
#[doc = "RSP0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp0`]
module"]
#[doc(alias = "RSP0")]
pub type Rsp0 = crate::Reg<rsp0::Rsp0Spec>;
#[doc = ""]
pub mod rsp0;
#[doc = "CID3 (r) register accessor: Card Identification Word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
#[doc(alias = "CID3")]
pub type Cid3 = crate::Reg<cid3::Cid3Spec>;
#[doc = "Card Identification Word 3"]
pub mod cid3;
#[doc = "CID1 (r) register accessor: Card Identification Word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
#[doc(alias = "CID1")]
pub type Cid1 = crate::Reg<cid1::Cid1Spec>;
#[doc = "Card Identification Word 1"]
pub mod cid1;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = ""]
pub mod status;
#[doc = "RSP3 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp3`]
module"]
#[doc(alias = "RSP3")]
pub type Rsp3 = crate::Reg<rsp3::Rsp3Spec>;
#[doc = ""]
pub mod rsp3;
#[doc = "RSP1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp1`]
module"]
#[doc(alias = "RSP1")]
pub type Rsp1 = crate::Reg<rsp1::Rsp1Spec>;
#[doc = ""]
pub mod rsp1;
#[doc = "RCA (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rca::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rca`]
module"]
#[doc(alias = "RCA")]
pub type Rca = crate::Reg<rca::RcaSpec>;
#[doc = ""]
pub mod rca;
#[doc = "CID2 (r) register accessor: Card Identification Word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
#[doc(alias = "CID2")]
pub type Cid2 = crate::Reg<cid2::Cid2Spec>;
#[doc = "Card Identification Word 2"]
pub mod cid2;
#[doc = "CID0 (r) register accessor: Card Identification Word0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
#[doc(alias = "CID0")]
pub type Cid0 = crate::Reg<cid0::Cid0Spec>;
#[doc = "Card Identification Word0"]
pub mod cid0;
#[doc = "CLK_DIV_0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_div_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_div_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_div_0`]
module"]
#[doc(alias = "CLK_DIV_0")]
pub type ClkDiv0 = crate::Reg<clk_div_0::ClkDiv0Spec>;
#[doc = ""]
pub mod clk_div_0;
#[doc = "RSP2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp2`]
module"]
#[doc(alias = "RSP2")]
pub type Rsp2 = crate::Reg<rsp2::Rsp2Spec>;
#[doc = ""]
pub mod rsp2;
