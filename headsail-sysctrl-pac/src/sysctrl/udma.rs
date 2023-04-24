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
