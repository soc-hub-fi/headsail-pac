#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - Apb_software_irq"]
    pub apb_software_irq: APB_SOFTWARE_IRQ,
}
#[doc = "Apb_software_irq"]
pub use self::apb_software_irq::APB_SOFTWARE_IRQ;
#[doc = r"Cluster"]
#[doc = "Apb_software_irq"]
pub mod apb_software_irq;
