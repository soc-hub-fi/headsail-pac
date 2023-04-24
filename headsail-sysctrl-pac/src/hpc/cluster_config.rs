#[doc = r"Register block"]
#[repr(C)]
pub struct CLUSTER_CONFIG {
    #[doc = "0x00..0x08 - Core #0 boot address"]
    pub core0_boot_addr: CORE0_BOOT_ADDR,
    #[doc = "0x08..0x10 - Core #1 boot address"]
    pub core1_boot_addr: CORE1_BOOT_ADDR,
    #[doc = "0x10..0x18 - Core #2 boot address"]
    pub core2_boot_addr: CORE2_BOOT_ADDR,
    #[doc = "0x18..0x20 - Core #3 boot address"]
    pub core3_boot_addr: CORE3_BOOT_ADDR,
    #[doc = "0x20..0x28 - Core #0 Hart ID"]
    pub core0_hart_id: CORE0_HART_ID,
    #[doc = "0x28..0x30 - Core #1 Hart ID"]
    pub core1_hart_id: CORE1_HART_ID,
    #[doc = "0x30..0x38 - Core #2 Hart ID"]
    pub core2_hart_id: CORE2_HART_ID,
    #[doc = "0x38..0x40 - Core #3 Hart ID"]
    pub core3_hart_id: CORE3_HART_ID,
    #[doc = "0x40..0x48 - Real-time clock generator clock high count"]
    pub rtc_cfg0: RTC_CFG0,
    #[doc = "0x48..0x50 - Real-time clock generator clock low count"]
    pub rtc_cfg1: RTC_CFG1,
    #[doc = "0x50..0x58 - Real-time clock generator clock fine tune"]
    pub rtc_cfg2: RTC_CFG2,
    _reserved11: [u8; 0x10],
    #[doc = "0x68..0x70 - "]
    pub timer_base: TIMER_BASE,
    #[doc = "0x70..0x78 - "]
    pub timer_len: TIMER_LEN,
    #[doc = "0x78..0x80 - "]
    pub plic_base: PLIC_BASE,
    #[doc = "0x80..0x88 - "]
    pub plic_len: PLIC_LEN,
    #[doc = "0x88..0x90 - "]
    pub clint_base: CLINT_BASE,
    #[doc = "0x90..0x98 - "]
    pub clint_len: CLINT_LEN,
    #[doc = "0x98..0xa0 - "]
    pub rom_base: ROM_BASE,
    #[doc = "0xa0..0xa8 - "]
    pub rom_len: ROM_LEN,
    #[doc = "0xa8..0xb0 - "]
    pub debug_base: DEBUG_BASE,
    #[doc = "0xb0..0xb8 - "]
    pub debug_len: DEBUG_LEN,
    #[doc = "0xb8..0xc0 - "]
    pub l2_config_base: L2_CONFIG_BASE,
    #[doc = "0xc0..0xc8 - "]
    pub l2_config_len: L2_CONFIG_LEN,
    #[doc = "0xc8..0xd0 - "]
    pub cluster_config_base: CLUSTER_CONFIG_BASE,
    #[doc = "0xd0..0xd8 - "]
    pub cluster_config_len: CLUSTER_CONFIG_LEN,
    #[doc = "0xd8..0xe0 - Original memory map calls this dram_base_addr but the register has been repurposed"]
    pub external_memory_base: EXTERNAL_MEMORY_BASE,
    #[doc = "0xe0..0xe8 - Original memory map calls this dram_addr_length but the register has been repurposed"]
    pub external_memory_len: EXTERNAL_MEMORY_LEN,
    #[doc = "0xe8..0xf0 - Valid address space flags"]
    pub addr_valid_rule: ADDR_VALID_RULE,
    #[doc = "0xf0..0xf8 - Number of execute regions"]
    pub nr_exec_regions: NR_EXEC_REGIONS,
    #[doc = "0xf8..0x100 - Base address for execute region #0"]
    pub execute_region_addr_base0: EXECUTE_REGION_ADDR_BASE0,
    #[doc = "0x100..0x108 - Length for execute region #0"]
    pub execute_region_length0: EXECUTE_REGION_LENGTH0,
    #[doc = "0x108..0x110 - Base address for execute region #1"]
    pub execute_region_addr_base1: EXECUTE_REGION_ADDR_BASE1,
    #[doc = "0x110..0x118 - Length for execute region #1"]
    pub execute_region_length1: EXECUTE_REGION_LENGTH1,
    #[doc = "0x118..0x120 - Base address for execute region #2"]
    pub execute_region_addr_base2: EXECUTE_REGION_ADDR_BASE2,
    #[doc = "0x120..0x128 - Length for execute region #2"]
    pub execute_region_length2: EXECUTE_REGION_LENGTH2,
    #[doc = "0x128..0x130 - Base address for execute region #3"]
    pub execute_region_addr_base3: EXECUTE_REGION_ADDR_BASE3,
    #[doc = "0x130..0x138 - Length for execute region #3"]
    pub execute_region_length3: EXECUTE_REGION_LENGTH3,
    #[doc = "0x138..0x140 - Base address for execute region #4"]
    pub execute_region_addr_base4: EXECUTE_REGION_ADDR_BASE4,
    #[doc = "0x140..0x148 - Length for execute region #4"]
    pub execute_region_length4: EXECUTE_REGION_LENGTH4,
    #[doc = "0x148..0x150 - Base address for execute region #5"]
    pub execute_region_addr_base5: EXECUTE_REGION_ADDR_BASE5,
    #[doc = "0x150..0x158 - Length for execute region #5"]
    pub execute_region_length5: EXECUTE_REGION_LENGTH5,
    #[doc = "0x158..0x160 - Base address for execute region #6"]
    pub execute_region_addr_base6: EXECUTE_REGION_ADDR_BASE6,
    #[doc = "0x160..0x168 - Length for execute region #6"]
    pub execute_region_length6: EXECUTE_REGION_LENGTH6,
    #[doc = "0x168..0x170 - Base address for execute region #7"]
    pub execute_region_addr_base7: EXECUTE_REGION_ADDR_BASE7,
    #[doc = "0x170..0x178 - Length for execute region #7"]
    pub execute_region_length7: EXECUTE_REGION_LENGTH7,
    #[doc = "0x178..0x180 - Number of cached regions"]
    pub nr_cached_regions: NR_CACHED_REGIONS,
    #[doc = "0x180..0x188 - Base address for cached region #0 There are 8x 64-bit cached region base address registers (0..=7), spaced 0x10 bytes from each other."]
    pub cached_region_addr_base0: CACHED_REGION_ADDR_BASE0,
    #[doc = "0x188..0x190 - Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other."]
    pub cached_region_addr_length0: CACHED_REGION_ADDR_LENGTH0,
    _reserved48: [u8; 0x70],
    #[doc = "0x200..0x208 - Number of regions with side-effects"]
    pub nr_non_idempotent_regions: NR_NON_IDEMPOTENT_REGIONS,
    #[doc = "0x208..0x210 - Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other."]
    pub non_idempotent_region_addr_base0: NON_IDEMPOTENT_REGION_ADDR_BASE0,
    #[doc = "0x210..0x218 - Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other."]
    pub non_idempotent_region_length0: NON_IDEMPOTENT_REGION_LENGTH0,
    _reserved51: [u8; 0x30],
    #[doc = "0x248..0x250 - "]
    pub l2_div: L2_DIV,
    #[doc = "0x250..0x258 - "]
    pub power_switch_en: POWER_SWITCH_EN,
    #[doc = "0x258..0x260 - "]
    pub power_switch_ack: POWER_SWITCH_ACK,
}
#[doc = "core0_boot_addr (rw) register accessor: an alias for `Reg<CORE0_BOOT_ADDR_SPEC>`"]
pub type CORE0_BOOT_ADDR = crate::Reg<core0_boot_addr::CORE0_BOOT_ADDR_SPEC>;
#[doc = "Core #0 boot address"]
pub mod core0_boot_addr;
#[doc = "core1_boot_addr (rw) register accessor: an alias for `Reg<CORE1_BOOT_ADDR_SPEC>`"]
pub type CORE1_BOOT_ADDR = crate::Reg<core1_boot_addr::CORE1_BOOT_ADDR_SPEC>;
#[doc = "Core #1 boot address"]
pub mod core1_boot_addr;
#[doc = "core2_boot_addr (rw) register accessor: an alias for `Reg<CORE2_BOOT_ADDR_SPEC>`"]
pub type CORE2_BOOT_ADDR = crate::Reg<core2_boot_addr::CORE2_BOOT_ADDR_SPEC>;
#[doc = "Core #2 boot address"]
pub mod core2_boot_addr;
#[doc = "core3_boot_addr (rw) register accessor: an alias for `Reg<CORE3_BOOT_ADDR_SPEC>`"]
pub type CORE3_BOOT_ADDR = crate::Reg<core3_boot_addr::CORE3_BOOT_ADDR_SPEC>;
#[doc = "Core #3 boot address"]
pub mod core3_boot_addr;
#[doc = "execute_region_length7 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH7_SPEC>`"]
pub type EXECUTE_REGION_LENGTH7 = crate::Reg<execute_region_length7::EXECUTE_REGION_LENGTH7_SPEC>;
#[doc = "Length for execute region #7"]
pub mod execute_region_length7;
#[doc = "execute_region_addr_base7 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE7_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE7 =
    crate::Reg<execute_region_addr_base7::EXECUTE_REGION_ADDR_BASE7_SPEC>;
#[doc = "Base address for execute region #7"]
pub mod execute_region_addr_base7;
#[doc = "execute_region_length6 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH6_SPEC>`"]
pub type EXECUTE_REGION_LENGTH6 = crate::Reg<execute_region_length6::EXECUTE_REGION_LENGTH6_SPEC>;
#[doc = "Length for execute region #6"]
pub mod execute_region_length6;
#[doc = "execute_region_addr_base6 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE6_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE6 =
    crate::Reg<execute_region_addr_base6::EXECUTE_REGION_ADDR_BASE6_SPEC>;
#[doc = "Base address for execute region #6"]
pub mod execute_region_addr_base6;
#[doc = "execute_region_length5 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH5_SPEC>`"]
pub type EXECUTE_REGION_LENGTH5 = crate::Reg<execute_region_length5::EXECUTE_REGION_LENGTH5_SPEC>;
#[doc = "Length for execute region #5"]
pub mod execute_region_length5;
#[doc = "execute_region_addr_base5 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE5_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE5 =
    crate::Reg<execute_region_addr_base5::EXECUTE_REGION_ADDR_BASE5_SPEC>;
#[doc = "Base address for execute region #5"]
pub mod execute_region_addr_base5;
#[doc = "power_switch_ack (r) register accessor: an alias for `Reg<POWER_SWITCH_ACK_SPEC>`"]
pub type POWER_SWITCH_ACK = crate::Reg<power_switch_ack::POWER_SWITCH_ACK_SPEC>;
#[doc = ""]
pub mod power_switch_ack;
#[doc = "power_switch_en (rw) register accessor: an alias for `Reg<POWER_SWITCH_EN_SPEC>`"]
pub type POWER_SWITCH_EN = crate::Reg<power_switch_en::POWER_SWITCH_EN_SPEC>;
#[doc = ""]
pub mod power_switch_en;
#[doc = "l2_div (rw) register accessor: an alias for `Reg<L2_DIV_SPEC>`"]
pub type L2_DIV = crate::Reg<l2_div::L2_DIV_SPEC>;
#[doc = ""]
pub mod l2_div;
#[doc = "non_idempotent_region_length0 (rw) register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_LENGTH0_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_LENGTH0 =
    crate::Reg<non_idempotent_region_length0::NON_IDEMPOTENT_REGION_LENGTH0_SPEC>;
#[doc = "Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other."]
pub mod non_idempotent_region_length0;
#[doc = "non_idempotent_region_addr_base0 (rw) register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_ADDR_BASE0_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_ADDR_BASE0 =
    crate::Reg<non_idempotent_region_addr_base0::NON_IDEMPOTENT_REGION_ADDR_BASE0_SPEC>;
#[doc = "Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other."]
pub mod non_idempotent_region_addr_base0;
#[doc = "nr_non_idempotent_regions (rw) register accessor: an alias for `Reg<NR_NON_IDEMPOTENT_REGIONS_SPEC>`"]
pub type NR_NON_IDEMPOTENT_REGIONS =
    crate::Reg<nr_non_idempotent_regions::NR_NON_IDEMPOTENT_REGIONS_SPEC>;
#[doc = "Number of regions with side-effects"]
pub mod nr_non_idempotent_regions;
#[doc = "cached_region_addr_length0 (rw) register accessor: an alias for `Reg<CACHED_REGION_ADDR_LENGTH0_SPEC>`"]
pub type CACHED_REGION_ADDR_LENGTH0 =
    crate::Reg<cached_region_addr_length0::CACHED_REGION_ADDR_LENGTH0_SPEC>;
#[doc = "Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other."]
pub mod cached_region_addr_length0;
#[doc = "cached_region_addr_base0 (rw) register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE0_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE0 =
    crate::Reg<cached_region_addr_base0::CACHED_REGION_ADDR_BASE0_SPEC>;
#[doc = "Base address for cached region #0 There are 8x 64-bit cached region base address registers (0..=7), spaced 0x10 bytes from each other."]
pub mod cached_region_addr_base0;
#[doc = "nr_cached_regions (rw) register accessor: an alias for `Reg<NR_CACHED_REGIONS_SPEC>`"]
pub type NR_CACHED_REGIONS = crate::Reg<nr_cached_regions::NR_CACHED_REGIONS_SPEC>;
#[doc = "Number of cached regions"]
pub mod nr_cached_regions;
#[doc = "execute_region_length4 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH4_SPEC>`"]
pub type EXECUTE_REGION_LENGTH4 = crate::Reg<execute_region_length4::EXECUTE_REGION_LENGTH4_SPEC>;
#[doc = "Length for execute region #4"]
pub mod execute_region_length4;
#[doc = "execute_region_addr_base4 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE4_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE4 =
    crate::Reg<execute_region_addr_base4::EXECUTE_REGION_ADDR_BASE4_SPEC>;
#[doc = "Base address for execute region #4"]
pub mod execute_region_addr_base4;
#[doc = "execute_region_length3 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH3_SPEC>`"]
pub type EXECUTE_REGION_LENGTH3 = crate::Reg<execute_region_length3::EXECUTE_REGION_LENGTH3_SPEC>;
#[doc = "Length for execute region #3"]
pub mod execute_region_length3;
#[doc = "execute_region_addr_base3 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE3_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE3 =
    crate::Reg<execute_region_addr_base3::EXECUTE_REGION_ADDR_BASE3_SPEC>;
#[doc = "Base address for execute region #3"]
pub mod execute_region_addr_base3;
#[doc = "execute_region_length2 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH2_SPEC>`"]
pub type EXECUTE_REGION_LENGTH2 = crate::Reg<execute_region_length2::EXECUTE_REGION_LENGTH2_SPEC>;
#[doc = "Length for execute region #2"]
pub mod execute_region_length2;
#[doc = "execute_region_addr_base2 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE2_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE2 =
    crate::Reg<execute_region_addr_base2::EXECUTE_REGION_ADDR_BASE2_SPEC>;
#[doc = "Base address for execute region #2"]
pub mod execute_region_addr_base2;
#[doc = "execute_region_length1 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH1_SPEC>`"]
pub type EXECUTE_REGION_LENGTH1 = crate::Reg<execute_region_length1::EXECUTE_REGION_LENGTH1_SPEC>;
#[doc = "Length for execute region #1"]
pub mod execute_region_length1;
#[doc = "execute_region_addr_base1 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE1_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE1 =
    crate::Reg<execute_region_addr_base1::EXECUTE_REGION_ADDR_BASE1_SPEC>;
#[doc = "Base address for execute region #1"]
pub mod execute_region_addr_base1;
#[doc = "execute_region_length0 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH0_SPEC>`"]
pub type EXECUTE_REGION_LENGTH0 = crate::Reg<execute_region_length0::EXECUTE_REGION_LENGTH0_SPEC>;
#[doc = "Length for execute region #0"]
pub mod execute_region_length0;
#[doc = "execute_region_addr_base0 (rw) register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE0_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE0 =
    crate::Reg<execute_region_addr_base0::EXECUTE_REGION_ADDR_BASE0_SPEC>;
#[doc = "Base address for execute region #0"]
pub mod execute_region_addr_base0;
#[doc = "nr_exec_regions (rw) register accessor: an alias for `Reg<NR_EXEC_REGIONS_SPEC>`"]
pub type NR_EXEC_REGIONS = crate::Reg<nr_exec_regions::NR_EXEC_REGIONS_SPEC>;
#[doc = "Number of execute regions"]
pub mod nr_exec_regions;
#[doc = "addr_valid_rule (rw) register accessor: an alias for `Reg<ADDR_VALID_RULE_SPEC>`"]
pub type ADDR_VALID_RULE = crate::Reg<addr_valid_rule::ADDR_VALID_RULE_SPEC>;
#[doc = "Valid address space flags"]
pub mod addr_valid_rule;
#[doc = "external_memory_len (rw) register accessor: an alias for `Reg<EXTERNAL_MEMORY_LEN_SPEC>`"]
pub type EXTERNAL_MEMORY_LEN = crate::Reg<external_memory_len::EXTERNAL_MEMORY_LEN_SPEC>;
#[doc = "Original memory map calls this dram_addr_length but the register has been repurposed"]
pub mod external_memory_len;
#[doc = "external_memory_base (rw) register accessor: an alias for `Reg<EXTERNAL_MEMORY_BASE_SPEC>`"]
pub type EXTERNAL_MEMORY_BASE = crate::Reg<external_memory_base::EXTERNAL_MEMORY_BASE_SPEC>;
#[doc = "Original memory map calls this dram_base_addr but the register has been repurposed"]
pub mod external_memory_base;
#[doc = "cluster_config_len (rw) register accessor: an alias for `Reg<CLUSTER_CONFIG_LEN_SPEC>`"]
pub type CLUSTER_CONFIG_LEN = crate::Reg<cluster_config_len::CLUSTER_CONFIG_LEN_SPEC>;
#[doc = ""]
pub mod cluster_config_len;
#[doc = "cluster_config_base (rw) register accessor: an alias for `Reg<CLUSTER_CONFIG_BASE_SPEC>`"]
pub type CLUSTER_CONFIG_BASE = crate::Reg<cluster_config_base::CLUSTER_CONFIG_BASE_SPEC>;
#[doc = ""]
pub mod cluster_config_base;
#[doc = "l2_config_len (rw) register accessor: an alias for `Reg<L2_CONFIG_LEN_SPEC>`"]
pub type L2_CONFIG_LEN = crate::Reg<l2_config_len::L2_CONFIG_LEN_SPEC>;
#[doc = ""]
pub mod l2_config_len;
#[doc = "l2_config_base (rw) register accessor: an alias for `Reg<L2_CONFIG_BASE_SPEC>`"]
pub type L2_CONFIG_BASE = crate::Reg<l2_config_base::L2_CONFIG_BASE_SPEC>;
#[doc = ""]
pub mod l2_config_base;
#[doc = "debug_len (rw) register accessor: an alias for `Reg<DEBUG_LEN_SPEC>`"]
pub type DEBUG_LEN = crate::Reg<debug_len::DEBUG_LEN_SPEC>;
#[doc = ""]
pub mod debug_len;
#[doc = "debug_base (rw) register accessor: an alias for `Reg<DEBUG_BASE_SPEC>`"]
pub type DEBUG_BASE = crate::Reg<debug_base::DEBUG_BASE_SPEC>;
#[doc = ""]
pub mod debug_base;
#[doc = "rom_len (rw) register accessor: an alias for `Reg<ROM_LEN_SPEC>`"]
pub type ROM_LEN = crate::Reg<rom_len::ROM_LEN_SPEC>;
#[doc = ""]
pub mod rom_len;
#[doc = "rom_base (rw) register accessor: an alias for `Reg<ROM_BASE_SPEC>`"]
pub type ROM_BASE = crate::Reg<rom_base::ROM_BASE_SPEC>;
#[doc = ""]
pub mod rom_base;
#[doc = "clint_len (rw) register accessor: an alias for `Reg<CLINT_LEN_SPEC>`"]
pub type CLINT_LEN = crate::Reg<clint_len::CLINT_LEN_SPEC>;
#[doc = ""]
pub mod clint_len;
#[doc = "clint_base (rw) register accessor: an alias for `Reg<CLINT_BASE_SPEC>`"]
pub type CLINT_BASE = crate::Reg<clint_base::CLINT_BASE_SPEC>;
#[doc = ""]
pub mod clint_base;
#[doc = "plic_len (rw) register accessor: an alias for `Reg<PLIC_LEN_SPEC>`"]
pub type PLIC_LEN = crate::Reg<plic_len::PLIC_LEN_SPEC>;
#[doc = ""]
pub mod plic_len;
#[doc = "plic_base (rw) register accessor: an alias for `Reg<PLIC_BASE_SPEC>`"]
pub type PLIC_BASE = crate::Reg<plic_base::PLIC_BASE_SPEC>;
#[doc = ""]
pub mod plic_base;
#[doc = "timer_len (rw) register accessor: an alias for `Reg<TIMER_LEN_SPEC>`"]
pub type TIMER_LEN = crate::Reg<timer_len::TIMER_LEN_SPEC>;
#[doc = ""]
pub mod timer_len;
#[doc = "timer_base (rw) register accessor: an alias for `Reg<TIMER_BASE_SPEC>`"]
pub type TIMER_BASE = crate::Reg<timer_base::TIMER_BASE_SPEC>;
#[doc = ""]
pub mod timer_base;
#[doc = "rtc_cfg2 (rw) register accessor: an alias for `Reg<RTC_CFG2_SPEC>`"]
pub type RTC_CFG2 = crate::Reg<rtc_cfg2::RTC_CFG2_SPEC>;
#[doc = "Real-time clock generator clock fine tune"]
pub mod rtc_cfg2;
#[doc = "rtc_cfg1 (rw) register accessor: an alias for `Reg<RTC_CFG1_SPEC>`"]
pub type RTC_CFG1 = crate::Reg<rtc_cfg1::RTC_CFG1_SPEC>;
#[doc = "Real-time clock generator clock low count"]
pub mod rtc_cfg1;
#[doc = "rtc_cfg0 (rw) register accessor: an alias for `Reg<RTC_CFG0_SPEC>`"]
pub type RTC_CFG0 = crate::Reg<rtc_cfg0::RTC_CFG0_SPEC>;
#[doc = "Real-time clock generator clock high count"]
pub mod rtc_cfg0;
#[doc = "core3_hart_id (r) register accessor: an alias for `Reg<CORE3_HART_ID_SPEC>`"]
pub type CORE3_HART_ID = crate::Reg<core3_hart_id::CORE3_HART_ID_SPEC>;
#[doc = "Core #3 Hart ID"]
pub mod core3_hart_id;
#[doc = "core2_hart_id (r) register accessor: an alias for `Reg<CORE2_HART_ID_SPEC>`"]
pub type CORE2_HART_ID = crate::Reg<core2_hart_id::CORE2_HART_ID_SPEC>;
#[doc = "Core #2 Hart ID"]
pub mod core2_hart_id;
#[doc = "core1_hart_id (r) register accessor: an alias for `Reg<CORE1_HART_ID_SPEC>`"]
pub type CORE1_HART_ID = crate::Reg<core1_hart_id::CORE1_HART_ID_SPEC>;
#[doc = "Core #1 Hart ID"]
pub mod core1_hart_id;
#[doc = "core0_hart_id (r) register accessor: an alias for `Reg<CORE0_HART_ID_SPEC>`"]
pub type CORE0_HART_ID = crate::Reg<core0_hart_id::CORE0_HART_ID_SPEC>;
#[doc = "Core #0 Hart ID"]
pub mod core0_hart_id;
