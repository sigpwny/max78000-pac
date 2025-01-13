#[doc = "Register `DMACH0` reader"]
pub type R = crate::R<Dmach0Spec>;
#[doc = "Register `DMACH0` writer"]
pub type W = crate::W<Dmach0Spec>;
#[doc = "Field `DMA_TX_THD_VAL` reader - TX FIFO Level DMA Trigger."]
pub type DmaTxThdValR = crate::FieldReader;
#[doc = "Field `DMA_TX_THD_VAL` writer - TX FIFO Level DMA Trigger."]
pub type DmaTxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA_TX_EN` reader - TX DMA channel enable."]
pub type DmaTxEnR = crate::BitReader;
#[doc = "Field `DMA_TX_EN` writer - TX DMA channel enable."]
pub type DmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_THD_VAL` reader - RX FIFO Level DMA Trigger."]
pub type DmaRxThdValR = crate::FieldReader;
#[doc = "Field `DMA_RX_THD_VAL` writer - RX FIFO Level DMA Trigger."]
pub type DmaRxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DMA_RX_EN` reader - RX DMA channel enable."]
pub type DmaRxEnR = crate::BitReader;
#[doc = "Field `DMA_RX_EN` writer - RX DMA channel enable."]
pub type DmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LVL` reader - Number of data word in the TX FIFO."]
pub type TxLvlR = crate::FieldReader;
#[doc = "Field `TX_LVL` writer - Number of data word in the TX FIFO."]
pub type TxLvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_LVL` reader - Number of data word in the RX FIFO."]
pub type RxLvlR = crate::FieldReader;
#[doc = "Field `RX_LVL` writer - Number of data word in the RX FIFO."]
pub type RxLvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_tx_thd_val(&self) -> DmaTxThdValR {
        DmaTxThdValR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DmaTxEnR {
        DmaTxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_rx_thd_val(&self) -> DmaRxThdValR {
        DmaRxThdValR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DmaRxEnR {
        DmaRxEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TxLvlR {
        TxLvlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RxLvlR {
        RxLvlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_tx_thd_val(&mut self) -> DmaTxThdValW<Dmach0Spec> {
        DmaTxThdValW::new(self, 0)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DmaTxEnW<Dmach0Spec> {
        DmaTxEnW::new(self, 7)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_rx_thd_val(&mut self) -> DmaRxThdValW<Dmach0Spec> {
        DmaRxThdValW::new(self, 8)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DmaRxEnW<Dmach0Spec> {
        DmaRxEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&mut self) -> TxLvlW<Dmach0Spec> {
        TxLvlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&mut self) -> RxLvlW<Dmach0Spec> {
        RxLvlW::new(self, 24)
    }
}
#[doc = "DMA Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmach0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmach0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach0Spec;
impl crate::RegisterSpec for Dmach0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach0::R`](R) reader structure"]
impl crate::Readable for Dmach0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmach0::W`](W) writer structure"]
impl crate::Writable for Dmach0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH0 to value 0"]
impl crate::Resettable for Dmach0Spec {
    const RESET_VALUE: u32 = 0;
}
