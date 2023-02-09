#[doc = r"Register block"]
#[repr(C)]
pub struct UDMA {
    #[doc = "0x00 - uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
    pub ctrl_cfg_cg: CTRL_CFG_CG,
    #[doc = "0x04 - uDMA peripherals external event configuration"]
    pub ctrl_cfg_event: CTRL_CFG_EVENT,
    #[doc = "0x08 - uDMA peripherals reset trigger (unimplemented)"]
    pub ctrl_cfg_rst: CTRL_CFG_RST,
    _reserved3: [u8; 0x74],
    #[doc = "0x80 - uDMA RX UART buffer base address configuration register"]
    pub uart_rx_saddr: UART_RX_SADDR,
    #[doc = "0x84 - uDMA RX UART buffer size configuration register"]
    pub uart_rx_size: UART_RX_SIZE,
    #[doc = "0x88 - uDMA RX UART stream configuration register"]
    pub uart_rx_cfg: UART_RX_CFG,
    _reserved6: [u8; 0x04],
    #[doc = "0x90 - uDMA TX UART buffer base address configuration register."]
    pub uart_tx_saddr: UART_TX_SADDR,
    #[doc = "0x94 - uDMA TX UART buffer size configuration register"]
    pub uart_tx_size: UART_TX_SIZE,
    #[doc = "0x98 - uDMA TX UART stream configuration register."]
    pub uart_tx_cfg: UART_TX_CFG,
    _reserved9: [u8; 0x04],
    #[doc = "0xa0 - uDMA UART status register"]
    pub uart_status: UART_STATUS,
    #[doc = "0xa4 - UDMA UART configuration register."]
    pub uart_setup: UART_SETUP,
    #[doc = "0xa8 - uDMA UART Error status"]
    pub uart_error: UART_ERROR,
    #[doc = "0xac - uDMA UART Read or Error interrupt enable register"]
    pub uart_irq_en: UART_IRQ_EN,
    #[doc = "0xb0 - uDMA UART Read polling data valid flag register"]
    pub uart_valid: UART_VALID,
    #[doc = "0xb4 - RX read data for polling or interrupt"]
    pub uart_data: UART_DATA,
    _reserved15: [u8; 0x48],
    #[doc = "0x100 - RX SPI uDMA transfer address of associated buffer"]
    pub spim_rx_saddr: SPIM_RX_SADDR,
    #[doc = "0x104 - RX SPI uDMA transfer size of buffer"]
    pub spim_rx_size: SPIM_RX_SIZE,
    #[doc = "0x108 - RX SPI uDMA transfer configuration"]
    pub spim_rx_cfg: SPIM_RX_CFG,
    _reserved18: [u8; 0x04],
    #[doc = "0x110 - TX SPI uDMA transfer address of associated buffer"]
    pub spim_tx_saddr: SPIM_TX_SADDR,
    #[doc = "0x114 - TX SPI uDMA transfer size of buffer"]
    pub spim_tx_size: SPIM_TX_SIZE,
    #[doc = "0x118 - TX SPI uDMA transfer configuration"]
    pub spim_tx_cfg: SPIM_TX_CFG,
    _reserved21: [u8; 0x04],
    #[doc = "0x120 - CMD SPI uDMA transfer address of associated buffer"]
    pub spim_cmd_saddr: SPIM_CMD_SADDR,
    #[doc = "0x124 - CMD SPI uDMA transfer size of buffer"]
    pub spim_cmd_size: SPIM_CMD_SIZE,
    #[doc = "0x128 - CMD SPI uDMA transfer configuration"]
    pub spim_cmd_cfg: SPIM_CMD_CFG,
    _reserved24: [u8; 0x02d4],
    #[doc = "0x400 - FILTER tx channel address register"]
    pub tx_ch0_add: TX_CH0_ADD,
    #[doc = "0x404 - FILTER tx channel configuration register"]
    pub tx_ch0_cfg: TX_CH0_CFG,
    #[doc = "0x408 - FILTER tx channel length1 register"]
    pub tx_ch0_len0: TX_CH0_LEN0,
    #[doc = "0x40c - FILTER tx channel length2 register"]
    pub tx_ch0_len1: TX_CH0_LEN1,
    #[doc = "0x410 - FILTER tx channel 0 length2 register"]
    pub tx_ch0_len2: TX_CH0_LEN2,
    #[doc = "0x414 - FILTER tx channel address register"]
    pub tx_ch1_add: TX_CH1_ADD,
    #[doc = "0x418 - FILTER tx channel configuration register"]
    pub tx_ch1_cfg: TX_CH1_CFG,
    #[doc = "0x41c - FILTER tx channel length1 register"]
    pub tx_ch1_len0: TX_CH1_LEN0,
    #[doc = "0x420 - FILTER tx channel length2 register"]
    pub tx_ch1_len1: TX_CH1_LEN1,
    #[doc = "0x424 - FILTER RX channel configuration register"]
    pub tx_ch1_len2: TX_CH1_LEN2,
    #[doc = "0x428 - FILTER RX channel address register"]
    pub rx_ch_add: RX_CH_ADD,
    #[doc = "0x42c - FILTER RX channel configuration register"]
    pub rx_ch_cfg: RX_CH_CFG,
    #[doc = "0x430 - FILTER RX channel configuration register"]
    pub rx_ch_len0: RX_CH_LEN0,
    #[doc = "0x434 - FILTER RX channel length1 register"]
    pub rx_ch_len1: RX_CH_LEN1,
    #[doc = "0x438 - FILTER RX channel length2 register"]
    pub rx_ch_len2: RX_CH_LEN2,
    #[doc = "0x43c - FILTER arithmetic unit configuration register"]
    pub au_cfg: AU_CFG,
    #[doc = "0x440 - FILTER arithmetic unit 0 register"]
    pub au_reg0: AU_REG0,
    #[doc = "0x444 - FILTER arithmetic unit 1 register"]
    pub au_reg1: AU_REG1,
    #[doc = "0x448 - FILTER binarization threshold register"]
    pub bincu_th: BINCU_TH,
    #[doc = "0x44c - FILTER binarization count register"]
    pub bincu_cnt: BINCU_CNT,
    #[doc = "0x450 - FILTER binarization result count register"]
    pub bincu_setup: BINCU_SETUP,
    #[doc = "0x454 - FILTER binarization result count register"]
    pub bincu_val: BINCU_VAL,
    #[doc = "0x458 - FILTER control mode register"]
    pub filt: FILT,
    #[doc = "0x45c - FILTER start register"]
    pub filt_cmd: FILT_CMD,
    #[doc = "0x460 - FILTER status register"]
    pub filt_status: FILT_STATUS,
}
#[doc = "CTRL_CFG_CG (rw) register accessor: an alias for `Reg<CTRL_CFG_CG_SPEC>`"]
pub type CTRL_CFG_CG = crate::Reg<ctrl_cfg_cg::CTRL_CFG_CG_SPEC>;
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
pub mod ctrl_cfg_cg;
#[doc = "CTRL_CFG_EVENT (rw) register accessor: an alias for `Reg<CTRL_CFG_EVENT_SPEC>`"]
pub type CTRL_CFG_EVENT = crate::Reg<ctrl_cfg_event::CTRL_CFG_EVENT_SPEC>;
#[doc = "uDMA peripherals external event configuration"]
pub mod ctrl_cfg_event;
#[doc = "CTRL_CFG_RST (rw) register accessor: an alias for `Reg<CTRL_CFG_RST_SPEC>`"]
pub type CTRL_CFG_RST = crate::Reg<ctrl_cfg_rst::CTRL_CFG_RST_SPEC>;
#[doc = "uDMA peripherals reset trigger (unimplemented)"]
pub mod ctrl_cfg_rst;
#[doc = "UART_RX_SADDR (rw) register accessor: an alias for `Reg<UART_RX_SADDR_SPEC>`"]
pub type UART_RX_SADDR = crate::Reg<uart_rx_saddr::UART_RX_SADDR_SPEC>;
#[doc = "uDMA RX UART buffer base address configuration register"]
pub mod uart_rx_saddr;
#[doc = "UART_RX_SIZE (rw) register accessor: an alias for `Reg<UART_RX_SIZE_SPEC>`"]
pub type UART_RX_SIZE = crate::Reg<uart_rx_size::UART_RX_SIZE_SPEC>;
#[doc = "uDMA RX UART buffer size configuration register"]
pub mod uart_rx_size;
#[doc = "UART_RX_CFG (rw) register accessor: an alias for `Reg<UART_RX_CFG_SPEC>`"]
pub type UART_RX_CFG = crate::Reg<uart_rx_cfg::UART_RX_CFG_SPEC>;
#[doc = "uDMA RX UART stream configuration register"]
pub mod uart_rx_cfg;
#[doc = "UART_TX_SADDR (rw) register accessor: an alias for `Reg<UART_TX_SADDR_SPEC>`"]
pub type UART_TX_SADDR = crate::Reg<uart_tx_saddr::UART_TX_SADDR_SPEC>;
#[doc = "uDMA TX UART buffer base address configuration register."]
pub mod uart_tx_saddr;
#[doc = "UART_TX_SIZE (rw) register accessor: an alias for `Reg<UART_TX_SIZE_SPEC>`"]
pub type UART_TX_SIZE = crate::Reg<uart_tx_size::UART_TX_SIZE_SPEC>;
#[doc = "uDMA TX UART buffer size configuration register"]
pub mod uart_tx_size;
#[doc = "UART_TX_CFG (rw) register accessor: an alias for `Reg<UART_TX_CFG_SPEC>`"]
pub type UART_TX_CFG = crate::Reg<uart_tx_cfg::UART_TX_CFG_SPEC>;
#[doc = "uDMA TX UART stream configuration register."]
pub mod uart_tx_cfg;
#[doc = "UART_ERROR (r) register accessor: an alias for `Reg<UART_ERROR_SPEC>`"]
pub type UART_ERROR = crate::Reg<uart_error::UART_ERROR_SPEC>;
#[doc = "uDMA UART Error status"]
pub mod uart_error;
#[doc = "SPIM_RX_SIZE (rw) register accessor: an alias for `Reg<SPIM_RX_SIZE_SPEC>`"]
pub type SPIM_RX_SIZE = crate::Reg<spim_rx_size::SPIM_RX_SIZE_SPEC>;
#[doc = "RX SPI uDMA transfer size of buffer"]
pub mod spim_rx_size;
#[doc = "UART_DATA (r) register accessor: an alias for `Reg<UART_DATA_SPEC>`"]
pub type UART_DATA = crate::Reg<uart_data::UART_DATA_SPEC>;
#[doc = "RX read data for polling or interrupt"]
pub mod uart_data;
#[doc = "SPIM_RX_SADDR (rw) register accessor: an alias for `Reg<SPIM_RX_SADDR_SPEC>`"]
pub type SPIM_RX_SADDR = crate::Reg<spim_rx_saddr::SPIM_RX_SADDR_SPEC>;
#[doc = "RX SPI uDMA transfer address of associated buffer"]
pub mod spim_rx_saddr;
#[doc = "UART_STATUS (r) register accessor: an alias for `Reg<UART_STATUS_SPEC>`"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "uDMA UART status register"]
pub mod uart_status;
#[doc = "UART_IRQ_EN (rw) register accessor: an alias for `Reg<UART_IRQ_EN_SPEC>`"]
pub type UART_IRQ_EN = crate::Reg<uart_irq_en::UART_IRQ_EN_SPEC>;
#[doc = "uDMA UART Read or Error interrupt enable register"]
pub mod uart_irq_en;
#[doc = "UART_VALID (r) register accessor: an alias for `Reg<UART_VALID_SPEC>`"]
pub type UART_VALID = crate::Reg<uart_valid::UART_VALID_SPEC>;
#[doc = "uDMA UART Read polling data valid flag register"]
pub mod uart_valid;
#[doc = "SPIM_CMD_SADDR (rw) register accessor: an alias for `Reg<SPIM_CMD_SADDR_SPEC>`"]
pub type SPIM_CMD_SADDR = crate::Reg<spim_cmd_saddr::SPIM_CMD_SADDR_SPEC>;
#[doc = "CMD SPI uDMA transfer address of associated buffer"]
pub mod spim_cmd_saddr;
#[doc = "SPIM_TX_SADDR (rw) register accessor: an alias for `Reg<SPIM_TX_SADDR_SPEC>`"]
pub type SPIM_TX_SADDR = crate::Reg<spim_tx_saddr::SPIM_TX_SADDR_SPEC>;
#[doc = "TX SPI uDMA transfer address of associated buffer"]
pub mod spim_tx_saddr;
#[doc = "SPIM_CMD_SIZE (rw) register accessor: an alias for `Reg<SPIM_CMD_SIZE_SPEC>`"]
pub type SPIM_CMD_SIZE = crate::Reg<spim_cmd_size::SPIM_CMD_SIZE_SPEC>;
#[doc = "CMD SPI uDMA transfer size of buffer"]
pub mod spim_cmd_size;
#[doc = "SPIM_TX_SIZE (rw) register accessor: an alias for `Reg<SPIM_TX_SIZE_SPEC>`"]
pub type SPIM_TX_SIZE = crate::Reg<spim_tx_size::SPIM_TX_SIZE_SPEC>;
#[doc = "TX SPI uDMA transfer size of buffer"]
pub mod spim_tx_size;
#[doc = "SPIM_CMD_CFG (rw) register accessor: an alias for `Reg<SPIM_CMD_CFG_SPEC>`"]
pub type SPIM_CMD_CFG = crate::Reg<spim_cmd_cfg::SPIM_CMD_CFG_SPEC>;
#[doc = "CMD SPI uDMA transfer configuration"]
pub mod spim_cmd_cfg;
#[doc = "UART_SETUP (rw) register accessor: an alias for `Reg<UART_SETUP_SPEC>`"]
pub type UART_SETUP = crate::Reg<uart_setup::UART_SETUP_SPEC>;
#[doc = "UDMA UART configuration register."]
pub mod uart_setup;
#[doc = "SPIM_TX_CFG (rw) register accessor: an alias for `Reg<SPIM_TX_CFG_SPEC>`"]
pub type SPIM_TX_CFG = crate::Reg<spim_tx_cfg::SPIM_TX_CFG_SPEC>;
#[doc = "TX SPI uDMA transfer configuration"]
pub mod spim_tx_cfg;
#[doc = "SPIM_RX_CFG (rw) register accessor: an alias for `Reg<SPIM_RX_CFG_SPEC>`"]
pub type SPIM_RX_CFG = crate::Reg<spim_rx_cfg::SPIM_RX_CFG_SPEC>;
#[doc = "RX SPI uDMA transfer configuration"]
pub mod spim_rx_cfg;
#[doc = "TX_CH0_LEN1 (rw) register accessor: an alias for `Reg<TX_CH0_LEN1_SPEC>`"]
pub type TX_CH0_LEN1 = crate::Reg<tx_ch0_len1::TX_CH0_LEN1_SPEC>;
#[doc = "FILTER tx channel length2 register"]
pub mod tx_ch0_len1;
#[doc = "TX_CH0_CFG (rw) register accessor: an alias for `Reg<TX_CH0_CFG_SPEC>`"]
pub type TX_CH0_CFG = crate::Reg<tx_ch0_cfg::TX_CH0_CFG_SPEC>;
#[doc = "FILTER tx channel configuration register"]
pub mod tx_ch0_cfg;
#[doc = "TX_CH0_LEN2 (rw) register accessor: an alias for `Reg<TX_CH0_LEN2_SPEC>`"]
pub type TX_CH0_LEN2 = crate::Reg<tx_ch0_len2::TX_CH0_LEN2_SPEC>;
#[doc = "FILTER tx channel 0 length2 register"]
pub mod tx_ch0_len2;
#[doc = "TX_CH0_LEN0 (rw) register accessor: an alias for `Reg<TX_CH0_LEN0_SPEC>`"]
pub type TX_CH0_LEN0 = crate::Reg<tx_ch0_len0::TX_CH0_LEN0_SPEC>;
#[doc = "FILTER tx channel length1 register"]
pub mod tx_ch0_len0;
#[doc = "TX_CH0_ADD (rw) register accessor: an alias for `Reg<TX_CH0_ADD_SPEC>`"]
pub type TX_CH0_ADD = crate::Reg<tx_ch0_add::TX_CH0_ADD_SPEC>;
#[doc = "FILTER tx channel address register"]
pub mod tx_ch0_add;
#[doc = "TX_CH1_ADD (rw) register accessor: an alias for `Reg<TX_CH1_ADD_SPEC>`"]
pub type TX_CH1_ADD = crate::Reg<tx_ch1_add::TX_CH1_ADD_SPEC>;
#[doc = "FILTER tx channel address register"]
pub mod tx_ch1_add;
#[doc = "TX_CH1_CFG (rw) register accessor: an alias for `Reg<TX_CH1_CFG_SPEC>`"]
pub type TX_CH1_CFG = crate::Reg<tx_ch1_cfg::TX_CH1_CFG_SPEC>;
#[doc = "FILTER tx channel configuration register"]
pub mod tx_ch1_cfg;
#[doc = "TX_CH1_LEN0 (rw) register accessor: an alias for `Reg<TX_CH1_LEN0_SPEC>`"]
pub type TX_CH1_LEN0 = crate::Reg<tx_ch1_len0::TX_CH1_LEN0_SPEC>;
#[doc = "FILTER tx channel length1 register"]
pub mod tx_ch1_len0;
#[doc = "TX_CH1_LEN1 (rw) register accessor: an alias for `Reg<TX_CH1_LEN1_SPEC>`"]
pub type TX_CH1_LEN1 = crate::Reg<tx_ch1_len1::TX_CH1_LEN1_SPEC>;
#[doc = "FILTER tx channel length2 register"]
pub mod tx_ch1_len1;
#[doc = "FILT_CMD (rw) register accessor: an alias for `Reg<FILT_CMD_SPEC>`"]
pub type FILT_CMD = crate::Reg<filt_cmd::FILT_CMD_SPEC>;
#[doc = "FILTER start register"]
pub mod filt_cmd;
#[doc = "BINCU_VAL (r) register accessor: an alias for `Reg<BINCU_VAL_SPEC>`"]
pub type BINCU_VAL = crate::Reg<bincu_val::BINCU_VAL_SPEC>;
#[doc = "FILTER binarization result count register"]
pub mod bincu_val;
#[doc = "BINCU_CNT (rw) register accessor: an alias for `Reg<BINCU_CNT_SPEC>`"]
pub type BINCU_CNT = crate::Reg<bincu_cnt::BINCU_CNT_SPEC>;
#[doc = "FILTER binarization count register"]
pub mod bincu_cnt;
#[doc = "AU_REG1 (rw) register accessor: an alias for `Reg<AU_REG1_SPEC>`"]
pub type AU_REG1 = crate::Reg<au_reg1::AU_REG1_SPEC>;
#[doc = "FILTER arithmetic unit 1 register"]
pub mod au_reg1;
#[doc = "AU_CFG (rw) register accessor: an alias for `Reg<AU_CFG_SPEC>`"]
pub type AU_CFG = crate::Reg<au_cfg::AU_CFG_SPEC>;
#[doc = "FILTER arithmetic unit configuration register"]
pub mod au_cfg;
#[doc = "RX_CH_LEN1 (rw) register accessor: an alias for `Reg<RX_CH_LEN1_SPEC>`"]
pub type RX_CH_LEN1 = crate::Reg<rx_ch_len1::RX_CH_LEN1_SPEC>;
#[doc = "FILTER RX channel length1 register"]
pub mod rx_ch_len1;
#[doc = "RX_CH_CFG (rw) register accessor: an alias for `Reg<RX_CH_CFG_SPEC>`"]
pub type RX_CH_CFG = crate::Reg<rx_ch_cfg::RX_CH_CFG_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod rx_ch_cfg;
#[doc = "TX_CH1_LEN2 (rw) register accessor: an alias for `Reg<TX_CH1_LEN2_SPEC>`"]
pub type TX_CH1_LEN2 = crate::Reg<tx_ch1_len2::TX_CH1_LEN2_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod tx_ch1_len2;
#[doc = "FILT_STATUS (rw) register accessor: an alias for `Reg<FILT_STATUS_SPEC>`"]
pub type FILT_STATUS = crate::Reg<filt_status::FILT_STATUS_SPEC>;
#[doc = "FILTER status register"]
pub mod filt_status;
#[doc = "FILT (rw) register accessor: an alias for `Reg<FILT_SPEC>`"]
pub type FILT = crate::Reg<filt::FILT_SPEC>;
#[doc = "FILTER control mode register"]
pub mod filt;
#[doc = "BINCU_SETUP (rw) register accessor: an alias for `Reg<BINCU_SETUP_SPEC>`"]
pub type BINCU_SETUP = crate::Reg<bincu_setup::BINCU_SETUP_SPEC>;
#[doc = "FILTER binarization result count register"]
pub mod bincu_setup;
#[doc = "BINCU_TH (rw) register accessor: an alias for `Reg<BINCU_TH_SPEC>`"]
pub type BINCU_TH = crate::Reg<bincu_th::BINCU_TH_SPEC>;
#[doc = "FILTER binarization threshold register"]
pub mod bincu_th;
#[doc = "AU_REG0 (rw) register accessor: an alias for `Reg<AU_REG0_SPEC>`"]
pub type AU_REG0 = crate::Reg<au_reg0::AU_REG0_SPEC>;
#[doc = "FILTER arithmetic unit 0 register"]
pub mod au_reg0;
#[doc = "RX_CH_LEN2 (rw) register accessor: an alias for `Reg<RX_CH_LEN2_SPEC>`"]
pub type RX_CH_LEN2 = crate::Reg<rx_ch_len2::RX_CH_LEN2_SPEC>;
#[doc = "FILTER RX channel length2 register"]
pub mod rx_ch_len2;
#[doc = "RX_CH_LEN0 (rw) register accessor: an alias for `Reg<RX_CH_LEN0_SPEC>`"]
pub type RX_CH_LEN0 = crate::Reg<rx_ch_len0::RX_CH_LEN0_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod rx_ch_len0;
#[doc = "RX_CH_ADD (rw) register accessor: an alias for `Reg<RX_CH_ADD_SPEC>`"]
pub type RX_CH_ADD = crate::Reg<rx_ch_add::RX_CH_ADD_SPEC>;
#[doc = "FILTER RX channel address register"]
pub mod rx_ch_add;
