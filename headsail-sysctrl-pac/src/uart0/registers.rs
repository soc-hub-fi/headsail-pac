#[doc = r"Register block"]
#[repr(C)]
pub struct REGISTERS {
    #[doc = "0x00 - RBR receiver buffer, THR transmitter holding, DLL divisor latch LSB"]
    pub rbr_thr_dll: RBR_THR_DLL,
    #[doc = "0x01 - IER interrupt enable, DLM divisor latch MSB"]
    pub ier_dlm: IER_DLM,
    #[doc = "0x02 - IIR interrupt identification, FCR FIFO control"]
    pub iir_fcr: IIR_FCR,
    #[doc = "0x03 - line control"]
    pub lcr: LCR,
    #[doc = "0x04 - modem control (not used)"]
    pub mcr: MCR,
    #[doc = "0x05 - line status"]
    pub lsr: LSR,
    #[doc = "0x06 - modem status (not used)"]
    pub msr: MSR,
    #[doc = "0x07 - scratch (not used)"]
    pub scr: SCR,
}
#[doc = "IER_DLM (rw) register accessor: an alias for `Reg<IER_DLM_SPEC>`"]
pub type IER_DLM = crate::Reg<ier_dlm::IER_DLM_SPEC>;
#[doc = "IER interrupt enable, DLM divisor latch MSB"]
pub mod ier_dlm;
#[doc = "IIR_FCR (rw) register accessor: an alias for `Reg<IIR_FCR_SPEC>`"]
pub type IIR_FCR = crate::Reg<iir_fcr::IIR_FCR_SPEC>;
#[doc = "IIR interrupt identification, FCR FIFO control"]
pub mod iir_fcr;
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "line control"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "modem control (not used)"]
pub mod mcr;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "line status"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "modem status (not used)"]
pub mod msr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "scratch (not used)"]
pub mod scr;
#[doc = "RBR_THR_DLL (rw) register accessor: an alias for `Reg<RBR_THR_DLL_SPEC>`"]
pub type RBR_THR_DLL = crate::Reg<rbr_thr_dll::RBR_THR_DLL_SPEC>;
#[doc = "RBR receiver buffer, THR transmitter holding, DLL divisor latch LSB"]
pub mod rbr_thr_dll;
