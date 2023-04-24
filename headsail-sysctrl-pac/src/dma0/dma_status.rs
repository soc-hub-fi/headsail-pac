#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_STATUS {
    #[doc = "0x00 - Constant 32'h70646d61 default value ; \"pdma\" in ascii"]
    pub ip_id: IP_ID,
    #[doc = "0x04 - 32'h00200401 default value Version number Number of available channels AXI data width"]
    pub info: INFO,
    #[doc = "0x08..0x18 - Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
    pub status: [STATUS; 4],
    #[doc = "0x18..0x28 - Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
    pub status_clear: [STATUS_CLEAR; 4],
    #[doc = "0x28..0x38 - Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    pub irq_enable: [IRQ_ENABLE; 4],
    #[doc = "0x38 - Configures the state (active high/low) and clearing conditions for the IRQ pin. Cleared: '0' = Use STATUS_CLEAR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high Set all bits IRQ_CONFIG(7 downto 4) of polarity to \"1111\" for IRQ active high or \"0000\" for IRQ active low."]
    pub irq_config: IRQ_CONFIG,
    #[doc = "0x3c - Configures Pad, 10 Bits for read pad configuration 10 bits for write pad configuration."]
    pub pad_config: PAD_CONFIG,
}
#[doc = "IP_ID (r) register accessor: an alias for `Reg<IP_ID_SPEC>`"]
pub type IP_ID = crate::Reg<ip_id::IP_ID_SPEC>;
#[doc = "Constant 32'h70646d61 default value ; \"pdma\" in ascii"]
pub mod ip_id;
#[doc = "INFO (r) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "32'h00200401 default value Version number Number of available channels AXI data width"]
pub mod info;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
pub mod status;
#[doc = "STATUS_CLEAR (rw) register accessor: an alias for `Reg<STATUS_CLEAR_SPEC>`"]
pub type STATUS_CLEAR = crate::Reg<status_clear::STATUS_CLEAR_SPEC>;
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
pub mod status_clear;
#[doc = "IRQ_ENABLE (rw) register accessor: an alias for `Reg<IRQ_ENABLE_SPEC>`"]
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLE_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_enable;
#[doc = "PAD_CONFIG (rw) register accessor: an alias for `Reg<PAD_CONFIG_SPEC>`"]
pub type PAD_CONFIG = crate::Reg<pad_config::PAD_CONFIG_SPEC>;
#[doc = "Configures Pad, 10 Bits for read pad configuration 10 bits for write pad configuration."]
pub mod pad_config;
#[doc = "IRQ_CONFIG (rw) register accessor: an alias for `Reg<IRQ_CONFIG_SPEC>`"]
pub type IRQ_CONFIG = crate::Reg<irq_config::IRQ_CONFIG_SPEC>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Cleared: '0' = Use STATUS_CLEAR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high Set all bits IRQ_CONFIG(7 downto 4) of polarity to \"1111\" for IRQ active high or \"0000\" for IRQ active low."]
pub mod irq_config;
