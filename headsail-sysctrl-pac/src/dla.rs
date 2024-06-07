#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: STATUS,
    dla_ctrl: DLA_CTRL,
    buf_ctrl: BUF_CTRL,
    mac_ctrl: MAC_CTRL,
    pp_ptrl: PP_PTRL,
    buf_input: BUF_INPUT,
    buf_kernel0: BUF_KERNEL0,
    buf_kernel1: BUF_KERNEL1,
    buf_pad: BUF_PAD,
    buf_stride: BUF_STRIDE,
    pp_input: PP_INPUT,
    buf_data_bank: BUF_DATA_BANK,
    buf_data_wait_a: BUF_DATA_WAIT_A,
    buf_data_wait_b: BUF_DATA_WAIT_B,
    buf_pipe_stall: BUF_PIPE_STALL,
    mac_data_b_wait: MAC_DATA_B_WAIT,
    mac_pipe_stall: MAC_PIPE_STALL,
    dma_ctrl: DMA_CTRL,
    dma_pad: DMA_PAD,
    power_ctrl: POWER_CTRL,
    power_stat: POWER_STAT,
    mac_sat_max: MAC_SAT_MAX,
    mac_sat_min: MAC_SAT_MIN,
    pp_axi_write: PP_AXI_WRITE,
    pp_axi_read: PP_AXI_READ,
    handshake: HANDSHAKE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn dla_ctrl(&self) -> &DLA_CTRL {
        &self.dla_ctrl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn buf_ctrl(&self) -> &BUF_CTRL {
        &self.buf_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn mac_ctrl(&self) -> &MAC_CTRL {
        &self.mac_ctrl
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn pp_ptrl(&self) -> &PP_PTRL {
        &self.pp_ptrl
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn buf_input(&self) -> &BUF_INPUT {
        &self.buf_input
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn buf_kernel0(&self) -> &BUF_KERNEL0 {
        &self.buf_kernel0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn buf_kernel1(&self) -> &BUF_KERNEL1 {
        &self.buf_kernel1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn buf_pad(&self) -> &BUF_PAD {
        &self.buf_pad
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn buf_stride(&self) -> &BUF_STRIDE {
        &self.buf_stride
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pp_input(&self) -> &PP_INPUT {
        &self.pp_input
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn buf_data_bank(&self) -> &BUF_DATA_BANK {
        &self.buf_data_bank
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn buf_data_wait_a(&self) -> &BUF_DATA_WAIT_A {
        &self.buf_data_wait_a
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn buf_data_wait_b(&self) -> &BUF_DATA_WAIT_B {
        &self.buf_data_wait_b
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn buf_pipe_stall(&self) -> &BUF_PIPE_STALL {
        &self.buf_pipe_stall
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn mac_data_b_wait(&self) -> &MAC_DATA_B_WAIT {
        &self.mac_data_b_wait
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn mac_pipe_stall(&self) -> &MAC_PIPE_STALL {
        &self.mac_pipe_stall
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn dma_ctrl(&self) -> &DMA_CTRL {
        &self.dma_ctrl
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn dma_pad(&self) -> &DMA_PAD {
        &self.dma_pad
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn power_ctrl(&self) -> &POWER_CTRL {
        &self.power_ctrl
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn power_stat(&self) -> &POWER_STAT {
        &self.power_stat
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn mac_sat_max(&self) -> &MAC_SAT_MAX {
        &self.mac_sat_max
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn mac_sat_min(&self) -> &MAC_SAT_MIN {
        &self.mac_sat_min
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn pp_axi_write(&self) -> &PP_AXI_WRITE {
        &self.pp_axi_write
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn pp_axi_read(&self) -> &PP_AXI_READ {
        &self.pp_axi_read
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn handshake(&self) -> &HANDSHAKE {
        &self.handshake
    }
}
#[doc = "status (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "dla_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dla_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dla_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dla_ctrl`]
module"]
#[doc(alias = "dla_ctrl")]
pub type DLA_CTRL = crate::Reg<dla_ctrl::DLA_CTRL_SPEC>;
#[doc = ""]
pub mod dla_ctrl;
#[doc = "mac_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_ctrl`]
module"]
#[doc(alias = "mac_ctrl")]
pub type MAC_CTRL = crate::Reg<mac_ctrl::MAC_CTRL_SPEC>;
#[doc = ""]
pub mod mac_ctrl;
#[doc = "buf_input (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_input`]
module"]
#[doc(alias = "buf_input")]
pub type BUF_INPUT = crate::Reg<buf_input::BUF_INPUT_SPEC>;
#[doc = ""]
pub mod buf_input;
#[doc = "buf_kernel0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_kernel0`]
module"]
#[doc(alias = "buf_kernel0")]
pub type BUF_KERNEL0 = crate::Reg<buf_kernel0::BUF_KERNEL0_SPEC>;
#[doc = ""]
pub mod buf_kernel0;
#[doc = "buf_kernel1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_kernel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_kernel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_kernel1`]
module"]
#[doc(alias = "buf_kernel1")]
pub type BUF_KERNEL1 = crate::Reg<buf_kernel1::BUF_KERNEL1_SPEC>;
#[doc = ""]
pub mod buf_kernel1;
#[doc = "buf_pad (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_pad`]
module"]
#[doc(alias = "buf_pad")]
pub type BUF_PAD = crate::Reg<buf_pad::BUF_PAD_SPEC>;
#[doc = ""]
pub mod buf_pad;
#[doc = "pp_input (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_input`]
module"]
#[doc(alias = "pp_input")]
pub type PP_INPUT = crate::Reg<pp_input::PP_INPUT_SPEC>;
#[doc = ""]
pub mod pp_input;
#[doc = "buf_stride (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_stride`]
module"]
#[doc(alias = "buf_stride")]
pub type BUF_STRIDE = crate::Reg<buf_stride::BUF_STRIDE_SPEC>;
#[doc = ""]
pub mod buf_stride;
#[doc = "buf_data_bank (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_bank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_bank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_bank`]
module"]
#[doc(alias = "buf_data_bank")]
pub type BUF_DATA_BANK = crate::Reg<buf_data_bank::BUF_DATA_BANK_SPEC>;
#[doc = ""]
pub mod buf_data_bank;
#[doc = "buf_data_wait_b (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_b::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_wait_b`]
module"]
#[doc(alias = "buf_data_wait_b")]
pub type BUF_DATA_WAIT_B = crate::Reg<buf_data_wait_b::BUF_DATA_WAIT_B_SPEC>;
#[doc = ""]
pub mod buf_data_wait_b;
#[doc = "buf_data_wait_a (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_wait_a::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_wait_a`]
module"]
#[doc(alias = "buf_data_wait_a")]
pub type BUF_DATA_WAIT_A = crate::Reg<buf_data_wait_a::BUF_DATA_WAIT_A_SPEC>;
#[doc = ""]
pub mod buf_data_wait_a;
#[doc = "pp_axi_read (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_axi_read`]
module"]
#[doc(alias = "pp_axi_read")]
pub type PP_AXI_READ = crate::Reg<pp_axi_read::PP_AXI_READ_SPEC>;
#[doc = ""]
pub mod pp_axi_read;
#[doc = "pp_axi_write (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_axi_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_axi_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_axi_write`]
module"]
#[doc(alias = "pp_axi_write")]
pub type PP_AXI_WRITE = crate::Reg<pp_axi_write::PP_AXI_WRITE_SPEC>;
#[doc = ""]
pub mod pp_axi_write;
#[doc = "power_stat (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_stat`]
module"]
#[doc(alias = "power_stat")]
pub type POWER_STAT = crate::Reg<power_stat::POWER_STAT_SPEC>;
#[doc = ""]
pub mod power_stat;
#[doc = "power_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctrl`]
module"]
#[doc(alias = "power_ctrl")]
pub type POWER_CTRL = crate::Reg<power_ctrl::POWER_CTRL_SPEC>;
#[doc = ""]
pub mod power_ctrl;
#[doc = "mac_sat_min (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_min::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_sat_min`]
module"]
#[doc(alias = "mac_sat_min")]
pub type MAC_SAT_MIN = crate::Reg<mac_sat_min::MAC_SAT_MIN_SPEC>;
#[doc = ""]
pub mod mac_sat_min;
#[doc = "mac_sat_max (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_sat_max::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_sat_max`]
module"]
#[doc(alias = "mac_sat_max")]
pub type MAC_SAT_MAX = crate::Reg<mac_sat_max::MAC_SAT_MAX_SPEC>;
#[doc = ""]
pub mod mac_sat_max;
#[doc = "dma_pad (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_pad::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_pad`]
module"]
#[doc(alias = "dma_pad")]
pub type DMA_PAD = crate::Reg<dma_pad::DMA_PAD_SPEC>;
#[doc = ""]
pub mod dma_pad;
#[doc = "dma_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ctrl`]
module"]
#[doc(alias = "dma_ctrl")]
pub type DMA_CTRL = crate::Reg<dma_ctrl::DMA_CTRL_SPEC>;
#[doc = ""]
pub mod dma_ctrl;
#[doc = "buf_pipe_stall (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_pipe_stall::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_pipe_stall`]
module"]
#[doc(alias = "buf_pipe_stall")]
pub type BUF_PIPE_STALL = crate::Reg<buf_pipe_stall::BUF_PIPE_STALL_SPEC>;
#[doc = ""]
pub mod buf_pipe_stall;
#[doc = "buf_ctrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_ctrl`]
module"]
#[doc(alias = "buf_ctrl")]
pub type BUF_CTRL = crate::Reg<buf_ctrl::BUF_CTRL_SPEC>;
#[doc = ""]
pub mod buf_ctrl;
#[doc = "handshake (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handshake::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handshake::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@handshake`]
module"]
#[doc(alias = "handshake")]
pub type HANDSHAKE = crate::Reg<handshake::HANDSHAKE_SPEC>;
#[doc = ""]
pub mod handshake;
#[doc = "pp_ptrl (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp_ptrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp_ptrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp_ptrl`]
module"]
#[doc(alias = "pp_ptrl")]
pub type PP_PTRL = crate::Reg<pp_ptrl::PP_PTRL_SPEC>;
#[doc = ""]
pub mod pp_ptrl;
#[doc = "mac_data_b_wait (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_data_b_wait::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_data_b_wait`]
module"]
#[doc(alias = "mac_data_b_wait")]
pub type MAC_DATA_B_WAIT = crate::Reg<mac_data_b_wait::MAC_DATA_B_WAIT_SPEC>;
#[doc = ""]
pub mod mac_data_b_wait;
#[doc = "mac_pipe_stall (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_pipe_stall::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_pipe_stall`]
module"]
#[doc(alias = "mac_pipe_stall")]
pub type MAC_PIPE_STALL = crate::Reg<mac_pipe_stall::MAC_PIPE_STALL_SPEC>;
#[doc = ""]
pub mod mac_pipe_stall;
