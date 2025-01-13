#[doc = "Register `CTRL0CH0` reader"]
pub type R = crate::R<Ctrl0ch0Spec>;
#[doc = "Register `CTRL0CH0` writer"]
pub type W = crate::W<Ctrl0ch0Spec>;
#[doc = "Field `LSB_FIRST` reader - LSB Transmit Receive First."]
pub type LsbFirstR = crate::BitReader;
#[doc = "Field `LSB_FIRST` writer - LSB Transmit Receive First."]
pub type LsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM_FILT` reader - PDM Filter."]
pub type PdmFiltR = crate::BitReader;
#[doc = "Field `PDM_FILT` writer - PDM Filter."]
pub type PdmFiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM_EN` reader - PDM Enable."]
pub type PdmEnR = crate::BitReader;
#[doc = "Field `PDM_EN` writer - PDM Enable."]
pub type PdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEDDR` reader - DDR."]
pub type UseddrR = crate::BitReader;
#[doc = "Field `USEDDR` writer - DDR."]
pub type UseddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM_INV` reader - Invert PDM."]
pub type PdmInvR = crate::BitReader;
#[doc = "Field `PDM_INV` writer - Invert PDM."]
pub type PdmInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_MODE` reader - SCK Select."]
pub type ChModeR = crate::FieldReader;
#[doc = "Field `CH_MODE` writer - SCK Select."]
pub type ChModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WS_POL` reader - WS polarity select."]
pub type WsPolR = crate::BitReader;
#[doc = "Field `WS_POL` writer - WS polarity select."]
pub type WsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB_LOC` reader - MSB location."]
pub type MsbLocR = crate::BitReader;
#[doc = "Field `ALIGN` reader - Align to MSB or LSB."]
pub type AlignR = crate::BitReader;
#[doc = "Field `EXT_SEL` reader - External SCK/WS selection."]
pub type ExtSelR = crate::BitReader;
#[doc = "Field `EXT_SEL` writer - External SCK/WS selection."]
pub type ExtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEREO` reader - Stereo mode of I2S."]
pub type StereoR = crate::FieldReader;
#[doc = "Field `WSIZE` reader - Data size when write to FIFO."]
pub type WsizeR = crate::FieldReader;
#[doc = "Field `WSIZE` writer - Data size when write to FIFO."]
pub type WsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_EN` reader - TX channel enable."]
pub type TxEnR = crate::BitReader;
#[doc = "Field `TX_EN` writer - TX channel enable."]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EN` reader - RX channel enable."]
pub type RxEnR = crate::BitReader;
#[doc = "Field `RX_EN` writer - RX channel enable."]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSH` reader - Flushes the TX/RX FIFO buffer."]
pub type FlushR = crate::BitReader;
#[doc = "Field `FLUSH` writer - Flushes the TX/RX FIFO buffer."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Write 1 to reset channel."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Write 1 to reset channel."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_LSB` reader - Bit Field Control."]
pub type FifoLsbR = crate::BitReader;
#[doc = "Field `FIFO_LSB` writer - Bit Field Control."]
pub type FifoLsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_VAL` reader - depth of receive FIFO for threshold interrupt generation."]
pub type RxThdValR = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - depth of receive FIFO for threshold interrupt generation."]
pub type RxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - LSB Transmit Receive First."]
    #[inline(always)]
    pub fn lsb_first(&self) -> LsbFirstR {
        LsbFirstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDM Filter."]
    #[inline(always)]
    pub fn pdm_filt(&self) -> PdmFiltR {
        PdmFiltR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDM Enable."]
    #[inline(always)]
    pub fn pdm_en(&self) -> PdmEnR {
        PdmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DDR."]
    #[inline(always)]
    pub fn useddr(&self) -> UseddrR {
        UseddrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert PDM."]
    #[inline(always)]
    pub fn pdm_inv(&self) -> PdmInvR {
        PdmInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SCK Select."]
    #[inline(always)]
    pub fn ch_mode(&self) -> ChModeR {
        ChModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WS polarity select."]
    #[inline(always)]
    pub fn ws_pol(&self) -> WsPolR {
        WsPolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSB location."]
    #[inline(always)]
    pub fn msb_loc(&self) -> MsbLocR {
        MsbLocR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Align to MSB or LSB."]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External SCK/WS selection."]
    #[inline(always)]
    pub fn ext_sel(&self) -> ExtSelR {
        ExtSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Stereo mode of I2S."]
    #[inline(always)]
    pub fn stereo(&self) -> StereoR {
        StereoR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data size when write to FIFO."]
    #[inline(always)]
    pub fn wsize(&self) -> WsizeR {
        WsizeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flushes the TX/RX FIFO buffer."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write 1 to reset channel."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Field Control."]
    #[inline(always)]
    pub fn fifo_lsb(&self) -> FifoLsbR {
        FifoLsbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - depth of receive FIFO for threshold interrupt generation."]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RxThdValR {
        RxThdValR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LSB Transmit Receive First."]
    #[inline(always)]
    pub fn lsb_first(&mut self) -> LsbFirstW<Ctrl0ch0Spec> {
        LsbFirstW::new(self, 1)
    }
    #[doc = "Bit 2 - PDM Filter."]
    #[inline(always)]
    pub fn pdm_filt(&mut self) -> PdmFiltW<Ctrl0ch0Spec> {
        PdmFiltW::new(self, 2)
    }
    #[doc = "Bit 3 - PDM Enable."]
    #[inline(always)]
    pub fn pdm_en(&mut self) -> PdmEnW<Ctrl0ch0Spec> {
        PdmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - DDR."]
    #[inline(always)]
    pub fn useddr(&mut self) -> UseddrW<Ctrl0ch0Spec> {
        UseddrW::new(self, 4)
    }
    #[doc = "Bit 5 - Invert PDM."]
    #[inline(always)]
    pub fn pdm_inv(&mut self) -> PdmInvW<Ctrl0ch0Spec> {
        PdmInvW::new(self, 5)
    }
    #[doc = "Bits 6:7 - SCK Select."]
    #[inline(always)]
    pub fn ch_mode(&mut self) -> ChModeW<Ctrl0ch0Spec> {
        ChModeW::new(self, 6)
    }
    #[doc = "Bit 8 - WS polarity select."]
    #[inline(always)]
    pub fn ws_pol(&mut self) -> WsPolW<Ctrl0ch0Spec> {
        WsPolW::new(self, 8)
    }
    #[doc = "Bit 11 - External SCK/WS selection."]
    #[inline(always)]
    pub fn ext_sel(&mut self) -> ExtSelW<Ctrl0ch0Spec> {
        ExtSelW::new(self, 11)
    }
    #[doc = "Bits 14:15 - Data size when write to FIFO."]
    #[inline(always)]
    pub fn wsize(&mut self) -> WsizeW<Ctrl0ch0Spec> {
        WsizeW::new(self, 14)
    }
    #[doc = "Bit 16 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TxEnW<Ctrl0ch0Spec> {
        TxEnW::new(self, 16)
    }
    #[doc = "Bit 17 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<Ctrl0ch0Spec> {
        RxEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Flushes the TX/RX FIFO buffer."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<Ctrl0ch0Spec> {
        FlushW::new(self, 18)
    }
    #[doc = "Bit 19 - Write 1 to reset channel."]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<Ctrl0ch0Spec> {
        RstW::new(self, 19)
    }
    #[doc = "Bit 20 - Bit Field Control."]
    #[inline(always)]
    pub fn fifo_lsb(&mut self) -> FifoLsbW<Ctrl0ch0Spec> {
        FifoLsbW::new(self, 20)
    }
    #[doc = "Bits 24:31 - depth of receive FIFO for threshold interrupt generation."]
    #[inline(always)]
    pub fn rx_thd_val(&mut self) -> RxThdValW<Ctrl0ch0Spec> {
        RxThdValW::new(self, 24)
    }
}
#[doc = "Global mode channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0ch0Spec;
impl crate::RegisterSpec for Ctrl0ch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0ch0::R`](R) reader structure"]
impl crate::Readable for Ctrl0ch0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0ch0::W`](W) writer structure"]
impl crate::Writable for Ctrl0ch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0CH0 to value 0"]
impl crate::Resettable for Ctrl0ch0Spec {
    const RESET_VALUE: u32 = 0;
}
