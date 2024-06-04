#[doc = "Register `UART_SETUP` reader"]
pub type R = crate::R<UART_SETUP_SPEC>;
#[doc = "Register `UART_SETUP` writer"]
pub type W = crate::W<UART_SETUP_SPEC>;
#[doc = "Field `PARITY_ENA` reader - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type PARITY_ENA_R = crate::BitReader;
#[doc = "Field `PARITY_ENA` writer - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type PARITY_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LENGTH` reader - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub type BIT_LENGTH_R = crate::FieldReader;
#[doc = "Field `BIT_LENGTH` writer - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub type BIT_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP_BITS` reader - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub type STOP_BITS_R = crate::BitReader;
#[doc = "Field `STOP_BITS` writer - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub type STOP_BITS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLING_EN` reader - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub type POLLING_EN_R = crate::BitReader;
#[doc = "Field `POLLING_EN` writer - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub type POLLING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAN_FIFO` reader - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub type CLEAN_FIFO_R = crate::BitReader;
#[doc = "Field `CLEAN_FIFO` writer - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub type CLEAN_FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENA` reader - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type TX_ENA_R = crate::BitReader;
#[doc = "Field `TX_ENA` writer - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type TX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENA` reader - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type RX_ENA_R = crate::BitReader;
#[doc = "Field `RX_ENA` writer - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type RX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn parity_ena(&self) -> PARITY_ENA_R {
        PARITY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
    #[inline(always)]
    pub fn bit_length(&self) -> BIT_LENGTH_R {
        BIT_LENGTH_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
    #[inline(always)]
    pub fn polling_en(&self) -> POLLING_EN_R {
        POLLING_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
    #[inline(always)]
    pub fn clean_fifo(&self) -> CLEAN_FIFO_R {
        CLEAN_FIFO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn tx_ena(&self) -> TX_ENA_R {
        TX_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn rx_ena(&self) -> RX_ENA_R {
        RX_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "derive-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_SETUP")
            .field("parity_ena", &self.parity_ena())
            .field("bit_length", &self.bit_length())
            .field("stop_bits", &self.stop_bits())
            .field("polling_en", &self.polling_en())
            .field("clean_fifo", &self.clean_fifo())
            .field("tx_ena", &self.tx_ena())
            .field("rx_ena", &self.rx_ena())
            .field("clkdiv", &self.clkdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_ena(&mut self) -> PARITY_ENA_W<UART_SETUP_SPEC> {
        PARITY_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn bit_length(&mut self) -> BIT_LENGTH_W<UART_SETUP_SPEC> {
        BIT_LENGTH_W::new(self, 1)
    }
    #[doc = "Bit 3 - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<UART_SETUP_SPEC> {
        STOP_BITS_W::new(self, 3)
    }
    #[doc = "Bit 4 - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn polling_en(&mut self) -> POLLING_EN_W<UART_SETUP_SPEC> {
        POLLING_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn clean_fifo(&mut self) -> CLEAN_FIFO_W<UART_SETUP_SPEC> {
        CLEAN_FIFO_W::new(self, 5)
    }
    #[doc = "Bit 8 - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ena(&mut self) -> TX_ENA_W<UART_SETUP_SPEC> {
        TX_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ena(&mut self) -> RX_ENA_W<UART_SETUP_SPEC> {
        RX_ENA_W::new(self, 9)
    }
    #[doc = "Bits 16:31 - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<UART_SETUP_SPEC> {
        CLKDIV_W::new(self, 16)
    }
}
#[doc = "UDMA UART configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_setup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_setup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_SETUP_SPEC;
impl crate::RegisterSpec for UART_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_setup::R`](R) reader structure"]
impl crate::Readable for UART_SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_setup::W`](W) writer structure"]
impl crate::Writable for UART_SETUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_SETUP to value 0"]
impl crate::Resettable for UART_SETUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
