#[doc = r"Register block"]
#[repr(C)]
pub struct STATUS_REGISTERS {
    #[doc = "0x00 - Constant 32'h70646d61 default value ; \"pdma\" in ascii"]
    pub ip_id: IP_ID,
    #[doc = "0x04 - 32'h00200401 default value Version number Number of available channels AXI data width"]
    pub info: INFO,
    _reserved_2_status0: [u8; 0x04],
    _reserved3: [u8; 0x0c],
    _reserved_3_status_clear: [u8; 0x04],
    _reserved4: [u8; 0x0c],
    _reserved_4_irq_enable: [u8; 0x04],
    _reserved5: [u8; 0x0c],
    #[doc = "0x38 - Configures the state (active high/low) and clearing conditions for the IRQ pin. Cleared: '0' = Use STATUS_CLEAR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high Set all bits IRQ_CONFIG(7 downto 4) of polarity to \"1111\" for IRQ active high or \"0000\" for IRQ active low."]
    pub irq_config: IRQ_CONFIG,
    #[doc = "0x3c - Configures Pad, 10 Bits for read pad configuration 10 bits for write pad configuration."]
    pub pad_config: PAD_CONFIG,
}
impl STATUS_REGISTERS {
    #[doc = "0x08 - Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
    #[inline(always)]
    pub const fn status3(&self) -> &STATUS3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
    #[inline(always)]
    pub const fn status2(&self) -> &STATUS2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x08 - Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
    #[inline(always)]
    pub const fn status0(&self) -> &STATUS0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x18 - Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clear3(&self) -> &STATUS_CLEAR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clear2(&self) -> &STATUS_CLEAR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clear1(&self) -> &STATUS_CLEAR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
    #[inline(always)]
    pub const fn status_clear0(&self) -> &STATUS_CLEAR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x28 - Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_enable3(&self) -> &IRQ_ENABLE3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_enable2(&self) -> &IRQ_ENABLE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_enable1(&self) -> &IRQ_ENABLE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
    #[inline(always)]
    pub const fn irq_enable0(&self) -> &IRQ_ENABLE0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "IP_ID (r) register accessor: an alias for `Reg<IP_ID_SPEC>`"]
pub type IP_ID = crate::Reg<ip_id::IP_ID_SPEC>;
#[doc = "Constant 32'h70646d61 default value ; \"pdma\" in ascii"]
pub mod ip_id;
#[doc = "INFO (r) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "32'h00200401 default value Version number Number of available channels AXI data width"]
pub mod info;
#[doc = "STATUS0 (r) register accessor: an alias for `Reg<STATUS0_SPEC>`"]
pub type STATUS0 = crate::Reg<status0::STATUS0_SPEC>;
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
pub mod status0;
#[doc = "STATUS1 (r) register accessor: an alias for `Reg<STATUS1_SPEC>`"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
pub mod status1;
#[doc = "STATUS2 (r) register accessor: an alias for `Reg<STATUS2_SPEC>`"]
pub type STATUS2 = crate::Reg<status2::STATUS2_SPEC>;
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
pub mod status2;
#[doc = "STATUS3 (r) register accessor: an alias for `Reg<STATUS3_SPEC>`"]
pub type STATUS3 = crate::Reg<status3::STATUS3_SPEC>;
#[doc = "Holds status flags for Channel(0-3). Each Status Flag can either be cleared by writing to the STATUS_CLEAR register or when the STATUS register is read (must be configured) Status bits of Reg Groups should be read by CPU '0' = Reg Group is Idle '1' = Hardware is using specific reg group"]
pub mod status3;
#[doc = "STATUS_CLEAR0 (rw) register accessor: an alias for `Reg<STATUS_CLEAR0_SPEC>`"]
pub type STATUS_CLEAR0 = crate::Reg<status_clear0::STATUS_CLEAR0_SPEC>;
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
pub mod status_clear0;
#[doc = "STATUS_CLEAR1 (rw) register accessor: an alias for `Reg<STATUS_CLEAR1_SPEC>`"]
pub type STATUS_CLEAR1 = crate::Reg<status_clear1::STATUS_CLEAR1_SPEC>;
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
pub mod status_clear1;
#[doc = "STATUS_CLEAR2 (rw) register accessor: an alias for `Reg<STATUS_CLEAR2_SPEC>`"]
pub type STATUS_CLEAR2 = crate::Reg<status_clear2::STATUS_CLEAR2_SPEC>;
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
pub mod status_clear2;
#[doc = "STATUS_CLEAR3 (rw) register accessor: an alias for `Reg<STATUS_CLEAR3_SPEC>`"]
pub type STATUS_CLEAR3 = crate::Reg<status_clear3::STATUS_CLEAR3_SPEC>;
#[doc = "Acknowledging status register for Channels(0-3). Clearing each one by setting corresponding bit"]
pub mod status_clear3;
#[doc = "IRQ_ENABLE0 (rw) register accessor: an alias for `Reg<IRQ_ENABLE0_SPEC>`"]
pub type IRQ_ENABLE0 = crate::Reg<irq_enable0::IRQ_ENABLE0_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_enable0;
#[doc = "IRQ_ENABLE1 (rw) register accessor: an alias for `Reg<IRQ_ENABLE1_SPEC>`"]
pub type IRQ_ENABLE1 = crate::Reg<irq_enable1::IRQ_ENABLE1_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_enable1;
#[doc = "IRQ_ENABLE2 (rw) register accessor: an alias for `Reg<IRQ_ENABLE2_SPEC>`"]
pub type IRQ_ENABLE2 = crate::Reg<irq_enable2::IRQ_ENABLE2_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_enable2;
#[doc = "IRQ_ENABLE3 (rw) register accessor: an alias for `Reg<IRQ_ENABLE3_SPEC>`"]
pub type IRQ_ENABLE3 = crate::Reg<irq_enable3::IRQ_ENABLE3_SPEC>;
#[doc = "Configuring which of the interrupts are routed to the IRQ pin for Channels(0-3). Enabling each intterupt signal by setting corresponding bit. All of the interrupts can be enabled and disabled independent from each other."]
pub mod irq_enable3;
#[doc = "PAD_CONFIG (rw) register accessor: an alias for `Reg<PAD_CONFIG_SPEC>`"]
pub type PAD_CONFIG = crate::Reg<pad_config::PAD_CONFIG_SPEC>;
#[doc = "Configures Pad, 10 Bits for read pad configuration 10 bits for write pad configuration."]
pub mod pad_config;
#[doc = "IRQ_CONFIG (rw) register accessor: an alias for `Reg<IRQ_CONFIG_SPEC>`"]
pub type IRQ_CONFIG = crate::Reg<irq_config::IRQ_CONFIG_SPEC>;
#[doc = "Configures the state (active high/low) and clearing conditions for the IRQ pin. Cleared: '0' = Use STATUS_CLEAR to clear '1' = Auto Clear on Read of STATUS register Polarity: '0' = IRQ active low '1' = IRQ active high Set all bits IRQ_CONFIG(7 downto 4) of polarity to \"1111\" for IRQ active high or \"0000\" for IRQ active low."]
pub mod irq_config;
