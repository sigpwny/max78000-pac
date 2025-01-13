#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `TX_BUSY` reader - Read-only flag indicating the UART transmit status"]
pub type TxBusyR = crate::BitReader;
#[doc = "Field `RX_BUSY` reader - Read-only flag indicating the UART receiver status"]
pub type RxBusyR = crate::BitReader;
#[doc = "Field `RX_EM` reader - Read-only flag indicating the RX FIFO state"]
pub type RxEmR = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Read-only flag indicating the RX FIFO state"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `TX_EM` reader - Read-only flag indicating the TX FIFO state"]
pub type TxEmR = crate::BitReader;
#[doc = "Field `TX_FULL` reader - Read-only flag indicating the TX FIFO state"]
pub type TxFullR = crate::BitReader;
#[doc = "Field `RX_LVL` reader - Indicates the number of bytes currently in the RX FIFO (0-RX FIFO_ELTS)"]
pub type RxLvlR = crate::FieldReader;
#[doc = "Field `TX_LVL` reader - Indicates the number of bytes currently in the TX FIFO (0-TX FIFO_ELTS)"]
pub type TxLvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Read-only flag indicating the UART transmit status"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TxBusyR {
        TxBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read-only flag indicating the UART receiver status"]
    #[inline(always)]
    pub fn rx_busy(&self) -> RxBusyR {
        RxBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Read-only flag indicating the RX FIFO state"]
    #[inline(always)]
    pub fn rx_em(&self) -> RxEmR {
        RxEmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read-only flag indicating the RX FIFO state"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read-only flag indicating the TX FIFO state"]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read-only flag indicating the TX FIFO state"]
    #[inline(always)]
    pub fn tx_full(&self) -> TxFullR {
        TxFullR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Indicates the number of bytes currently in the RX FIFO (0-RX FIFO_ELTS)"]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RxLvlR {
        RxLvlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the number of bytes currently in the TX FIFO (0-TX FIFO_ELTS)"]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TxLvlR {
        TxLvlR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
