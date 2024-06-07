#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "AdvancedTimer"]
pub struct AdvancedTimer {
    t0_cmd: T0Cmd,
    t0_config: T0Config,
    t0_threshold: T0Threshold,
    t0_th_channel0: T0ThChannel0,
    t0_th_channel1: T0ThChannel1,
    t0_th_channel2: T0ThChannel2,
    t0_th_channel3: T0ThChannel3,
    _reserved7: [u8; 0x10],
    t0_counter: T0Counter,
    _reserved8: [u8; 0x10],
    t1_cmd: T1Cmd,
    t1_config: T1Config,
    t1_threshold: T1Threshold,
    t1_th_channel0: T1ThChannel0,
    t1_th_channel1: T1ThChannel1,
    t1_th_channel2: T1ThChannel2,
    t1_th_channel3: T1ThChannel3,
    _reserved15: [u8; 0x10],
    t1_counter: T1Counter,
    _reserved16: [u8; 0x10],
    t2_cmd: T2Cmd,
    t2_config: T2Config,
    t2_threshold: T2Threshold,
    t2_th_channel0: T2ThChannel0,
    t2_th_channel1: T2ThChannel1,
    t2_th_channel2: T2ThChannel2,
    t2_th_channel3: T2ThChannel3,
    _reserved23: [u8; 0x10],
    t2_counter: T2Counter,
    _reserved24: [u8; 0x10],
    t3_cmd: T3Cmd,
    t3_config: T3Config,
    t3_threshold: T3Threshold,
    t3_th_channel0: T3ThChannel0,
    t3_th_channel1: T3ThChannel1,
    t3_th_channel2: T3ThChannel2,
    t3_th_channel3: T3ThChannel3,
    _reserved31: [u8; 0x10],
    t3_counter: T3Counter,
    _reserved32: [u8; 0x10],
    event_cfg: EventCfg,
    cg: Cg,
}
impl AdvancedTimer {
    #[doc = "0x00 - ADV_TIMER0 command register"]
    #[inline(always)]
    pub const fn t0_cmd(&self) -> &T0Cmd {
        &self.t0_cmd
    }
    #[doc = "0x04 - ADV_TIMER0 configuration register."]
    #[inline(always)]
    pub const fn t0_config(&self) -> &T0Config {
        &self.t0_config
    }
    #[doc = "0x08 - ADV_TIMER0 threshold configuration register."]
    #[inline(always)]
    pub const fn t0_threshold(&self) -> &T0Threshold {
        &self.t0_threshold
    }
    #[doc = "0x0c - ADV_TIMER0 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel0(&self) -> &T0ThChannel0 {
        &self.t0_th_channel0
    }
    #[doc = "0x10 - ADV_TIMER0 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel1(&self) -> &T0ThChannel1 {
        &self.t0_th_channel1
    }
    #[doc = "0x14 - ADV_TIMER0 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel2(&self) -> &T0ThChannel2 {
        &self.t0_th_channel2
    }
    #[doc = "0x18 - ADV_TIMER0 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t0_th_channel3(&self) -> &T0ThChannel3 {
        &self.t0_th_channel3
    }
    #[doc = "0x2c - ADV_TIMER0 counter register"]
    #[inline(always)]
    pub const fn t0_counter(&self) -> &T0Counter {
        &self.t0_counter
    }
    #[doc = "0x40 - ADV_TIMER1 command register"]
    #[inline(always)]
    pub const fn t1_cmd(&self) -> &T1Cmd {
        &self.t1_cmd
    }
    #[doc = "0x44 - ADV_TIMER1 configuration register"]
    #[inline(always)]
    pub const fn t1_config(&self) -> &T1Config {
        &self.t1_config
    }
    #[doc = "0x48 - ADV_TIMER1 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_threshold(&self) -> &T1Threshold {
        &self.t1_threshold
    }
    #[doc = "0x4c - ADV_TIMER1 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel0(&self) -> &T1ThChannel0 {
        &self.t1_th_channel0
    }
    #[doc = "0x50 - ADV_TIMER1 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel1(&self) -> &T1ThChannel1 {
        &self.t1_th_channel1
    }
    #[doc = "0x54 - ADV_TIMER1 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel2(&self) -> &T1ThChannel2 {
        &self.t1_th_channel2
    }
    #[doc = "0x58 - ADV_TIMER1 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t1_th_channel3(&self) -> &T1ThChannel3 {
        &self.t1_th_channel3
    }
    #[doc = "0x6c - ADV_TIMER1 counter register"]
    #[inline(always)]
    pub const fn t1_counter(&self) -> &T1Counter {
        &self.t1_counter
    }
    #[doc = "0x80 - ADV_TIMER2 command register"]
    #[inline(always)]
    pub const fn t2_cmd(&self) -> &T2Cmd {
        &self.t2_cmd
    }
    #[doc = "0x84 - ADV_TIMER2 configuration register"]
    #[inline(always)]
    pub const fn t2_config(&self) -> &T2Config {
        &self.t2_config
    }
    #[doc = "0x88 - ADV_TIMER2 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_threshold(&self) -> &T2Threshold {
        &self.t2_threshold
    }
    #[doc = "0x8c - ADV_TIMER2 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel0(&self) -> &T2ThChannel0 {
        &self.t2_th_channel0
    }
    #[doc = "0x90 - ADV_TIMER2 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel1(&self) -> &T2ThChannel1 {
        &self.t2_th_channel1
    }
    #[doc = "0x94 - ADV_TIMER2 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel2(&self) -> &T2ThChannel2 {
        &self.t2_th_channel2
    }
    #[doc = "0x98 - ADV_TIMER2 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t2_th_channel3(&self) -> &T2ThChannel3 {
        &self.t2_th_channel3
    }
    #[doc = "0xac - ADV_TIMER2 counter register"]
    #[inline(always)]
    pub const fn t2_counter(&self) -> &T2Counter {
        &self.t2_counter
    }
    #[doc = "0xc0 - ADV_TIMER3 command register"]
    #[inline(always)]
    pub const fn t3_cmd(&self) -> &T3Cmd {
        &self.t3_cmd
    }
    #[doc = "0xc4 - ADV_TIMER3 configuration register"]
    #[inline(always)]
    pub const fn t3_config(&self) -> &T3Config {
        &self.t3_config
    }
    #[doc = "0xc8 - ADV_TIMER3 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_threshold(&self) -> &T3Threshold {
        &self.t3_threshold
    }
    #[doc = "0xcc - ADV_TIMER3 channel 0 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel0(&self) -> &T3ThChannel0 {
        &self.t3_th_channel0
    }
    #[doc = "0xd0 - ADV_TIMER3 channel 1 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel1(&self) -> &T3ThChannel1 {
        &self.t3_th_channel1
    }
    #[doc = "0xd4 - ADV_TIMER3 channel 2 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel2(&self) -> &T3ThChannel2 {
        &self.t3_th_channel2
    }
    #[doc = "0xd8 - ADV_TIMER3 channel 3 threshold configuration register"]
    #[inline(always)]
    pub const fn t3_th_channel3(&self) -> &T3ThChannel3 {
        &self.t3_th_channel3
    }
    #[doc = "0xec - ADV_TIMER3 counter register"]
    #[inline(always)]
    pub const fn t3_counter(&self) -> &T3Counter {
        &self.t3_counter
    }
    #[doc = "0x100 - ADV_TIMERS events configuration register."]
    #[inline(always)]
    pub const fn event_cfg(&self) -> &EventCfg {
        &self.event_cfg
    }
    #[doc = "0x104 - ADV_TIMERS channels clock gating configuration register."]
    #[inline(always)]
    pub const fn cg(&self) -> &Cg {
        &self.cg
    }
}
#[doc = "T0_CMD (rw) register accessor: ADV_TIMER0 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_cmd`]
module"]
#[doc(alias = "T0_CMD")]
pub type T0Cmd = crate::Reg<t0_cmd::T0CmdSpec>;
#[doc = "ADV_TIMER0 command register"]
pub mod t0_cmd;
#[doc = "T0_CONFIG (rw) register accessor: ADV_TIMER0 configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_config`]
module"]
#[doc(alias = "T0_CONFIG")]
pub type T0Config = crate::Reg<t0_config::T0ConfigSpec>;
#[doc = "ADV_TIMER0 configuration register."]
pub mod t0_config;
#[doc = "T0_THRESHOLD (rw) register accessor: ADV_TIMER0 threshold configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_threshold`]
module"]
#[doc(alias = "T0_THRESHOLD")]
pub type T0Threshold = crate::Reg<t0_threshold::T0ThresholdSpec>;
#[doc = "ADV_TIMER0 threshold configuration register."]
pub mod t0_threshold;
#[doc = "T0_TH_CHANNEL0 (rw) register accessor: ADV_TIMER0 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel0`]
module"]
#[doc(alias = "T0_TH_CHANNEL0")]
pub type T0ThChannel0 = crate::Reg<t0_th_channel0::T0ThChannel0Spec>;
#[doc = "ADV_TIMER0 channel 0 threshold configuration register"]
pub mod t0_th_channel0;
#[doc = "T0_TH_CHANNEL1 (rw) register accessor: ADV_TIMER0 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel1`]
module"]
#[doc(alias = "T0_TH_CHANNEL1")]
pub type T0ThChannel1 = crate::Reg<t0_th_channel1::T0ThChannel1Spec>;
#[doc = "ADV_TIMER0 channel 1 threshold configuration register"]
pub mod t0_th_channel1;
#[doc = "T0_TH_CHANNEL2 (rw) register accessor: ADV_TIMER0 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel2`]
module"]
#[doc(alias = "T0_TH_CHANNEL2")]
pub type T0ThChannel2 = crate::Reg<t0_th_channel2::T0ThChannel2Spec>;
#[doc = "ADV_TIMER0 channel 2 threshold configuration register"]
pub mod t0_th_channel2;
#[doc = "T0_TH_CHANNEL3 (rw) register accessor: ADV_TIMER0 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_th_channel3`]
module"]
#[doc(alias = "T0_TH_CHANNEL3")]
pub type T0ThChannel3 = crate::Reg<t0_th_channel3::T0ThChannel3Spec>;
#[doc = "ADV_TIMER0 channel 3 threshold configuration register"]
pub mod t0_th_channel3;
#[doc = "T0_COUNTER (r) register accessor: ADV_TIMER0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_counter`]
module"]
#[doc(alias = "T0_COUNTER")]
pub type T0Counter = crate::Reg<t0_counter::T0CounterSpec>;
#[doc = "ADV_TIMER0 counter register"]
pub mod t0_counter;
#[doc = "T1_CMD (rw) register accessor: ADV_TIMER1 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_cmd`]
module"]
#[doc(alias = "T1_CMD")]
pub type T1Cmd = crate::Reg<t1_cmd::T1CmdSpec>;
#[doc = "ADV_TIMER1 command register"]
pub mod t1_cmd;
#[doc = "T1_CONFIG (rw) register accessor: ADV_TIMER1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_config`]
module"]
#[doc(alias = "T1_CONFIG")]
pub type T1Config = crate::Reg<t1_config::T1ConfigSpec>;
#[doc = "ADV_TIMER1 configuration register"]
pub mod t1_config;
#[doc = "T1_THRESHOLD (rw) register accessor: ADV_TIMER1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_threshold`]
module"]
#[doc(alias = "T1_THRESHOLD")]
pub type T1Threshold = crate::Reg<t1_threshold::T1ThresholdSpec>;
#[doc = "ADV_TIMER1 threshold configuration register"]
pub mod t1_threshold;
#[doc = "T1_TH_CHANNEL0 (rw) register accessor: ADV_TIMER1 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel0`]
module"]
#[doc(alias = "T1_TH_CHANNEL0")]
pub type T1ThChannel0 = crate::Reg<t1_th_channel0::T1ThChannel0Spec>;
#[doc = "ADV_TIMER1 channel 0 threshold configuration register"]
pub mod t1_th_channel0;
#[doc = "T1_TH_CHANNEL1 (rw) register accessor: ADV_TIMER1 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel1`]
module"]
#[doc(alias = "T1_TH_CHANNEL1")]
pub type T1ThChannel1 = crate::Reg<t1_th_channel1::T1ThChannel1Spec>;
#[doc = "ADV_TIMER1 channel 1 threshold configuration register"]
pub mod t1_th_channel1;
#[doc = "T1_TH_CHANNEL2 (rw) register accessor: ADV_TIMER1 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel2`]
module"]
#[doc(alias = "T1_TH_CHANNEL2")]
pub type T1ThChannel2 = crate::Reg<t1_th_channel2::T1ThChannel2Spec>;
#[doc = "ADV_TIMER1 channel 2 threshold configuration register"]
pub mod t1_th_channel2;
#[doc = "T1_TH_CHANNEL3 (rw) register accessor: ADV_TIMER1 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_th_channel3`]
module"]
#[doc(alias = "T1_TH_CHANNEL3")]
pub type T1ThChannel3 = crate::Reg<t1_th_channel3::T1ThChannel3Spec>;
#[doc = "ADV_TIMER1 channel 3 threshold configuration register"]
pub mod t1_th_channel3;
#[doc = "T1_COUNTER (r) register accessor: ADV_TIMER1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1_counter`]
module"]
#[doc(alias = "T1_COUNTER")]
pub type T1Counter = crate::Reg<t1_counter::T1CounterSpec>;
#[doc = "ADV_TIMER1 counter register"]
pub mod t1_counter;
#[doc = "T2_CMD (rw) register accessor: ADV_TIMER2 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_cmd`]
module"]
#[doc(alias = "T2_CMD")]
pub type T2Cmd = crate::Reg<t2_cmd::T2CmdSpec>;
#[doc = "ADV_TIMER2 command register"]
pub mod t2_cmd;
#[doc = "T2_CONFIG (rw) register accessor: ADV_TIMER2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_config`]
module"]
#[doc(alias = "T2_CONFIG")]
pub type T2Config = crate::Reg<t2_config::T2ConfigSpec>;
#[doc = "ADV_TIMER2 configuration register"]
pub mod t2_config;
#[doc = "T2_THRESHOLD (rw) register accessor: ADV_TIMER2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_threshold`]
module"]
#[doc(alias = "T2_THRESHOLD")]
pub type T2Threshold = crate::Reg<t2_threshold::T2ThresholdSpec>;
#[doc = "ADV_TIMER2 threshold configuration register"]
pub mod t2_threshold;
#[doc = "T2_TH_CHANNEL0 (rw) register accessor: ADV_TIMER2 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel0`]
module"]
#[doc(alias = "T2_TH_CHANNEL0")]
pub type T2ThChannel0 = crate::Reg<t2_th_channel0::T2ThChannel0Spec>;
#[doc = "ADV_TIMER2 channel 0 threshold configuration register"]
pub mod t2_th_channel0;
#[doc = "T2_TH_CHANNEL1 (rw) register accessor: ADV_TIMER2 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel1`]
module"]
#[doc(alias = "T2_TH_CHANNEL1")]
pub type T2ThChannel1 = crate::Reg<t2_th_channel1::T2ThChannel1Spec>;
#[doc = "ADV_TIMER2 channel 1 threshold configuration register"]
pub mod t2_th_channel1;
#[doc = "T2_TH_CHANNEL2 (rw) register accessor: ADV_TIMER2 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel2`]
module"]
#[doc(alias = "T2_TH_CHANNEL2")]
pub type T2ThChannel2 = crate::Reg<t2_th_channel2::T2ThChannel2Spec>;
#[doc = "ADV_TIMER2 channel 2 threshold configuration register"]
pub mod t2_th_channel2;
#[doc = "T2_TH_CHANNEL3 (rw) register accessor: ADV_TIMER2 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_th_channel3`]
module"]
#[doc(alias = "T2_TH_CHANNEL3")]
pub type T2ThChannel3 = crate::Reg<t2_th_channel3::T2ThChannel3Spec>;
#[doc = "ADV_TIMER2 channel 3 threshold configuration register"]
pub mod t2_th_channel3;
#[doc = "T2_COUNTER (rw) register accessor: ADV_TIMER2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t2_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t2_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2_counter`]
module"]
#[doc(alias = "T2_COUNTER")]
pub type T2Counter = crate::Reg<t2_counter::T2CounterSpec>;
#[doc = "ADV_TIMER2 counter register"]
pub mod t2_counter;
#[doc = "T3_CMD (rw) register accessor: ADV_TIMER3 command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_cmd`]
module"]
#[doc(alias = "T3_CMD")]
pub type T3Cmd = crate::Reg<t3_cmd::T3CmdSpec>;
#[doc = "ADV_TIMER3 command register"]
pub mod t3_cmd;
#[doc = "T3_CONFIG (rw) register accessor: ADV_TIMER3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_config`]
module"]
#[doc(alias = "T3_CONFIG")]
pub type T3Config = crate::Reg<t3_config::T3ConfigSpec>;
#[doc = "ADV_TIMER3 configuration register"]
pub mod t3_config;
#[doc = "T3_THRESHOLD (rw) register accessor: ADV_TIMER3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_threshold`]
module"]
#[doc(alias = "T3_THRESHOLD")]
pub type T3Threshold = crate::Reg<t3_threshold::T3ThresholdSpec>;
#[doc = "ADV_TIMER3 threshold configuration register"]
pub mod t3_threshold;
#[doc = "T3_TH_CHANNEL0 (rw) register accessor: ADV_TIMER3 channel 0 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel0`]
module"]
#[doc(alias = "T3_TH_CHANNEL0")]
pub type T3ThChannel0 = crate::Reg<t3_th_channel0::T3ThChannel0Spec>;
#[doc = "ADV_TIMER3 channel 0 threshold configuration register"]
pub mod t3_th_channel0;
#[doc = "T3_TH_CHANNEL1 (rw) register accessor: ADV_TIMER3 channel 1 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel1`]
module"]
#[doc(alias = "T3_TH_CHANNEL1")]
pub type T3ThChannel1 = crate::Reg<t3_th_channel1::T3ThChannel1Spec>;
#[doc = "ADV_TIMER3 channel 1 threshold configuration register"]
pub mod t3_th_channel1;
#[doc = "T3_TH_CHANNEL2 (rw) register accessor: ADV_TIMER3 channel 2 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel2`]
module"]
#[doc(alias = "T3_TH_CHANNEL2")]
pub type T3ThChannel2 = crate::Reg<t3_th_channel2::T3ThChannel2Spec>;
#[doc = "ADV_TIMER3 channel 2 threshold configuration register"]
pub mod t3_th_channel2;
#[doc = "T3_TH_CHANNEL3 (rw) register accessor: ADV_TIMER3 channel 3 threshold configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_th_channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t3_th_channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_th_channel3`]
module"]
#[doc(alias = "T3_TH_CHANNEL3")]
pub type T3ThChannel3 = crate::Reg<t3_th_channel3::T3ThChannel3Spec>;
#[doc = "ADV_TIMER3 channel 3 threshold configuration register"]
pub mod t3_th_channel3;
#[doc = "CG (rw) register accessor: ADV_TIMERS channels clock gating configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cg`]
module"]
#[doc(alias = "CG")]
pub type Cg = crate::Reg<cg::CgSpec>;
#[doc = "ADV_TIMERS channels clock gating configuration register."]
pub mod cg;
#[doc = "T3_COUNTER (r) register accessor: ADV_TIMER3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t3_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3_counter`]
module"]
#[doc(alias = "T3_COUNTER")]
pub type T3Counter = crate::Reg<t3_counter::T3CounterSpec>;
#[doc = "ADV_TIMER3 counter register"]
pub mod t3_counter;
#[doc = "EVENT_CFG (rw) register accessor: ADV_TIMERS events configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_cfg`]
module"]
#[doc(alias = "EVENT_CFG")]
pub type EventCfg = crate::Reg<event_cfg::EventCfgSpec>;
#[doc = "ADV_TIMERS events configuration register."]
pub mod event_cfg;
