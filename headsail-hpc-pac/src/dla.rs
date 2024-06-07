#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    dla_ctrl: DlaCtrl,
    buf_ctrl: BufCtrl,
    mac_ctrl: MacCtrl,
    pp_ptrl: PpPtrl,
    buf_input: BufInput,
    buf_kernel0: BufKernel0,
    buf_kernel1: BufKernel1,
    buf_pad: BufPad,
    buf_stride: BufStride,
    pp_input: PpInput,
    buf_data_bank: BufDataBank,
    buf_data_wait_a: BufDataWaitA,
    buf_data_wait_b: BufDataWaitB,
    buf_pipe_stall: BufPipeStall,
    mac_data_b_wait: MacDataBWait,
    mac_pipe_stall: MacPipeStall,
    dma_ctrl: DmaCtrl,
    dma_pad: DmaPad,
    power_ctrl: PowerCtrl,
    power_stat: PowerStat,
    mac_sat_max: MacSatMax,
    mac_sat_min: MacSatMin,
    pp_axi_write: PpAxiWrite,
    pp_axi_read: PpAxiRead,
    handshake: Handshake,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dla_ctrl(&self) -> &DlaCtrl {
        &self.dla_ctrl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn buf_ctrl(&self) -> &BufCtrl {
        &self.buf_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn mac_ctrl(&self) -> &MacCtrl {
        &self.mac_ctrl
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn pp_ptrl(&self) -> &PpPtrl {
        &self.pp_ptrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn buf_input(&self) -> &BufInput {
        &self.buf_input
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn buf_kernel0(&self) -> &BufKernel0 {
        &self.buf_kernel0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn buf_kernel1(&self) -> &BufKernel1 {
        &self.buf_kernel1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn buf_pad(&self) -> &BufPad {
        &self.buf_pad
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn buf_stride(&self) -> &BufStride {
        &self.buf_stride
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pp_input(&self) -> &PpInput {
        &self.pp_input
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn buf_data_bank(&self) -> &BufDataBank {
        &self.buf_data_bank
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn buf_data_wait_a(&self) -> &BufDataWaitA {
        &self.buf_data_wait_a
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn buf_data_wait_b(&self) -> &BufDataWaitB {
        &self.buf_data_wait_b
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn buf_pipe_stall(&self) -> &BufPipeStall {
        &self.buf_pipe_stall
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn mac_data_b_wait(&self) -> &MacDataBWait {
        &self.mac_data_b_wait
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn mac_pipe_stall(&self) -> &MacPipeStall {
        &self.mac_pipe_stall
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn dma_ctrl(&self) -> &DmaCtrl {
        &self.dma_ctrl
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn dma_pad(&self) -> &DmaPad {
        &self.dma_pad
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn power_ctrl(&self) -> &PowerCtrl {
        &self.power_ctrl
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn power_stat(&self) -> &PowerStat {
        &self.power_stat
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn mac_sat_max(&self) -> &MacSatMax {
        &self.mac_sat_max
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn mac_sat_min(&self) -> &MacSatMin {
        &self.mac_sat_min
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn pp_axi_write(&self) -> &PpAxiWrite {
        &self.pp_axi_write
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn pp_axi_read(&self) -> &PpAxiRead {
        &self.pp_axi_read
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn handshake(&self) -> &Handshake {
        &self.handshake
    }
}
#[doc = "status (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = ""]
pub mod status;
#[doc = "dla_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_ctrl`]
module"]
#[doc(alias = "dla_ctrl")]
pub type DlaCtrl = crate::Reg<dla_ctrl::DlaCtrlSpec>;
#[doc = ""]
pub mod dla_ctrl;
#[doc = "mac_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_ctrl`]
module"]
#[doc(alias = "mac_ctrl")]
pub type MacCtrl = crate::Reg<mac_ctrl::MacCtrlSpec>;
#[doc = ""]
pub mod mac_ctrl;
#[doc = "buf_input (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_input`]
module"]
#[doc(alias = "buf_input")]
pub type BufInput = crate::Reg<buf_input::BufInputSpec>;
#[doc = ""]
pub mod buf_input;
#[doc = "buf_kernel0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_kernel0`]
module"]
#[doc(alias = "buf_kernel0")]
pub type BufKernel0 = crate::Reg<buf_kernel0::BufKernel0Spec>;
#[doc = ""]
pub mod buf_kernel0;
#[doc = "buf_kernel1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_kernel1`]
module"]
#[doc(alias = "buf_kernel1")]
pub type BufKernel1 = crate::Reg<buf_kernel1::BufKernel1Spec>;
#[doc = ""]
pub mod buf_kernel1;
#[doc = "buf_pad (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_pad`]
module"]
#[doc(alias = "buf_pad")]
pub type BufPad = crate::Reg<buf_pad::BufPadSpec>;
#[doc = ""]
pub mod buf_pad;
#[doc = "pp_input (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_input`]
module"]
#[doc(alias = "pp_input")]
pub type PpInput = crate::Reg<pp_input::PpInputSpec>;
#[doc = ""]
pub mod pp_input;
#[doc = "buf_stride (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_stride`]
module"]
#[doc(alias = "buf_stride")]
pub type BufStride = crate::Reg<buf_stride::BufStrideSpec>;
#[doc = ""]
pub mod buf_stride;
#[doc = "buf_data_bank (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_bank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_bank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_bank`]
module"]
#[doc(alias = "buf_data_bank")]
pub type BufDataBank = crate::Reg<buf_data_bank::BufDataBankSpec>;
#[doc = ""]
pub mod buf_data_bank;
#[doc = "buf_data_wait_b (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_wait_b`]
module"]
#[doc(alias = "buf_data_wait_b")]
pub type BufDataWaitB = crate::Reg<buf_data_wait_b::BufDataWaitBSpec>;
#[doc = ""]
pub mod buf_data_wait_b;
#[doc = "buf_data_wait_a (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_wait_a`]
module"]
#[doc(alias = "buf_data_wait_a")]
pub type BufDataWaitA = crate::Reg<buf_data_wait_a::BufDataWaitASpec>;
#[doc = ""]
pub mod buf_data_wait_a;
#[doc = "pp_axi_read (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_axi_read`]
module"]
#[doc(alias = "pp_axi_read")]
pub type PpAxiRead = crate::Reg<pp_axi_read::PpAxiReadSpec>;
#[doc = ""]
pub mod pp_axi_read;
#[doc = "pp_axi_write (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_axi_write`]
module"]
#[doc(alias = "pp_axi_write")]
pub type PpAxiWrite = crate::Reg<pp_axi_write::PpAxiWriteSpec>;
#[doc = ""]
pub mod pp_axi_write;
#[doc = "power_stat (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_stat`]
module"]
#[doc(alias = "power_stat")]
pub type PowerStat = crate::Reg<power_stat::PowerStatSpec>;
#[doc = ""]
pub mod power_stat;
#[doc = "power_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctrl`]
module"]
#[doc(alias = "power_ctrl")]
pub type PowerCtrl = crate::Reg<power_ctrl::PowerCtrlSpec>;
#[doc = ""]
pub mod power_ctrl;
#[doc = "mac_sat_min (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_min::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_sat_min`]
module"]
#[doc(alias = "mac_sat_min")]
pub type MacSatMin = crate::Reg<mac_sat_min::MacSatMinSpec>;
#[doc = ""]
pub mod mac_sat_min;
#[doc = "mac_sat_max (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_max::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_sat_max`]
module"]
#[doc(alias = "mac_sat_max")]
pub type MacSatMax = crate::Reg<mac_sat_max::MacSatMaxSpec>;
#[doc = ""]
pub mod mac_sat_max;
#[doc = "dma_pad (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_pad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_pad`]
module"]
#[doc(alias = "dma_pad")]
pub type DmaPad = crate::Reg<dma_pad::DmaPadSpec>;
#[doc = ""]
pub mod dma_pad;
#[doc = "dma_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ctrl`]
module"]
#[doc(alias = "dma_ctrl")]
pub type DmaCtrl = crate::Reg<dma_ctrl::DmaCtrlSpec>;
#[doc = ""]
pub mod dma_ctrl;
#[doc = "buf_pipe_stall (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pipe_stall::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_pipe_stall`]
module"]
#[doc(alias = "buf_pipe_stall")]
pub type BufPipeStall = crate::Reg<buf_pipe_stall::BufPipeStallSpec>;
#[doc = ""]
pub mod buf_pipe_stall;
#[doc = "buf_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_ctrl`]
module"]
#[doc(alias = "buf_ctrl")]
pub type BufCtrl = crate::Reg<buf_ctrl::BufCtrlSpec>;
#[doc = ""]
pub mod buf_ctrl;
#[doc = "handshake (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handshake::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handshake::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@handshake`]
module"]
#[doc(alias = "handshake")]
pub type Handshake = crate::Reg<handshake::HandshakeSpec>;
#[doc = ""]
pub mod handshake;
#[doc = "pp_ptrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_ptrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_ptrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_ptrl`]
module"]
#[doc(alias = "pp_ptrl")]
pub type PpPtrl = crate::Reg<pp_ptrl::PpPtrlSpec>;
#[doc = ""]
pub mod pp_ptrl;
#[doc = "mac_data_b_wait (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_data_b_wait::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_data_b_wait`]
module"]
#[doc(alias = "mac_data_b_wait")]
pub type MacDataBWait = crate::Reg<mac_data_b_wait::MacDataBWaitSpec>;
#[doc = ""]
pub mod mac_data_b_wait;
#[doc = "mac_pipe_stall (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_pipe_stall::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_pipe_stall`]
module"]
#[doc(alias = "mac_pipe_stall")]
pub type MacPipeStall = crate::Reg<mac_pipe_stall::MacPipeStallSpec>;
#[doc = ""]
pub mod mac_pipe_stall;
