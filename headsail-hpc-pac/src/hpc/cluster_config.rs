#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "cluster_config"]
#[doc(alias = "cluster_config")]
pub struct ClusterConfig {
    core0_boot_addr: Core0BootAddr,
    core1_boot_addr: Core1BootAddr,
    core2_boot_addr: Core2BootAddr,
    core3_boot_addr: Core3BootAddr,
    core0_hart_id: Core0HartId,
    core1_hart_id: Core1HartId,
    core2_hart_id: Core2HartId,
    core3_hart_id: Core3HartId,
    rtc_cfg0: RtcCfg0,
    rtc_cfg1: RtcCfg1,
    rtc_cfg2: RtcCfg2,
    _reserved11: [u8; 0x10],
    timer_base: TimerBase,
    timer_len: TimerLen,
    plic_base: PlicBase,
    plic_len: PlicLen,
    clint_base: ClintBase,
    clint_len: ClintLen,
    rom_base: RomBase,
    rom_len: RomLen,
    debug_base: DebugBase,
    debug_len: DebugLen,
    l2_config_base: L2ConfigBase,
    l2_config_len: L2ConfigLen,
    cluster_config_base: ClusterConfigBase,
    cluster_config_len: ClusterConfigLen,
    external_memory_base: ExternalMemoryBase,
    external_memory_len: ExternalMemoryLen,
    addr_valid_rule: AddrValidRule,
    nr_exec_regions: NrExecRegions,
    execute_region_addr_base0: ExecuteRegionAddrBase0,
    execute_region_length0: ExecuteRegionLength0,
    execute_region_addr_base1: ExecuteRegionAddrBase1,
    execute_region_length1: ExecuteRegionLength1,
    execute_region_addr_base2: ExecuteRegionAddrBase2,
    execute_region_length2: ExecuteRegionLength2,
    execute_region_addr_base3: ExecuteRegionAddrBase3,
    execute_region_length3: ExecuteRegionLength3,
    execute_region_addr_base4: ExecuteRegionAddrBase4,
    execute_region_length4: ExecuteRegionLength4,
    execute_region_addr_base5: ExecuteRegionAddrBase5,
    execute_region_length5: ExecuteRegionLength5,
    execute_region_addr_base6: ExecuteRegionAddrBase6,
    execute_region_length6: ExecuteRegionLength6,
    execute_region_addr_base7: ExecuteRegionAddrBase7,
    execute_region_length7: ExecuteRegionLength7,
    nr_cached_regions: NrCachedRegions,
    cached_region_addr_base0: CachedRegionAddrBase0,
    cached_region_addr_length0: CachedRegionAddrLength0,
    _reserved48: [u8; 0x70],
    nr_non_idempotent_regions: NrNonIdempotentRegions,
    non_idempotent_region_addr_base0: NonIdempotentRegionAddrBase0,
    non_idempotent_region_length0: NonIdempotentRegionLength0,
    _reserved51: [u8; 0x30],
    l2_div: L2Div,
    power_switch_en: PowerSwitchEn,
    power_switch_ack: PowerSwitchAck,
}
impl ClusterConfig {
    #[doc = "0x00..0x08 - Core #0 boot address"]
    #[inline(always)]
    pub const fn core0_boot_addr(&self) -> &Core0BootAddr {
        &self.core0_boot_addr
    }
    #[doc = "0x08..0x10 - Core #1 boot address"]
    #[inline(always)]
    pub const fn core1_boot_addr(&self) -> &Core1BootAddr {
        &self.core1_boot_addr
    }
    #[doc = "0x10..0x18 - Core #2 boot address"]
    #[inline(always)]
    pub const fn core2_boot_addr(&self) -> &Core2BootAddr {
        &self.core2_boot_addr
    }
    #[doc = "0x18..0x20 - Core #3 boot address"]
    #[inline(always)]
    pub const fn core3_boot_addr(&self) -> &Core3BootAddr {
        &self.core3_boot_addr
    }
    #[doc = "0x20..0x28 - Core #0 Hart ID"]
    #[inline(always)]
    pub const fn core0_hart_id(&self) -> &Core0HartId {
        &self.core0_hart_id
    }
    #[doc = "0x28..0x30 - Core #1 Hart ID"]
    #[inline(always)]
    pub const fn core1_hart_id(&self) -> &Core1HartId {
        &self.core1_hart_id
    }
    #[doc = "0x30..0x38 - Core #2 Hart ID"]
    #[inline(always)]
    pub const fn core2_hart_id(&self) -> &Core2HartId {
        &self.core2_hart_id
    }
    #[doc = "0x38..0x40 - Core #3 Hart ID"]
    #[inline(always)]
    pub const fn core3_hart_id(&self) -> &Core3HartId {
        &self.core3_hart_id
    }
    #[doc = "0x40..0x48 - Real-time clock generator clock high count"]
    #[inline(always)]
    pub const fn rtc_cfg0(&self) -> &RtcCfg0 {
        &self.rtc_cfg0
    }
    #[doc = "0x48..0x50 - Real-time clock generator clock low count"]
    #[inline(always)]
    pub const fn rtc_cfg1(&self) -> &RtcCfg1 {
        &self.rtc_cfg1
    }
    #[doc = "0x50..0x58 - Real-time clock generator clock fine tune"]
    #[inline(always)]
    pub const fn rtc_cfg2(&self) -> &RtcCfg2 {
        &self.rtc_cfg2
    }
    #[doc = "0x68..0x70 - "]
    #[inline(always)]
    pub const fn timer_base(&self) -> &TimerBase {
        &self.timer_base
    }
    #[doc = "0x70..0x78 - "]
    #[inline(always)]
    pub const fn timer_len(&self) -> &TimerLen {
        &self.timer_len
    }
    #[doc = "0x78..0x80 - "]
    #[inline(always)]
    pub const fn plic_base(&self) -> &PlicBase {
        &self.plic_base
    }
    #[doc = "0x80..0x88 - "]
    #[inline(always)]
    pub const fn plic_len(&self) -> &PlicLen {
        &self.plic_len
    }
    #[doc = "0x88..0x90 - "]
    #[inline(always)]
    pub const fn clint_base(&self) -> &ClintBase {
        &self.clint_base
    }
    #[doc = "0x90..0x98 - "]
    #[inline(always)]
    pub const fn clint_len(&self) -> &ClintLen {
        &self.clint_len
    }
    #[doc = "0x98..0xa0 - "]
    #[inline(always)]
    pub const fn rom_base(&self) -> &RomBase {
        &self.rom_base
    }
    #[doc = "0xa0..0xa8 - "]
    #[inline(always)]
    pub const fn rom_len(&self) -> &RomLen {
        &self.rom_len
    }
    #[doc = "0xa8..0xb0 - "]
    #[inline(always)]
    pub const fn debug_base(&self) -> &DebugBase {
        &self.debug_base
    }
    #[doc = "0xb0..0xb8 - "]
    #[inline(always)]
    pub const fn debug_len(&self) -> &DebugLen {
        &self.debug_len
    }
    #[doc = "0xb8..0xc0 - "]
    #[inline(always)]
    pub const fn l2_config_base(&self) -> &L2ConfigBase {
        &self.l2_config_base
    }
    #[doc = "0xc0..0xc8 - "]
    #[inline(always)]
    pub const fn l2_config_len(&self) -> &L2ConfigLen {
        &self.l2_config_len
    }
    #[doc = "0xc8..0xd0 - "]
    #[inline(always)]
    pub const fn cluster_config_base(&self) -> &ClusterConfigBase {
        &self.cluster_config_base
    }
    #[doc = "0xd0..0xd8 - "]
    #[inline(always)]
    pub const fn cluster_config_len(&self) -> &ClusterConfigLen {
        &self.cluster_config_len
    }
    #[doc = "0xd8..0xe0 - Original memory map calls this dram_base_addr but the register has been repurposed"]
    #[inline(always)]
    pub const fn external_memory_base(&self) -> &ExternalMemoryBase {
        &self.external_memory_base
    }
    #[doc = "0xe0..0xe8 - Original memory map calls this dram_addr_length but the register has been repurposed"]
    #[inline(always)]
    pub const fn external_memory_len(&self) -> &ExternalMemoryLen {
        &self.external_memory_len
    }
    #[doc = "0xe8..0xf0 - Valid address space flags"]
    #[inline(always)]
    pub const fn addr_valid_rule(&self) -> &AddrValidRule {
        &self.addr_valid_rule
    }
    #[doc = "0xf0..0xf8 - Number of execute regions"]
    #[inline(always)]
    pub const fn nr_exec_regions(&self) -> &NrExecRegions {
        &self.nr_exec_regions
    }
    #[doc = "0xf8..0x100 - Base address for execute region #0"]
    #[inline(always)]
    pub const fn execute_region_addr_base0(&self) -> &ExecuteRegionAddrBase0 {
        &self.execute_region_addr_base0
    }
    #[doc = "0x100..0x108 - Length for execute region #0"]
    #[inline(always)]
    pub const fn execute_region_length0(&self) -> &ExecuteRegionLength0 {
        &self.execute_region_length0
    }
    #[doc = "0x108..0x110 - Base address for execute region #1"]
    #[inline(always)]
    pub const fn execute_region_addr_base1(&self) -> &ExecuteRegionAddrBase1 {
        &self.execute_region_addr_base1
    }
    #[doc = "0x110..0x118 - Length for execute region #1"]
    #[inline(always)]
    pub const fn execute_region_length1(&self) -> &ExecuteRegionLength1 {
        &self.execute_region_length1
    }
    #[doc = "0x118..0x120 - Base address for execute region #2"]
    #[inline(always)]
    pub const fn execute_region_addr_base2(&self) -> &ExecuteRegionAddrBase2 {
        &self.execute_region_addr_base2
    }
    #[doc = "0x120..0x128 - Length for execute region #2"]
    #[inline(always)]
    pub const fn execute_region_length2(&self) -> &ExecuteRegionLength2 {
        &self.execute_region_length2
    }
    #[doc = "0x128..0x130 - Base address for execute region #3"]
    #[inline(always)]
    pub const fn execute_region_addr_base3(&self) -> &ExecuteRegionAddrBase3 {
        &self.execute_region_addr_base3
    }
    #[doc = "0x130..0x138 - Length for execute region #3"]
    #[inline(always)]
    pub const fn execute_region_length3(&self) -> &ExecuteRegionLength3 {
        &self.execute_region_length3
    }
    #[doc = "0x138..0x140 - Base address for execute region #4"]
    #[inline(always)]
    pub const fn execute_region_addr_base4(&self) -> &ExecuteRegionAddrBase4 {
        &self.execute_region_addr_base4
    }
    #[doc = "0x140..0x148 - Length for execute region #4"]
    #[inline(always)]
    pub const fn execute_region_length4(&self) -> &ExecuteRegionLength4 {
        &self.execute_region_length4
    }
    #[doc = "0x148..0x150 - Base address for execute region #5"]
    #[inline(always)]
    pub const fn execute_region_addr_base5(&self) -> &ExecuteRegionAddrBase5 {
        &self.execute_region_addr_base5
    }
    #[doc = "0x150..0x158 - Length for execute region #5"]
    #[inline(always)]
    pub const fn execute_region_length5(&self) -> &ExecuteRegionLength5 {
        &self.execute_region_length5
    }
    #[doc = "0x158..0x160 - Base address for execute region #6"]
    #[inline(always)]
    pub const fn execute_region_addr_base6(&self) -> &ExecuteRegionAddrBase6 {
        &self.execute_region_addr_base6
    }
    #[doc = "0x160..0x168 - Length for execute region #6"]
    #[inline(always)]
    pub const fn execute_region_length6(&self) -> &ExecuteRegionLength6 {
        &self.execute_region_length6
    }
    #[doc = "0x168..0x170 - Base address for execute region #7"]
    #[inline(always)]
    pub const fn execute_region_addr_base7(&self) -> &ExecuteRegionAddrBase7 {
        &self.execute_region_addr_base7
    }
    #[doc = "0x170..0x178 - Length for execute region #7"]
    #[inline(always)]
    pub const fn execute_region_length7(&self) -> &ExecuteRegionLength7 {
        &self.execute_region_length7
    }
    #[doc = "0x178..0x180 - Number of cached regions"]
    #[inline(always)]
    pub const fn nr_cached_regions(&self) -> &NrCachedRegions {
        &self.nr_cached_regions
    }
    #[doc = "0x180..0x188 - Base address for cached region #0 There are 8x 64-bit cached region base address registers (0..=7), spaced 0x10 bytes from each other."]
    #[inline(always)]
    pub const fn cached_region_addr_base0(&self) -> &CachedRegionAddrBase0 {
        &self.cached_region_addr_base0
    }
    #[doc = "0x188..0x190 - Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other."]
    #[inline(always)]
    pub const fn cached_region_addr_length0(&self) -> &CachedRegionAddrLength0 {
        &self.cached_region_addr_length0
    }
    #[doc = "0x200..0x208 - Number of regions with side-effects"]
    #[inline(always)]
    pub const fn nr_non_idempotent_regions(&self) -> &NrNonIdempotentRegions {
        &self.nr_non_idempotent_regions
    }
    #[doc = "0x208..0x210 - Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other."]
    #[inline(always)]
    pub const fn non_idempotent_region_addr_base0(&self) -> &NonIdempotentRegionAddrBase0 {
        &self.non_idempotent_region_addr_base0
    }
    #[doc = "0x210..0x218 - Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other."]
    #[inline(always)]
    pub const fn non_idempotent_region_length0(&self) -> &NonIdempotentRegionLength0 {
        &self.non_idempotent_region_length0
    }
    #[doc = "0x248..0x250 - "]
    #[inline(always)]
    pub const fn l2_div(&self) -> &L2Div {
        &self.l2_div
    }
    #[doc = "0x250..0x258 - "]
    #[inline(always)]
    pub const fn power_switch_en(&self) -> &PowerSwitchEn {
        &self.power_switch_en
    }
    #[doc = "0x258..0x260 - "]
    #[inline(always)]
    pub const fn power_switch_ack(&self) -> &PowerSwitchAck {
        &self.power_switch_ack
    }
}
#[doc = "core0_boot_addr (rw) register accessor: Core #0 boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_boot_addr`]
module"]
#[doc(alias = "core0_boot_addr")]
pub type Core0BootAddr = crate::Reg<core0_boot_addr::Core0BootAddrSpec>;
#[doc = "Core #0 boot address"]
pub mod core0_boot_addr;
#[doc = "core1_boot_addr (rw) register accessor: Core #1 boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core1_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_boot_addr`]
module"]
#[doc(alias = "core1_boot_addr")]
pub type Core1BootAddr = crate::Reg<core1_boot_addr::Core1BootAddrSpec>;
#[doc = "Core #1 boot address"]
pub mod core1_boot_addr;
#[doc = "core2_boot_addr (rw) register accessor: Core #2 boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core2_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core2_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_boot_addr`]
module"]
#[doc(alias = "core2_boot_addr")]
pub type Core2BootAddr = crate::Reg<core2_boot_addr::Core2BootAddrSpec>;
#[doc = "Core #2 boot address"]
pub mod core2_boot_addr;
#[doc = "core3_boot_addr (rw) register accessor: Core #3 boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core3_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core3_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core3_boot_addr`]
module"]
#[doc(alias = "core3_boot_addr")]
pub type Core3BootAddr = crate::Reg<core3_boot_addr::Core3BootAddrSpec>;
#[doc = "Core #3 boot address"]
pub mod core3_boot_addr;
#[doc = "execute_region_length7 (rw) register accessor: Length for execute region #7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length7`]
module"]
#[doc(alias = "execute_region_length7")]
pub type ExecuteRegionLength7 = crate::Reg<execute_region_length7::ExecuteRegionLength7Spec>;
#[doc = "Length for execute region #7"]
pub mod execute_region_length7;
#[doc = "execute_region_addr_base7 (rw) register accessor: Base address for execute region #7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base7`]
module"]
#[doc(alias = "execute_region_addr_base7")]
pub type ExecuteRegionAddrBase7 = crate::Reg<execute_region_addr_base7::ExecuteRegionAddrBase7Spec>;
#[doc = "Base address for execute region #7"]
pub mod execute_region_addr_base7;
#[doc = "execute_region_length6 (rw) register accessor: Length for execute region #6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length6`]
module"]
#[doc(alias = "execute_region_length6")]
pub type ExecuteRegionLength6 = crate::Reg<execute_region_length6::ExecuteRegionLength6Spec>;
#[doc = "Length for execute region #6"]
pub mod execute_region_length6;
#[doc = "execute_region_addr_base6 (rw) register accessor: Base address for execute region #6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base6`]
module"]
#[doc(alias = "execute_region_addr_base6")]
pub type ExecuteRegionAddrBase6 = crate::Reg<execute_region_addr_base6::ExecuteRegionAddrBase6Spec>;
#[doc = "Base address for execute region #6"]
pub mod execute_region_addr_base6;
#[doc = "execute_region_length5 (rw) register accessor: Length for execute region #5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length5`]
module"]
#[doc(alias = "execute_region_length5")]
pub type ExecuteRegionLength5 = crate::Reg<execute_region_length5::ExecuteRegionLength5Spec>;
#[doc = "Length for execute region #5"]
pub mod execute_region_length5;
#[doc = "execute_region_addr_base5 (rw) register accessor: Base address for execute region #5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base5`]
module"]
#[doc(alias = "execute_region_addr_base5")]
pub type ExecuteRegionAddrBase5 = crate::Reg<execute_region_addr_base5::ExecuteRegionAddrBase5Spec>;
#[doc = "Base address for execute region #5"]
pub mod execute_region_addr_base5;
#[doc = "power_switch_ack (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_switch_ack::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_switch_ack`]
module"]
#[doc(alias = "power_switch_ack")]
pub type PowerSwitchAck = crate::Reg<power_switch_ack::PowerSwitchAckSpec>;
#[doc = ""]
pub mod power_switch_ack;
#[doc = "power_switch_en (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_switch_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_switch_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_switch_en`]
module"]
#[doc(alias = "power_switch_en")]
pub type PowerSwitchEn = crate::Reg<power_switch_en::PowerSwitchEnSpec>;
#[doc = ""]
pub mod power_switch_en;
#[doc = "l2_div (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_div`]
module"]
#[doc(alias = "l2_div")]
pub type L2Div = crate::Reg<l2_div::L2DivSpec>;
#[doc = ""]
pub mod l2_div;
#[doc = "non_idempotent_region_length0 (rw) register accessor: Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_idempotent_region_length0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_idempotent_region_length0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@non_idempotent_region_length0`]
module"]
#[doc(alias = "non_idempotent_region_length0")]
pub type NonIdempotentRegionLength0 =
    crate::Reg<non_idempotent_region_length0::NonIdempotentRegionLength0Spec>;
#[doc = "Length for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region length registers (0..=3), spaced 0x10 bytes from each other."]
pub mod non_idempotent_region_length0;
#[doc = "non_idempotent_region_addr_base0 (rw) register accessor: Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`non_idempotent_region_addr_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`non_idempotent_region_addr_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@non_idempotent_region_addr_base0`]
module"]
#[doc(alias = "non_idempotent_region_addr_base0")]
pub type NonIdempotentRegionAddrBase0 =
    crate::Reg<non_idempotent_region_addr_base0::NonIdempotentRegionAddrBase0Spec>;
#[doc = "Base address for non-idempotent (side-effectful) region #0 There are 4x 64-bit non-idempotent region base address registers (0..=3), spaced 0x10 bytes from each other."]
pub mod non_idempotent_region_addr_base0;
#[doc = "nr_non_idempotent_regions (rw) register accessor: Number of regions with side-effects\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nr_non_idempotent_regions::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nr_non_idempotent_regions::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nr_non_idempotent_regions`]
module"]
#[doc(alias = "nr_non_idempotent_regions")]
pub type NrNonIdempotentRegions = crate::Reg<nr_non_idempotent_regions::NrNonIdempotentRegionsSpec>;
#[doc = "Number of regions with side-effects"]
pub mod nr_non_idempotent_regions;
#[doc = "cached_region_addr_length0 (rw) register accessor: Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cached_region_addr_length0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cached_region_addr_length0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cached_region_addr_length0`]
module"]
#[doc(alias = "cached_region_addr_length0")]
pub type CachedRegionAddrLength0 =
    crate::Reg<cached_region_addr_length0::CachedRegionAddrLength0Spec>;
#[doc = "Length for cached region #0 There are 8x 64-bit cached region length registers (0..=7), spaced 0x10 bytes from each other."]
pub mod cached_region_addr_length0;
#[doc = "cached_region_addr_base0 (rw) register accessor: Base address for cached region #0 There are 8x 64-bit cached region base address registers (0..=7), spaced 0x10 bytes from each other.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cached_region_addr_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cached_region_addr_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cached_region_addr_base0`]
module"]
#[doc(alias = "cached_region_addr_base0")]
pub type CachedRegionAddrBase0 = crate::Reg<cached_region_addr_base0::CachedRegionAddrBase0Spec>;
#[doc = "Base address for cached region #0 There are 8x 64-bit cached region base address registers (0..=7), spaced 0x10 bytes from each other."]
pub mod cached_region_addr_base0;
#[doc = "nr_cached_regions (rw) register accessor: Number of cached regions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nr_cached_regions::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nr_cached_regions::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nr_cached_regions`]
module"]
#[doc(alias = "nr_cached_regions")]
pub type NrCachedRegions = crate::Reg<nr_cached_regions::NrCachedRegionsSpec>;
#[doc = "Number of cached regions"]
pub mod nr_cached_regions;
#[doc = "execute_region_length4 (rw) register accessor: Length for execute region #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length4`]
module"]
#[doc(alias = "execute_region_length4")]
pub type ExecuteRegionLength4 = crate::Reg<execute_region_length4::ExecuteRegionLength4Spec>;
#[doc = "Length for execute region #4"]
pub mod execute_region_length4;
#[doc = "execute_region_addr_base4 (rw) register accessor: Base address for execute region #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base4`]
module"]
#[doc(alias = "execute_region_addr_base4")]
pub type ExecuteRegionAddrBase4 = crate::Reg<execute_region_addr_base4::ExecuteRegionAddrBase4Spec>;
#[doc = "Base address for execute region #4"]
pub mod execute_region_addr_base4;
#[doc = "execute_region_length3 (rw) register accessor: Length for execute region #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length3`]
module"]
#[doc(alias = "execute_region_length3")]
pub type ExecuteRegionLength3 = crate::Reg<execute_region_length3::ExecuteRegionLength3Spec>;
#[doc = "Length for execute region #3"]
pub mod execute_region_length3;
#[doc = "execute_region_addr_base3 (rw) register accessor: Base address for execute region #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base3`]
module"]
#[doc(alias = "execute_region_addr_base3")]
pub type ExecuteRegionAddrBase3 = crate::Reg<execute_region_addr_base3::ExecuteRegionAddrBase3Spec>;
#[doc = "Base address for execute region #3"]
pub mod execute_region_addr_base3;
#[doc = "execute_region_length2 (rw) register accessor: Length for execute region #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length2`]
module"]
#[doc(alias = "execute_region_length2")]
pub type ExecuteRegionLength2 = crate::Reg<execute_region_length2::ExecuteRegionLength2Spec>;
#[doc = "Length for execute region #2"]
pub mod execute_region_length2;
#[doc = "execute_region_addr_base2 (rw) register accessor: Base address for execute region #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base2`]
module"]
#[doc(alias = "execute_region_addr_base2")]
pub type ExecuteRegionAddrBase2 = crate::Reg<execute_region_addr_base2::ExecuteRegionAddrBase2Spec>;
#[doc = "Base address for execute region #2"]
pub mod execute_region_addr_base2;
#[doc = "execute_region_length1 (rw) register accessor: Length for execute region #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length1`]
module"]
#[doc(alias = "execute_region_length1")]
pub type ExecuteRegionLength1 = crate::Reg<execute_region_length1::ExecuteRegionLength1Spec>;
#[doc = "Length for execute region #1"]
pub mod execute_region_length1;
#[doc = "execute_region_addr_base1 (rw) register accessor: Base address for execute region #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base1`]
module"]
#[doc(alias = "execute_region_addr_base1")]
pub type ExecuteRegionAddrBase1 = crate::Reg<execute_region_addr_base1::ExecuteRegionAddrBase1Spec>;
#[doc = "Base address for execute region #1"]
pub mod execute_region_addr_base1;
#[doc = "execute_region_length0 (rw) register accessor: Length for execute region #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_length0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_length0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_length0`]
module"]
#[doc(alias = "execute_region_length0")]
pub type ExecuteRegionLength0 = crate::Reg<execute_region_length0::ExecuteRegionLength0Spec>;
#[doc = "Length for execute region #0"]
pub mod execute_region_length0;
#[doc = "execute_region_addr_base0 (rw) register accessor: Base address for execute region #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`execute_region_addr_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`execute_region_addr_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@execute_region_addr_base0`]
module"]
#[doc(alias = "execute_region_addr_base0")]
pub type ExecuteRegionAddrBase0 = crate::Reg<execute_region_addr_base0::ExecuteRegionAddrBase0Spec>;
#[doc = "Base address for execute region #0"]
pub mod execute_region_addr_base0;
#[doc = "nr_exec_regions (rw) register accessor: Number of execute regions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nr_exec_regions::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nr_exec_regions::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nr_exec_regions`]
module"]
#[doc(alias = "nr_exec_regions")]
pub type NrExecRegions = crate::Reg<nr_exec_regions::NrExecRegionsSpec>;
#[doc = "Number of execute regions"]
pub mod nr_exec_regions;
#[doc = "addr_valid_rule (rw) register accessor: Valid address space flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_valid_rule::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_valid_rule::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_valid_rule`]
module"]
#[doc(alias = "addr_valid_rule")]
pub type AddrValidRule = crate::Reg<addr_valid_rule::AddrValidRuleSpec>;
#[doc = "Valid address space flags"]
pub mod addr_valid_rule;
#[doc = "external_memory_len (rw) register accessor: Original memory map calls this dram_addr_length but the register has been repurposed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`external_memory_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_memory_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@external_memory_len`]
module"]
#[doc(alias = "external_memory_len")]
pub type ExternalMemoryLen = crate::Reg<external_memory_len::ExternalMemoryLenSpec>;
#[doc = "Original memory map calls this dram_addr_length but the register has been repurposed"]
pub mod external_memory_len;
#[doc = "external_memory_base (rw) register accessor: Original memory map calls this dram_base_addr but the register has been repurposed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`external_memory_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_memory_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@external_memory_base`]
module"]
#[doc(alias = "external_memory_base")]
pub type ExternalMemoryBase = crate::Reg<external_memory_base::ExternalMemoryBaseSpec>;
#[doc = "Original memory map calls this dram_base_addr but the register has been repurposed"]
pub mod external_memory_base;
#[doc = "cluster_config_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cluster_config_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cluster_config_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cluster_config_len`]
module"]
#[doc(alias = "cluster_config_len")]
pub type ClusterConfigLen = crate::Reg<cluster_config_len::ClusterConfigLenSpec>;
#[doc = ""]
pub mod cluster_config_len;
#[doc = "cluster_config_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cluster_config_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cluster_config_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cluster_config_base`]
module"]
#[doc(alias = "cluster_config_base")]
pub type ClusterConfigBase = crate::Reg<cluster_config_base::ClusterConfigBaseSpec>;
#[doc = ""]
pub mod cluster_config_base;
#[doc = "l2_config_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_config_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_config_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_config_len`]
module"]
#[doc(alias = "l2_config_len")]
pub type L2ConfigLen = crate::Reg<l2_config_len::L2ConfigLenSpec>;
#[doc = ""]
pub mod l2_config_len;
#[doc = "l2_config_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_config_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_config_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_config_base`]
module"]
#[doc(alias = "l2_config_base")]
pub type L2ConfigBase = crate::Reg<l2_config_base::L2ConfigBaseSpec>;
#[doc = ""]
pub mod l2_config_base;
#[doc = "debug_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_len`]
module"]
#[doc(alias = "debug_len")]
pub type DebugLen = crate::Reg<debug_len::DebugLenSpec>;
#[doc = ""]
pub mod debug_len;
#[doc = "debug_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_base`]
module"]
#[doc(alias = "debug_base")]
pub type DebugBase = crate::Reg<debug_base::DebugBaseSpec>;
#[doc = ""]
pub mod debug_base;
#[doc = "rom_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_len`]
module"]
#[doc(alias = "rom_len")]
pub type RomLen = crate::Reg<rom_len::RomLenSpec>;
#[doc = ""]
pub mod rom_len;
#[doc = "rom_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_base`]
module"]
#[doc(alias = "rom_base")]
pub type RomBase = crate::Reg<rom_base::RomBaseSpec>;
#[doc = ""]
pub mod rom_base;
#[doc = "clint_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clint_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clint_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clint_len`]
module"]
#[doc(alias = "clint_len")]
pub type ClintLen = crate::Reg<clint_len::ClintLenSpec>;
#[doc = ""]
pub mod clint_len;
#[doc = "clint_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clint_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clint_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clint_base`]
module"]
#[doc(alias = "clint_base")]
pub type ClintBase = crate::Reg<clint_base::ClintBaseSpec>;
#[doc = ""]
pub mod clint_base;
#[doc = "plic_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plic_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plic_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_len`]
module"]
#[doc(alias = "plic_len")]
pub type PlicLen = crate::Reg<plic_len::PlicLenSpec>;
#[doc = ""]
pub mod plic_len;
#[doc = "plic_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plic_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plic_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plic_base`]
module"]
#[doc(alias = "plic_base")]
pub type PlicBase = crate::Reg<plic_base::PlicBaseSpec>;
#[doc = ""]
pub mod plic_base;
#[doc = "timer_len (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_len`]
module"]
#[doc(alias = "timer_len")]
pub type TimerLen = crate::Reg<timer_len::TimerLenSpec>;
#[doc = ""]
pub mod timer_len;
#[doc = "timer_base (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_base`]
module"]
#[doc(alias = "timer_base")]
pub type TimerBase = crate::Reg<timer_base::TimerBaseSpec>;
#[doc = ""]
pub mod timer_base;
#[doc = "rtc_cfg2 (rw) register accessor: Real-time clock generator clock fine tune\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cfg2`]
module"]
#[doc(alias = "rtc_cfg2")]
pub type RtcCfg2 = crate::Reg<rtc_cfg2::RtcCfg2Spec>;
#[doc = "Real-time clock generator clock fine tune"]
pub mod rtc_cfg2;
#[doc = "rtc_cfg1 (rw) register accessor: Real-time clock generator clock low count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cfg1`]
module"]
#[doc(alias = "rtc_cfg1")]
pub type RtcCfg1 = crate::Reg<rtc_cfg1::RtcCfg1Spec>;
#[doc = "Real-time clock generator clock low count"]
pub mod rtc_cfg1;
#[doc = "rtc_cfg0 (rw) register accessor: Real-time clock generator clock high count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cfg0`]
module"]
#[doc(alias = "rtc_cfg0")]
pub type RtcCfg0 = crate::Reg<rtc_cfg0::RtcCfg0Spec>;
#[doc = "Real-time clock generator clock high count"]
pub mod rtc_cfg0;
#[doc = "core3_hart_id (r) register accessor: Core #3 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core3_hart_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core3_hart_id`]
module"]
#[doc(alias = "core3_hart_id")]
pub type Core3HartId = crate::Reg<core3_hart_id::Core3HartIdSpec>;
#[doc = "Core #3 Hart ID"]
pub mod core3_hart_id;
#[doc = "core2_hart_id (r) register accessor: Core #2 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core2_hart_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core2_hart_id`]
module"]
#[doc(alias = "core2_hart_id")]
pub type Core2HartId = crate::Reg<core2_hart_id::Core2HartIdSpec>;
#[doc = "Core #2 Hart ID"]
pub mod core2_hart_id;
#[doc = "core1_hart_id (r) register accessor: Core #1 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_hart_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_hart_id`]
module"]
#[doc(alias = "core1_hart_id")]
pub type Core1HartId = crate::Reg<core1_hart_id::Core1HartIdSpec>;
#[doc = "Core #1 Hart ID"]
pub mod core1_hart_id;
#[doc = "core0_hart_id (r) register accessor: Core #0 Hart ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_hart_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_hart_id`]
module"]
#[doc(alias = "core0_hart_id")]
pub type Core0HartId = crate::Reg<core0_hart_id::Core0HartIdSpec>;
#[doc = "Core #0 Hart ID"]
pub mod core0_hart_id;
