#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    rbr_thr_dll: RBR_THR_DLL,
    ier_dlm: IER_DLM,
    iir_fcr: IIR_FCR,
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: MSR,
    scr: SCR,
}
impl RegisterBlock {
    #[doc = "0x00 - RBR_THR_DLL is a multi-purpose register address. you can access 3 different registers using the same address. IF LCR\\[7\\]
is 0 RBR and THR are accessable. OW DLL is accessable. RBR read only. Reads from rx fifo THR write only. Writes into a tx fifo DLL writes/reads into/from the 8 LSBs in the divisor"]
    #[inline(always)]
    pub const fn rbr_thr_dll(&self) -> &RBR_THR_DLL {
        &self.rbr_thr_dll
    }
    #[doc = "0x01 - IER_DLM is a multi-purpose register address. you can access 2 different registers using the same address. IF LCR\\[7\\]
is 0 IER is accessable. O.W DLM is accessable. IER writes/reads into/from position IER => IER\\[2:0\\]: The interrupt enable bits. => xx1: Interrupt is raised when Received data available or trigger level reached in FIFO mode if non of these it checks Character timeout indication. => x10: Interrupt is raised when Transmitter holding register empty. => 100: Interrupt is raised when Overrun error, parity error, framing error or break interrupt. => other bits aren't used"]
    #[inline(always)]
    pub const fn ier_dlm(&self) -> &IER_DLM {
        &self.ier_dlm
    }
    #[doc = "0x02 - IIR_FCR is a multi-purpose register address. you can access 2 different registers using the same address. However LCR\\[7\\]
isn't needed in this case because one of them is only written and the other is only read. FCR (fifo control register) write only => FCR\\[1\\]: clears the rx fifo if high => FCR\\[2\\]: clears the tx fifo if high => FCR\\[7:6\\]: sets the trigger level =>00: trigger level is high when there is 1 element in the fifo =>01: trigger level is high when there are 4 elements in the fifo =>10: trigger level is high when there are 8 elements in the fifo =>11: trigger level is high when there are 14 elements in the fifo => other bits aren't used IIR (Interrupt Identification Register) read only"]
    #[inline(always)]
    pub const fn iir_fcr(&self) -> &IIR_FCR {
        &self.iir_fcr
    }
    #[doc = "0x03 - LCR is a register configures the main operation of the uart. It configures the width of the received data, stop bit, parity, and DLAB bit. => LCR\\[1:0\\]: configuration bits =>00: data is configured to be 5 bits =>01: data is configured to be 6 bits =>10: data is configured to be 7 bits =>11: data is configured to be 8 bits => LCR\\[2\\]: stop bit configuration =>0: 1 stop bit =>1: 1.5 stop bits for 5 bits data word OR 2 stop bits 6, 7 or 8 bits data word => LCR\\[3\\]: parity enable bit => LCR\\[7\\]: divisor latch access bit (DLAB) =>0: RBR, THR and IER accessible =>1: DLL and DLM accessible => other bits aren't used"]
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    #[doc = "0x04 - modem control (not used)"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x05 - LSR is the line status register => LSR\\[0\\]: rx fifo data valid => LSR\\[2\\]: parity error from the rx fifo => LSR\\[5\\]: the tx fifo is empty => LSR\\[6\\]: shift register and tx fifo are empty => other bits aren't used"]
    #[inline(always)]
    pub const fn lsr(&self) -> &LSR {
        &self.lsr
    }
    #[doc = "0x06 - modem status (not used)"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    #[doc = "0x07 - scratch (not used)"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
}
#[doc = "IER_DLM (rw) register accessor: IER_DLM is a multi-purpose register address. you can access 2 different registers using the same address. IF LCR\\[7\\]
is 0 IER is accessable. O.W DLM is accessable. IER writes/reads into/from position IER => IER\\[2:0\\]: The interrupt enable bits. => xx1: Interrupt is raised when Received data available or trigger level reached in FIFO mode if non of these it checks Character timeout indication. => x10: Interrupt is raised when Transmitter holding register empty. => 100: Interrupt is raised when Overrun error, parity error, framing error or break interrupt. => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier_dlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier_dlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier_dlm`]
module"]
pub type IER_DLM = crate::Reg<ier_dlm::IER_DLM_SPEC>;
#[doc = "IER_DLM is a multi-purpose register address. you can access 2 different registers using the same address. IF LCR\\[7\\]
is 0 IER is accessable. O.W DLM is accessable. IER writes/reads into/from position IER => IER\\[2:0\\]: The interrupt enable bits. => xx1: Interrupt is raised when Received data available or trigger level reached in FIFO mode if non of these it checks Character timeout indication. => x10: Interrupt is raised when Transmitter holding register empty. => 100: Interrupt is raised when Overrun error, parity error, framing error or break interrupt. => other bits aren't used"]
pub mod ier_dlm;
#[doc = "IIR_FCR (rw) register accessor: IIR_FCR is a multi-purpose register address. you can access 2 different registers using the same address. However LCR\\[7\\]
isn't needed in this case because one of them is only written and the other is only read. FCR (fifo control register) write only => FCR\\[1\\]: clears the rx fifo if high => FCR\\[2\\]: clears the tx fifo if high => FCR\\[7:6\\]: sets the trigger level =>00: trigger level is high when there is 1 element in the fifo =>01: trigger level is high when there are 4 elements in the fifo =>10: trigger level is high when there are 8 elements in the fifo =>11: trigger level is high when there are 14 elements in the fifo => other bits aren't used IIR (Interrupt Identification Register) read only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir_fcr`]
module"]
pub type IIR_FCR = crate::Reg<iir_fcr::IIR_FCR_SPEC>;
#[doc = "IIR_FCR is a multi-purpose register address. you can access 2 different registers using the same address. However LCR\\[7\\]
isn't needed in this case because one of them is only written and the other is only read. FCR (fifo control register) write only => FCR\\[1\\]: clears the rx fifo if high => FCR\\[2\\]: clears the tx fifo if high => FCR\\[7:6\\]: sets the trigger level =>00: trigger level is high when there is 1 element in the fifo =>01: trigger level is high when there are 4 elements in the fifo =>10: trigger level is high when there are 8 elements in the fifo =>11: trigger level is high when there are 14 elements in the fifo => other bits aren't used IIR (Interrupt Identification Register) read only"]
pub mod iir_fcr;
#[doc = "LCR (rw) register accessor: LCR is a register configures the main operation of the uart. It configures the width of the received data, stop bit, parity, and DLAB bit. => LCR\\[1:0\\]: configuration bits =>00: data is configured to be 5 bits =>01: data is configured to be 6 bits =>10: data is configured to be 7 bits =>11: data is configured to be 8 bits => LCR\\[2\\]: stop bit configuration =>0: 1 stop bit =>1: 1.5 stop bits for 5 bits data word OR 2 stop bits 6, 7 or 8 bits data word => LCR\\[3\\]: parity enable bit => LCR\\[7\\]: divisor latch access bit (DLAB) =>0: RBR, THR and IER accessible =>1: DLL and DLM accessible => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "LCR is a register configures the main operation of the uart. It configures the width of the received data, stop bit, parity, and DLAB bit. => LCR\\[1:0\\]: configuration bits =>00: data is configured to be 5 bits =>01: data is configured to be 6 bits =>10: data is configured to be 7 bits =>11: data is configured to be 8 bits => LCR\\[2\\]: stop bit configuration =>0: 1 stop bit =>1: 1.5 stop bits for 5 bits data word OR 2 stop bits 6, 7 or 8 bits data word => LCR\\[3\\]: parity enable bit => LCR\\[7\\]: divisor latch access bit (DLAB) =>0: RBR, THR and IER accessible =>1: DLL and DLM accessible => other bits aren't used"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: modem control (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "modem control (not used)"]
pub mod mcr;
#[doc = "LSR (r) register accessor: LSR is the line status register => LSR\\[0\\]: rx fifo data valid => LSR\\[2\\]: parity error from the rx fifo => LSR\\[5\\]: the tx fifo is empty => LSR\\[6\\]: shift register and tx fifo are empty => other bits aren't used\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "LSR is the line status register => LSR\\[0\\]: rx fifo data valid => LSR\\[2\\]: parity error from the rx fifo => LSR\\[5\\]: the tx fifo is empty => LSR\\[6\\]: shift register and tx fifo are empty => other bits aren't used"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: modem status (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "modem status (not used)"]
pub mod msr;
#[doc = "SCR (rw) register accessor: scratch (not used)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "scratch (not used)"]
pub mod scr;
#[doc = "RBR_THR_DLL (rw) register accessor: RBR_THR_DLL is a multi-purpose register address. you can access 3 different registers using the same address. IF LCR\\[7\\]
is 0 RBR and THR are accessable. OW DLL is accessable. RBR read only. Reads from rx fifo THR write only. Writes into a tx fifo DLL writes/reads into/from the 8 LSBs in the divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr_thr_dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr_thr_dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr_thr_dll`]
module"]
pub type RBR_THR_DLL = crate::Reg<rbr_thr_dll::RBR_THR_DLL_SPEC>;
#[doc = "RBR_THR_DLL is a multi-purpose register address. you can access 3 different registers using the same address. IF LCR\\[7\\]
is 0 RBR and THR are accessable. OW DLL is accessable. RBR read only. Reads from rx fifo THR write only. Writes into a tx fifo DLL writes/reads into/from the 8 LSBs in the divisor"]
pub mod rbr_thr_dll;
