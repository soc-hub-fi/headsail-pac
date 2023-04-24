#[doc = r"Register block"]
#[repr(C)]
pub struct CTRL {
    #[doc = "0x00 - "]
    pub status: STATUS,
    #[doc = "0x04 - "]
    pub dla_ctrl: DLA_CTRL,
    #[doc = "0x08 - "]
    pub buf_ctrl: BUF_CTRL,
    #[doc = "0x0c - "]
    pub mac_ctrl: MAC_CTRL,
    #[doc = "0x10 - "]
    pub pp_ptrl: PP_PTRL,
    #[doc = "0x14 - "]
    pub buf_input: BUF_INPUT,
    #[doc = "0x18 - "]
    pub buf_kernel0: BUF_KERNEL0,
    #[doc = "0x1c - "]
    pub buf_kernel1: BUF_KERNEL1,
    #[doc = "0x20 - "]
    pub buf_pad: BUF_PAD,
    #[doc = "0x24 - "]
    pub buf_stride: BUF_STRIDE,
    #[doc = "0x28 - "]
    pub pp_input: PP_INPUT,
    #[doc = "0x2c - "]
    pub buf_data_bank: BUF_DATA_BANK,
    #[doc = "0x30 - "]
    pub buf_data_wait_a: BUF_DATA_WAIT_A,
    #[doc = "0x34 - "]
    pub buf_data_wait_b: BUF_DATA_WAIT_B,
    #[doc = "0x38 - "]
    pub buf_pipe_stall: BUF_PIPE_STALL,
    #[doc = "0x3c - "]
    pub mac_data_b_wait: MAC_DATA_B_WAIT,
    #[doc = "0x40 - "]
    pub mac_pipe_stall: MAC_PIPE_STALL,
    #[doc = "0x44 - "]
    pub dma_ctrl: DMA_CTRL,
    #[doc = "0x48 - "]
    pub dma_pad: DMA_PAD,
    #[doc = "0x4c - "]
    pub power_ctrl: POWER_CTRL,
    #[doc = "0x50 - "]
    pub power_stat: POWER_STAT,
    #[doc = "0x54 - "]
    pub mac_sat_max: MAC_SAT_MAX,
    #[doc = "0x58 - "]
    pub mac_sat_min: MAC_SAT_MIN,
    #[doc = "0x5c - "]
    pub pp_axi_write: PP_AXI_WRITE,
    #[doc = "0x60 - "]
    pub pp_axi_read: PP_AXI_READ,
    #[doc = "0x64 - "]
    pub handshake: HANDSHAKE,
}
#[doc = "status (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "dla_ctrl (rw) register accessor: an alias for `Reg<DLA_CTRL_SPEC>`"]
pub type DLA_CTRL = crate::Reg<dla_ctrl::DLA_CTRL_SPEC>;
#[doc = ""]
pub mod dla_ctrl;
#[doc = "mac_ctrl (rw) register accessor: an alias for `Reg<MAC_CTRL_SPEC>`"]
pub type MAC_CTRL = crate::Reg<mac_ctrl::MAC_CTRL_SPEC>;
#[doc = ""]
pub mod mac_ctrl;
#[doc = "buf_input (rw) register accessor: an alias for `Reg<BUF_INPUT_SPEC>`"]
pub type BUF_INPUT = crate::Reg<buf_input::BUF_INPUT_SPEC>;
#[doc = ""]
pub mod buf_input;
#[doc = "buf_kernel0 (rw) register accessor: an alias for `Reg<BUF_KERNEL0_SPEC>`"]
pub type BUF_KERNEL0 = crate::Reg<buf_kernel0::BUF_KERNEL0_SPEC>;
#[doc = ""]
pub mod buf_kernel0;
#[doc = "buf_kernel1 (rw) register accessor: an alias for `Reg<BUF_KERNEL1_SPEC>`"]
pub type BUF_KERNEL1 = crate::Reg<buf_kernel1::BUF_KERNEL1_SPEC>;
#[doc = ""]
pub mod buf_kernel1;
#[doc = "buf_pad (rw) register accessor: an alias for `Reg<BUF_PAD_SPEC>`"]
pub type BUF_PAD = crate::Reg<buf_pad::BUF_PAD_SPEC>;
#[doc = ""]
pub mod buf_pad;
#[doc = "pp_input (rw) register accessor: an alias for `Reg<PP_INPUT_SPEC>`"]
pub type PP_INPUT = crate::Reg<pp_input::PP_INPUT_SPEC>;
#[doc = ""]
pub mod pp_input;
#[doc = "buf_stride (rw) register accessor: an alias for `Reg<BUF_STRIDE_SPEC>`"]
pub type BUF_STRIDE = crate::Reg<buf_stride::BUF_STRIDE_SPEC>;
#[doc = ""]
pub mod buf_stride;
#[doc = "buf_data_bank (rw) register accessor: an alias for `Reg<BUF_DATA_BANK_SPEC>`"]
pub type BUF_DATA_BANK = crate::Reg<buf_data_bank::BUF_DATA_BANK_SPEC>;
#[doc = ""]
pub mod buf_data_bank;
#[doc = "buf_data_wait_b (r) register accessor: an alias for `Reg<BUF_DATA_WAIT_B_SPEC>`"]
pub type BUF_DATA_WAIT_B = crate::Reg<buf_data_wait_b::BUF_DATA_WAIT_B_SPEC>;
#[doc = ""]
pub mod buf_data_wait_b;
#[doc = "buf_data_wait_a (r) register accessor: an alias for `Reg<BUF_DATA_WAIT_A_SPEC>`"]
pub type BUF_DATA_WAIT_A = crate::Reg<buf_data_wait_a::BUF_DATA_WAIT_A_SPEC>;
#[doc = ""]
pub mod buf_data_wait_a;
#[doc = "pp_axi_read (rw) register accessor: an alias for `Reg<PP_AXI_READ_SPEC>`"]
pub type PP_AXI_READ = crate::Reg<pp_axi_read::PP_AXI_READ_SPEC>;
#[doc = ""]
pub mod pp_axi_read;
#[doc = "pp_axi_write (rw) register accessor: an alias for `Reg<PP_AXI_WRITE_SPEC>`"]
pub type PP_AXI_WRITE = crate::Reg<pp_axi_write::PP_AXI_WRITE_SPEC>;
#[doc = ""]
pub mod pp_axi_write;
#[doc = "power_stat (r) register accessor: an alias for `Reg<POWER_STAT_SPEC>`"]
pub type POWER_STAT = crate::Reg<power_stat::POWER_STAT_SPEC>;
#[doc = ""]
pub mod power_stat;
#[doc = "power_ctrl (rw) register accessor: an alias for `Reg<POWER_CTRL_SPEC>`"]
pub type POWER_CTRL = crate::Reg<power_ctrl::POWER_CTRL_SPEC>;
#[doc = ""]
pub mod power_ctrl;
#[doc = "mac_sat_min (r) register accessor: an alias for `Reg<MAC_SAT_MIN_SPEC>`"]
pub type MAC_SAT_MIN = crate::Reg<mac_sat_min::MAC_SAT_MIN_SPEC>;
#[doc = ""]
pub mod mac_sat_min;
#[doc = "mac_sat_max (r) register accessor: an alias for `Reg<MAC_SAT_MAX_SPEC>`"]
pub type MAC_SAT_MAX = crate::Reg<mac_sat_max::MAC_SAT_MAX_SPEC>;
#[doc = ""]
pub mod mac_sat_max;
#[doc = "dma_pad (r) register accessor: an alias for `Reg<DMA_PAD_SPEC>`"]
pub type DMA_PAD = crate::Reg<dma_pad::DMA_PAD_SPEC>;
#[doc = ""]
pub mod dma_pad;
#[doc = "dma_ctrl (rw) register accessor: an alias for `Reg<DMA_CTRL_SPEC>`"]
pub type DMA_CTRL = crate::Reg<dma_ctrl::DMA_CTRL_SPEC>;
#[doc = ""]
pub mod dma_ctrl;
#[doc = "buf_pipe_stall (r) register accessor: an alias for `Reg<BUF_PIPE_STALL_SPEC>`"]
pub type BUF_PIPE_STALL = crate::Reg<buf_pipe_stall::BUF_PIPE_STALL_SPEC>;
#[doc = ""]
pub mod buf_pipe_stall;
#[doc = "buf_ctrl (rw) register accessor: an alias for `Reg<BUF_CTRL_SPEC>`"]
pub type BUF_CTRL = crate::Reg<buf_ctrl::BUF_CTRL_SPEC>;
#[doc = ""]
pub mod buf_ctrl;
#[doc = "handshake (rw) register accessor: an alias for `Reg<HANDSHAKE_SPEC>`"]
pub type HANDSHAKE = crate::Reg<handshake::HANDSHAKE_SPEC>;
#[doc = ""]
pub mod handshake;
#[doc = "pp_ptrl (rw) register accessor: an alias for `Reg<PP_PTRL_SPEC>`"]
pub type PP_PTRL = crate::Reg<pp_ptrl::PP_PTRL_SPEC>;
#[doc = ""]
pub mod pp_ptrl;
#[doc = "mac_data_b_wait (r) register accessor: an alias for `Reg<MAC_DATA_B_WAIT_SPEC>`"]
pub type MAC_DATA_B_WAIT = crate::Reg<mac_data_b_wait::MAC_DATA_B_WAIT_SPEC>;
#[doc = ""]
pub mod mac_data_b_wait;
#[doc = "mac_pipe_stall (r) register accessor: an alias for `Reg<MAC_PIPE_STALL_SPEC>`"]
pub type MAC_PIPE_STALL = crate::Reg<mac_pipe_stall::MAC_PIPE_STALL_SPEC>;
#[doc = ""]
pub mod mac_pipe_stall;
