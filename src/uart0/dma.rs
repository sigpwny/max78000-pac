#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `TX_THD_VAL` reader - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
pub type TxThdValR = crate::FieldReader;
#[doc = "Field `TX_THD_VAL` writer - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
pub type TxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_EN` reader - TX DMA channel enable"]
pub type TxEnR = crate::BitReader;
#[doc = "Field `TX_EN` writer - TX DMA channel enable"]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_VAL` reader - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
pub type RxThdValR = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
pub type RxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_EN` reader - RX DMA channel enable"]
pub type RxEnR = crate::BitReader;
#[doc = "Field `RX_EN` writer - RX DMA channel enable"]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
    #[inline(always)]
    pub fn tx_thd_val(&self) -> TxThdValR {
        TxThdValR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - TX DMA channel enable"]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RxThdValR {
        RxThdValR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - RX DMA channel enable"]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX FIFO Level DMA Trigger If the TX FIFO level is less than this value, then the TX FIFO DMA interface will send a signal to system DMA to notify that TX FIFO is ready to receive data from memory."]
    #[inline(always)]
    pub fn tx_thd_val(&mut self) -> TxThdValW<DmaSpec> {
        TxThdValW::new(self, 0)
    }
    #[doc = "Bit 4 - TX DMA channel enable"]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TxEnW<DmaSpec> {
        TxEnW::new(self, 4)
    }
    #[doc = "Bits 5:8 - Rx FIFO Level DMA Trigger If the RX FIFO level is greater than this value, then the RX FIFO DMA interface will send a signal to the system DMA to notify that RX FIFO has characters to transfer to memory."]
    #[inline(always)]
    pub fn rx_thd_val(&mut self) -> RxThdValW<DmaSpec> {
        RxThdValW::new(self, 5)
    }
    #[doc = "Bit 9 - RX DMA channel enable"]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<DmaSpec> {
        RxEnW::new(self, 9)
    }
}
#[doc = "DMA Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
