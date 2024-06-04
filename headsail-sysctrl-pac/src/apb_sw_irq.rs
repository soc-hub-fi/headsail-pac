#[repr(C)]
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    apb_software_irq: APB_SOFTWARE_IRQ,
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Apb_software_irq"]
    #[inline(always)]
    pub const fn apb_software_irq(&self) -> &APB_SOFTWARE_IRQ {
        &self.apb_software_irq
    }
}
#[doc = "Apb_software_irq"]
pub use self::apb_software_irq::APB_SOFTWARE_IRQ;
#[doc = r"Cluster"]
#[doc = "Apb_software_irq"]
pub mod apb_software_irq;
