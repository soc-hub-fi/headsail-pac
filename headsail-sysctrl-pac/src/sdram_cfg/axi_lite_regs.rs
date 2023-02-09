#[doc = r"Register block"]
#[repr(C)]
pub struct AXI_LITE_REGS {
    #[doc = "0x00 - "]
    pub ddr_refresh_ack: DDR_REFRESH_ACK,
    #[doc = "0x04 - "]
    pub ddr_init_done: DDR_INIT_DONE,
    #[doc = "0x08 - "]
    pub axi_enabled: AXI_ENABLED,
    #[doc = "0x0c - "]
    pub t_init: T_INIT,
    #[doc = "0x10 - "]
    pub t_arf: T_ARF,
    #[doc = "0x14 - "]
    pub t_mrd: T_MRD,
    #[doc = "0x18 - "]
    pub t_ras: T_RAS,
    #[doc = "0x1c - "]
    pub t_rc: T_RC,
    #[doc = "0x20 - "]
    pub t_rcd: T_RCD,
    #[doc = "0x24 - "]
    pub t_rfc: T_RFC,
    #[doc = "0x28 - "]
    pub t_rp: T_RP,
    #[doc = "0x2c - "]
    pub t_rrd: T_RRD,
    #[doc = "0x30 - "]
    pub t_wr: T_WR,
    #[doc = "0x34 - "]
    pub t_wtr: T_WTR,
    #[doc = "0x38 - "]
    pub c_burst_len: C_BURST_LEN,
    #[doc = "0x3c - "]
    pub c_burst_cycles: C_BURST_CYCLES,
    #[doc = "0x40 - "]
    pub c_rd_lat: C_RD_LAT,
    #[doc = "0x44 - "]
    pub c_wr_lat: C_WR_LAT,
    #[doc = "0x48 - "]
    pub c_col_shift: C_COL_SHIFT,
    #[doc = "0x4c - "]
    pub c_col_mask: C_COL_MASK,
    #[doc = "0x50 - "]
    pub c_row_shift: C_ROW_SHIFT,
    #[doc = "0x54 - "]
    pub c_row_mask: C_ROW_MASK,
    #[doc = "0x58 - "]
    pub c_bank_shift: C_BANK_SHIFT,
    #[doc = "0x5c - "]
    pub c_bank_mask: C_BANK_MASK,
    #[doc = "0x60 - "]
    pub c_offset: C_OFFSET,
    #[doc = "0x64 - "]
    pub c_start_addr: C_START_ADDR,
    #[doc = "0x68 - "]
    pub c_end_addr: C_END_ADDR,
    #[doc = "0x6c - "]
    pub d_wrseparation: D_WRSEPARATION,
    #[doc = "0x70 - "]
    pub d_wrlatcorr: D_WRLATCORR,
}
#[doc = "ddr_refresh_ack (rw) register accessor: an alias for `Reg<DDR_REFRESH_ACK_SPEC>`"]
pub type DDR_REFRESH_ACK = crate::Reg<ddr_refresh_ack::DDR_REFRESH_ACK_SPEC>;
#[doc = ""]
pub mod ddr_refresh_ack;
#[doc = "ddr_init_done (rw) register accessor: an alias for `Reg<DDR_INIT_DONE_SPEC>`"]
pub type DDR_INIT_DONE = crate::Reg<ddr_init_done::DDR_INIT_DONE_SPEC>;
#[doc = ""]
pub mod ddr_init_done;
#[doc = "axi_enabled (rw) register accessor: an alias for `Reg<AXI_ENABLED_SPEC>`"]
pub type AXI_ENABLED = crate::Reg<axi_enabled::AXI_ENABLED_SPEC>;
#[doc = ""]
pub mod axi_enabled;
#[doc = "t_init (rw) register accessor: an alias for `Reg<T_INIT_SPEC>`"]
pub type T_INIT = crate::Reg<t_init::T_INIT_SPEC>;
#[doc = ""]
pub mod t_init;
#[doc = "t_arf (rw) register accessor: an alias for `Reg<T_ARF_SPEC>`"]
pub type T_ARF = crate::Reg<t_arf::T_ARF_SPEC>;
#[doc = ""]
pub mod t_arf;
#[doc = "t_mrd (rw) register accessor: an alias for `Reg<T_MRD_SPEC>`"]
pub type T_MRD = crate::Reg<t_mrd::T_MRD_SPEC>;
#[doc = ""]
pub mod t_mrd;
#[doc = "t_ras (rw) register accessor: an alias for `Reg<T_RAS_SPEC>`"]
pub type T_RAS = crate::Reg<t_ras::T_RAS_SPEC>;
#[doc = ""]
pub mod t_ras;
#[doc = "t_rc (rw) register accessor: an alias for `Reg<T_RC_SPEC>`"]
pub type T_RC = crate::Reg<t_rc::T_RC_SPEC>;
#[doc = ""]
pub mod t_rc;
#[doc = "t_rcd (rw) register accessor: an alias for `Reg<T_RCD_SPEC>`"]
pub type T_RCD = crate::Reg<t_rcd::T_RCD_SPEC>;
#[doc = ""]
pub mod t_rcd;
#[doc = "t_rfc (rw) register accessor: an alias for `Reg<T_RFC_SPEC>`"]
pub type T_RFC = crate::Reg<t_rfc::T_RFC_SPEC>;
#[doc = ""]
pub mod t_rfc;
#[doc = "t_rp (rw) register accessor: an alias for `Reg<T_RP_SPEC>`"]
pub type T_RP = crate::Reg<t_rp::T_RP_SPEC>;
#[doc = ""]
pub mod t_rp;
#[doc = "t_rrd (rw) register accessor: an alias for `Reg<T_RRD_SPEC>`"]
pub type T_RRD = crate::Reg<t_rrd::T_RRD_SPEC>;
#[doc = ""]
pub mod t_rrd;
#[doc = "t_wr (rw) register accessor: an alias for `Reg<T_WR_SPEC>`"]
pub type T_WR = crate::Reg<t_wr::T_WR_SPEC>;
#[doc = ""]
pub mod t_wr;
#[doc = "t_wtr (rw) register accessor: an alias for `Reg<T_WTR_SPEC>`"]
pub type T_WTR = crate::Reg<t_wtr::T_WTR_SPEC>;
#[doc = ""]
pub mod t_wtr;
#[doc = "c_burst_len (rw) register accessor: an alias for `Reg<C_BURST_LEN_SPEC>`"]
pub type C_BURST_LEN = crate::Reg<c_burst_len::C_BURST_LEN_SPEC>;
#[doc = ""]
pub mod c_burst_len;
#[doc = "c_burst_cycles (rw) register accessor: an alias for `Reg<C_BURST_CYCLES_SPEC>`"]
pub type C_BURST_CYCLES = crate::Reg<c_burst_cycles::C_BURST_CYCLES_SPEC>;
#[doc = ""]
pub mod c_burst_cycles;
#[doc = "c_rd_lat (rw) register accessor: an alias for `Reg<C_RD_LAT_SPEC>`"]
pub type C_RD_LAT = crate::Reg<c_rd_lat::C_RD_LAT_SPEC>;
#[doc = ""]
pub mod c_rd_lat;
#[doc = "c_wr_lat (rw) register accessor: an alias for `Reg<C_WR_LAT_SPEC>`"]
pub type C_WR_LAT = crate::Reg<c_wr_lat::C_WR_LAT_SPEC>;
#[doc = ""]
pub mod c_wr_lat;
#[doc = "c_col_shift (rw) register accessor: an alias for `Reg<C_COL_SHIFT_SPEC>`"]
pub type C_COL_SHIFT = crate::Reg<c_col_shift::C_COL_SHIFT_SPEC>;
#[doc = ""]
pub mod c_col_shift;
#[doc = "c_col_mask (rw) register accessor: an alias for `Reg<C_COL_MASK_SPEC>`"]
pub type C_COL_MASK = crate::Reg<c_col_mask::C_COL_MASK_SPEC>;
#[doc = ""]
pub mod c_col_mask;
#[doc = "c_row_shift (rw) register accessor: an alias for `Reg<C_ROW_SHIFT_SPEC>`"]
pub type C_ROW_SHIFT = crate::Reg<c_row_shift::C_ROW_SHIFT_SPEC>;
#[doc = ""]
pub mod c_row_shift;
#[doc = "c_row_mask (rw) register accessor: an alias for `Reg<C_ROW_MASK_SPEC>`"]
pub type C_ROW_MASK = crate::Reg<c_row_mask::C_ROW_MASK_SPEC>;
#[doc = ""]
pub mod c_row_mask;
#[doc = "c_bank_shift (rw) register accessor: an alias for `Reg<C_BANK_SHIFT_SPEC>`"]
pub type C_BANK_SHIFT = crate::Reg<c_bank_shift::C_BANK_SHIFT_SPEC>;
#[doc = ""]
pub mod c_bank_shift;
#[doc = "c_bank_mask (rw) register accessor: an alias for `Reg<C_BANK_MASK_SPEC>`"]
pub type C_BANK_MASK = crate::Reg<c_bank_mask::C_BANK_MASK_SPEC>;
#[doc = ""]
pub mod c_bank_mask;
#[doc = "c_offset (rw) register accessor: an alias for `Reg<C_OFFSET_SPEC>`"]
pub type C_OFFSET = crate::Reg<c_offset::C_OFFSET_SPEC>;
#[doc = ""]
pub mod c_offset;
#[doc = "c_start_addr (rw) register accessor: an alias for `Reg<C_START_ADDR_SPEC>`"]
pub type C_START_ADDR = crate::Reg<c_start_addr::C_START_ADDR_SPEC>;
#[doc = ""]
pub mod c_start_addr;
#[doc = "c_end_addr (rw) register accessor: an alias for `Reg<C_END_ADDR_SPEC>`"]
pub type C_END_ADDR = crate::Reg<c_end_addr::C_END_ADDR_SPEC>;
#[doc = ""]
pub mod c_end_addr;
#[doc = "d_wrseparation (rw) register accessor: an alias for `Reg<D_WRSEPARATION_SPEC>`"]
pub type D_WRSEPARATION = crate::Reg<d_wrseparation::D_WRSEPARATION_SPEC>;
#[doc = ""]
pub mod d_wrseparation;
#[doc = "d_wrlatcorr (rw) register accessor: an alias for `Reg<D_WRLATCORR_SPEC>`"]
pub type D_WRLATCORR = crate::Reg<d_wrlatcorr::D_WRLATCORR_SPEC>;
#[doc = ""]
pub mod d_wrlatcorr;
