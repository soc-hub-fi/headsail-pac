#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIGURATION_REGISTERS {
    #[doc = "0x00 - ID register. Constant ID value"]
    pub id: ID,
    #[doc = "0x04 - Version number - AXI Address width - AXI Data width-Reserved"]
    pub info: INFO,
    #[doc = "0x08 - Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
    pub status: STATUS,
    #[doc = "0x0c - Acknowledging status register. Clearing each respective one by setting corresponding bit"]
    pub status_clr: STATUS_CLR,
    #[doc = "0x10 - Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    pub irq_en: IRQ_EN,
    #[doc = "0x14 - Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
    pub irq_config: IRQ_CONFIG,
    #[doc = "0x18 - Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
    pub address_mapping_tx: ADDRESS_MAPPING_TX,
    #[doc = "0x1c - Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    pub address_masking_tx: ADDRESS_MASKING_TX,
    #[doc = "0x20 - Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
    pub address_mapping_rx: ADDRESS_MAPPING_RX,
    #[doc = "0x24 - Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
    pub address_masking_rx: ADDRESS_MASKING_RX,
    #[doc = "0x28 - PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    pub pad_c2c_config_1: PAD_C2C_CONFIG_1,
    #[doc = "0x2c - PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
    pub pad_c2c_config_2: PAD_C2C_CONFIG_2,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID register. Constant ID value"]
pub mod id;
#[doc = "INFO (r) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Version number - AXI Address width - AXI Data width-Reserved"]
pub mod info;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Holds status flags. Each Status Flag can either be cleared by writing to the STATUS_CLR register or when the STATUS register is read (must be configured)"]
pub mod status;
#[doc = "STATUS_CLR (rw) register accessor: an alias for `Reg<STATUS_CLR_SPEC>`"]
pub type STATUS_CLR = crate::Reg<status_clr::STATUS_CLR_SPEC>;
#[doc = "Acknowledging status register. Clearing each respective one by setting corresponding bit"]
pub mod status_clr;
#[doc = "IRQ_EN (rw) register accessor: an alias for `Reg<IRQ_EN_SPEC>`"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin. Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_en;
#[doc = "IRQ_CONFIG (rw) register accessor: an alias for `Reg<IRQ_CONFIG_SPEC>`"]
pub type IRQ_CONFIG = crate::Reg<irq_config::IRQ_CONFIG_SPEC>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Clear: '0' = Use STATUS_CLR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high"]
pub mod irq_config;
#[doc = "ADDRESS_MAPPING_TX (rw) register accessor: an alias for `Reg<ADDRESS_MAPPING_TX_SPEC>`"]
pub type ADDRESS_MAPPING_TX = crate::Reg<address_mapping_tx::ADDRESS_MAPPING_TX_SPEC>;
#[doc = "Address Mapping for AXI4 slave interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_tx;
#[doc = "PAD_C2C_CONFIG_2 (rw) register accessor: an alias for `Reg<PAD_C2C_CONFIG_2_SPEC>`"]
pub type PAD_C2C_CONFIG_2 = crate::Reg<pad_c2c_config_2::PAD_C2C_CONFIG_2_SPEC>;
#[doc = "PAD configuration Register 2. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_2;
#[doc = "PAD_C2C_CONFIG_1 (rw) register accessor: an alias for `Reg<PAD_C2C_CONFIG_1_SPEC>`"]
pub type PAD_C2C_CONFIG_1 = crate::Reg<pad_c2c_config_1::PAD_C2C_CONFIG_1_SPEC>;
#[doc = "PAD configuration Register 1. It is used to perform proper configuration for the corresponding pads of the I/O ports 10 bits per PAD configuration 0: drive strenght / 1: drive strenght / 2: schmitt trigger / 3: schmitt trigger / 4: rate / 5: output en(0) / 6: hold / 7: pull enable / 8: pd(0)/pu(1) / 9: input en(1)"]
pub mod pad_c2c_config_1;
#[doc = "ADDRESS_MASKING_RX (rw) register accessor: an alias for `Reg<ADDRESS_MASKING_RX_SPEC>`"]
pub type ADDRESS_MASKING_RX = crate::Reg<address_masking_rx::ADDRESS_MASKING_RX_SPEC>;
#[doc = "Address Masking for AXI4 master interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_rx;
#[doc = "ADDRESS_MAPPING_RX (rw) register accessor: an alias for `Reg<ADDRESS_MAPPING_RX_SPEC>`"]
pub type ADDRESS_MAPPING_RX = crate::Reg<address_mapping_rx::ADDRESS_MAPPING_RX_SPEC>;
#[doc = "Address Mapping for AXI4 master interface. It provides a subset of the bits that will be used as a map value"]
pub mod address_mapping_rx;
#[doc = "ADDRESS_MASKING_TX (rw) register accessor: an alias for `Reg<ADDRESS_MASKING_TX_SPEC>`"]
pub type ADDRESS_MASKING_TX = crate::Reg<address_masking_tx::ADDRESS_MASKING_TX_SPEC>;
#[doc = "Address Masking for AXI4 slave interface. It specifies a bitmask. address_masking (i) = 1 : address_out(i) <=address_in(i) address_masking (i) = 0 : address_out(i) <= address_mapping(i) E.g. address_in = 0x70007000 address_masking= 0x0000FFFF address_mapping = 0xABCD0000 address_out = 0xABCD7000"]
pub mod address_masking_tx;
