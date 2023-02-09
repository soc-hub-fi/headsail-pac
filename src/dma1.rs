#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Status_Registers"]
    pub status_registers: STATUS_REGISTERS,
    #[doc = "0x40..0xb4 - Channel_0_Registers"]
    pub channel_0_registers: CHANNEL_0_REGISTERS,
    _reserved2: [u8; 0x0c],
    #[doc = "0xc0..0x134 - Channel_1_Registers"]
    pub channel_1_registers: CHANNEL_1_REGISTERS,
    _reserved3: [u8; 0x0c],
    #[doc = "0x140..0x1b4 - Channel_2_Registers"]
    pub channel_2_registers: CHANNEL_2_REGISTERS,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c0..0x234 - Channel_3_Registers"]
    pub channel_3_registers: CHANNEL_3_REGISTERS,
}
#[doc = "Status_Registers"]
pub use self::status_registers::STATUS_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Status_Registers"]
pub mod status_registers;
#[doc = "Channel_0_Registers"]
pub use self::channel_0_registers::CHANNEL_0_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Channel_0_Registers"]
pub mod channel_0_registers;
#[doc = "Channel_1_Registers"]
pub use self::channel_1_registers::CHANNEL_1_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Channel_1_Registers"]
pub mod channel_1_registers;
#[doc = "Channel_3_Registers"]
pub use self::channel_3_registers::CHANNEL_3_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Channel_3_Registers"]
pub mod channel_3_registers;
#[doc = "Channel_2_Registers"]
pub use self::channel_2_registers::CHANNEL_2_REGISTERS;
#[doc = r"Cluster"]
#[doc = "Channel_2_Registers"]
pub mod channel_2_registers;
