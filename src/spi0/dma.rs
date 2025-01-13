#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `TX_THD_VAL` reader - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub type TxThdValR = crate::FieldReader;
#[doc = "Field `TX_THD_VAL` writer - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
pub type TxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transmit FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoEn {
    #[doc = "0: Transmit FIFO is not enabled."]
    Dis = 0,
    #[doc = "1: Transmit FIFO is enabled."]
    En = 1,
}
impl From<TxFifoEn> for bool {
    #[inline(always)]
    fn from(variant: TxFifoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_EN` reader - Transmit FIFO enabled for SPI transactions."]
pub type TxFifoEnR = crate::BitReader<TxFifoEn>;
impl TxFifoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoEn {
        match self.bits {
            false => TxFifoEn::Dis,
            true => TxFifoEn::En,
        }
    }
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxFifoEn::Dis
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxFifoEn::En
    }
}
#[doc = "Field `TX_FIFO_EN` writer - Transmit FIFO enabled for SPI transactions."]
pub type TxFifoEnW<'a, REG> = crate::BitWriter<'a, REG, TxFifoEn>;
impl<'a, REG> TxFifoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifoEn::Dis)
    }
    #[doc = "Transmit FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifoEn::En)
    }
}
#[doc = "Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFlush {
    #[doc = "1: Clear the Transmit FIFO, clears any pending TX FIFO status."]
    Clear = 1,
}
impl From<TxFlush> for bool {
    #[inline(always)]
    fn from(variant: TxFlush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FLUSH` reader - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type TxFlushR = crate::BitReader<TxFlush>;
impl TxFlushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxFlush> {
        match self.bits {
            true => Some(TxFlush::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TxFlush::Clear
    }
}
#[doc = "Field `TX_FLUSH` writer - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type TxFlushW<'a, REG> = crate::BitWriter<'a, REG, TxFlush>;
impl<'a, REG> TxFlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Transmit FIFO, clears any pending TX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxFlush::Clear)
    }
}
#[doc = "Field `TX_LVL` reader - Count of entries in TX FIFO."]
pub type TxLvlR = crate::FieldReader;
#[doc = "TX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaTxEn {
    #[doc = "0: TX DMA requests are disabled, andy pending DMA requests are cleared."]
    Dis = 0,
    #[doc = "1: TX DMA requests are enabled."]
    En = 1,
}
impl From<DmaTxEn> for bool {
    #[inline(always)]
    fn from(variant: DmaTxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_TX_EN` reader - TX DMA Enable."]
pub type DmaTxEnR = crate::BitReader<DmaTxEn>;
impl DmaTxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaTxEn {
        match self.bits {
            false => DmaTxEn::Dis,
            true => DmaTxEn::En,
        }
    }
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DmaTxEn::Dis
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DmaTxEn::En
    }
}
#[doc = "Field `DMA_TX_EN` writer - TX DMA Enable."]
pub type DmaTxEnW<'a, REG> = crate::BitWriter<'a, REG, DmaTxEn>;
impl<'a, REG> DmaTxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX DMA requests are disabled, andy pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DmaTxEn::Dis)
    }
    #[doc = "TX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DmaTxEn::En)
    }
}
#[doc = "Field `RX_THD_VAL` reader - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub type RxThdValR = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
pub type RxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Receive FIFO enabled for SPI transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFifoEn {
    #[doc = "0: Receive FIFO is not enabled."]
    Dis = 0,
    #[doc = "1: Receive FIFO is enabled."]
    En = 1,
}
impl From<RxFifoEn> for bool {
    #[inline(always)]
    fn from(variant: RxFifoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_EN` reader - Receive FIFO enabled for SPI transactions."]
pub type RxFifoEnR = crate::BitReader<RxFifoEn>;
impl RxFifoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFifoEn {
        match self.bits {
            false => RxFifoEn::Dis,
            true => RxFifoEn::En,
        }
    }
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxFifoEn::Dis
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxFifoEn::En
    }
}
#[doc = "Field `RX_FIFO_EN` writer - Receive FIFO enabled for SPI transactions."]
pub type RxFifoEnW<'a, REG> = crate::BitWriter<'a, REG, RxFifoEn>;
impl<'a, REG> RxFifoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO is not enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifoEn::Dis)
    }
    #[doc = "Receive FIFO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifoEn::En)
    }
}
#[doc = "Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFlush {
    #[doc = "1: Clear the Receive FIFO, clears any pending RX FIFO status."]
    Clear = 1,
}
impl From<RxFlush> for bool {
    #[inline(always)]
    fn from(variant: RxFlush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FLUSH` reader - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type RxFlushR = crate::BitReader<RxFlush>;
impl RxFlushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxFlush> {
        match self.bits {
            true => Some(RxFlush::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RxFlush::Clear
    }
}
#[doc = "Field `RX_FLUSH` writer - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
pub type RxFlushW<'a, REG> = crate::BitWriter<'a, REG, RxFlush>;
impl<'a, REG> RxFlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Receive FIFO, clears any pending RX FIFO status."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxFlush::Clear)
    }
}
#[doc = "Field `RX_LVL` reader - Count of entries in RX FIFO."]
pub type RxLvlR = crate::FieldReader;
#[doc = "RX DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxEn {
    #[doc = "0: RX DMA requests are disabled, any pending DMA requests are cleared."]
    Dis = 0,
    #[doc = "1: RX DMA requests are enabled."]
    En = 1,
}
impl From<DmaRxEn> for bool {
    #[inline(always)]
    fn from(variant: DmaRxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_EN` reader - RX DMA Enable."]
pub type DmaRxEnR = crate::BitReader<DmaRxEn>;
impl DmaRxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxEn {
        match self.bits {
            false => DmaRxEn::Dis,
            true => DmaRxEn::En,
        }
    }
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DmaRxEn::Dis
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DmaRxEn::En
    }
}
#[doc = "Field `DMA_RX_EN` writer - RX DMA Enable."]
pub type DmaRxEnW<'a, REG> = crate::BitWriter<'a, REG, DmaRxEn>;
impl<'a, REG> DmaRxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX DMA requests are disabled, any pending DMA requests are cleared."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEn::Dis)
    }
    #[doc = "RX DMA requests are enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEn::En)
    }
}
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn tx_thd_val(&self) -> TxThdValR {
        TxThdValR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TxFifoEnR {
        TxFifoEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TxFlushR {
        TxFlushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Count of entries in TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TxLvlR {
        TxLvlR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DmaTxEnR {
        DmaTxEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RxThdValR {
        RxThdValR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RxFifoEnR {
        RxFifoEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RxFlushR {
        RxFlushR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Count of entries in RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RxLvlR {
        RxLvlR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DmaRxEnR {
        DmaRxEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO level that will trigger a DMA request, also level for threshold status. When TX FIFO has fewer than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn tx_thd_val(&mut self) -> TxThdValW<DmaSpec> {
        TxThdValW::new(self, 0)
    }
    #[doc = "Bit 6 - Transmit FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TxFifoEnW<DmaSpec> {
        TxFifoEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear TX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn tx_flush(&mut self) -> TxFlushW<DmaSpec> {
        TxFlushW::new(self, 7)
    }
    #[doc = "Bit 15 - TX DMA Enable."]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DmaTxEnW<DmaSpec> {
        DmaTxEnW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Receive FIFO level that will trigger a DMA request, also level for threshold status. When RX FIFO has more than this many bytes, the associated events and conditions are triggered."]
    #[inline(always)]
    pub fn rx_thd_val(&mut self) -> RxThdValW<DmaSpec> {
        RxThdValW::new(self, 16)
    }
    #[doc = "Bit 22 - Receive FIFO enabled for SPI transactions."]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RxFifoEnW<DmaSpec> {
        RxFifoEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Clear RX FIFO, clear is accomplished by resetting the read and write pointers. This should be done when FIFO is not being accessed on the SPI side."]
    #[inline(always)]
    pub fn rx_flush(&mut self) -> RxFlushW<DmaSpec> {
        RxFlushW::new(self, 23)
    }
    #[doc = "Bit 31 - RX DMA Enable."]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DmaRxEnW<DmaSpec> {
        DmaRxEnW::new(self, 31)
    }
}
#[doc = "Register for controlling DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
