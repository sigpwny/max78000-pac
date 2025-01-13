#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RX_THD_VAL` reader - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
pub type RxThdValR = crate::FieldReader;
#[doc = "Field `RX_THD_VAL` writer - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
pub type RxThdValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAR_EN` reader - Parity Enable"]
pub type ParEnR = crate::BitReader;
#[doc = "Field `PAR_EN` writer - Parity Enable"]
pub type ParEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR_EO` reader - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
pub type ParEoR = crate::BitReader;
#[doc = "Field `PAR_EO` writer - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
pub type ParEoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAR_MD` reader - Selects parity based on 1s or 0s count (when PAREN=1)"]
pub type ParMdR = crate::BitReader;
#[doc = "Field `PAR_MD` writer - Selects parity based on 1s or 0s count (when PAREN=1)"]
pub type ParMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_DIS` reader - CTS Sampling Disable"]
pub type CtsDisR = crate::BitReader;
#[doc = "Field `CTS_DIS` writer - CTS Sampling Disable"]
pub type CtsDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FLUSH` reader - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type TxFlushR = crate::BitReader;
#[doc = "Field `TX_FLUSH` writer - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type TxFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FLUSH` reader - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type RxFlushR = crate::BitReader;
#[doc = "Field `RX_FLUSH` writer - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
pub type RxFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects UART character size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CharSize {
    #[doc = "0: 5 bits"]
    _5bits = 0,
    #[doc = "1: 6 bits"]
    _6bits = 1,
    #[doc = "2: 7 bits"]
    _7bits = 2,
    #[doc = "3: 8 bits"]
    _8bits = 3,
}
impl From<CharSize> for u8 {
    #[inline(always)]
    fn from(variant: CharSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CharSize {
    type Ux = u8;
}
impl crate::IsEnum for CharSize {}
#[doc = "Field `CHAR_SIZE` reader - Selects UART character size"]
pub type CharSizeR = crate::FieldReader<CharSize>;
impl CharSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CharSize {
        match self.bits {
            0 => CharSize::_5bits,
            1 => CharSize::_6bits,
            2 => CharSize::_7bits,
            3 => CharSize::_8bits,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_5bits(&self) -> bool {
        *self == CharSize::_5bits
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_6bits(&self) -> bool {
        *self == CharSize::_6bits
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == CharSize::_7bits
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == CharSize::_8bits
    }
}
#[doc = "Field `CHAR_SIZE` writer - Selects UART character size"]
pub type CharSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CharSize, crate::Safe>;
impl<'a, REG> CharSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _5bits(self) -> &'a mut crate::W<REG> {
        self.variant(CharSize::_5bits)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6bits(self) -> &'a mut crate::W<REG> {
        self.variant(CharSize::_6bits)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(CharSize::_7bits)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(CharSize::_8bits)
    }
}
#[doc = "Field `STOPBITS` reader - Selects the number of stop bits that will be generated"]
pub type StopbitsR = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Selects the number of stop bits that will be generated"]
pub type StopbitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFC_EN` reader - Enables/disables hardware flow control"]
pub type HfcEnR = crate::BitReader;
#[doc = "Field `HFC_EN` writer - Enables/disables hardware flow control"]
pub type HfcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDC` reader - Hardware Flow Control RTS Mode"]
pub type RtsdcR = crate::BitReader;
#[doc = "Field `RTSDC` writer - Hardware Flow Control RTS Mode"]
pub type RtsdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCLKEN` reader - Baud clock enable"]
pub type BclkenR = crate::BitReader;
#[doc = "Field `BCLKEN` writer - Baud clock enable"]
pub type BclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bclksrc {
    #[doc = "0: apb clock"]
    PeripheralClock = 0,
    #[doc = "1: Clock 1"]
    ExternalClock = 1,
    #[doc = "2: Clock 2"]
    Clk2 = 2,
    #[doc = "3: Clock 3"]
    Clk3 = 3,
}
impl From<Bclksrc> for u8 {
    #[inline(always)]
    fn from(variant: Bclksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bclksrc {
    type Ux = u8;
}
impl crate::IsEnum for Bclksrc {}
#[doc = "Field `BCLKSRC` reader - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
pub type BclksrcR = crate::FieldReader<Bclksrc>;
impl BclksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bclksrc {
        match self.bits {
            0 => Bclksrc::PeripheralClock,
            1 => Bclksrc::ExternalClock,
            2 => Bclksrc::Clk2,
            3 => Bclksrc::Clk3,
            _ => unreachable!(),
        }
    }
    #[doc = "apb clock"]
    #[inline(always)]
    pub fn is_peripheral_clock(&self) -> bool {
        *self == Bclksrc::PeripheralClock
    }
    #[doc = "Clock 1"]
    #[inline(always)]
    pub fn is_external_clock(&self) -> bool {
        *self == Bclksrc::ExternalClock
    }
    #[doc = "Clock 2"]
    #[inline(always)]
    pub fn is_clk2(&self) -> bool {
        *self == Bclksrc::Clk2
    }
    #[doc = "Clock 3"]
    #[inline(always)]
    pub fn is_clk3(&self) -> bool {
        *self == Bclksrc::Clk3
    }
}
#[doc = "Field `BCLKSRC` writer - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
pub type BclksrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bclksrc, crate::Safe>;
impl<'a, REG> BclksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "apb clock"]
    #[inline(always)]
    pub fn peripheral_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Bclksrc::PeripheralClock)
    }
    #[doc = "Clock 1"]
    #[inline(always)]
    pub fn external_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Bclksrc::ExternalClock)
    }
    #[doc = "Clock 2"]
    #[inline(always)]
    pub fn clk2(self) -> &'a mut crate::W<REG> {
        self.variant(Bclksrc::Clk2)
    }
    #[doc = "Clock 3"]
    #[inline(always)]
    pub fn clk3(self) -> &'a mut crate::W<REG> {
        self.variant(Bclksrc::Clk3)
    }
}
#[doc = "Field `DPFE_EN` reader - Data/Parity bit frame error detection enable"]
pub type DpfeEnR = crate::BitReader;
#[doc = "Field `DPFE_EN` writer - Data/Parity bit frame error detection enable"]
pub type DpfeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCLKRDY` reader - Baud clock Ready read only bit"]
pub type BclkrdyR = crate::BitReader;
#[doc = "Field `BCLKRDY` writer - Baud clock Ready read only bit"]
pub type BclkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCAGM` reader - UART Clock Auto Gating mode"]
pub type UcagmR = crate::BitReader;
#[doc = "Field `UCAGM` writer - UART Clock Auto Gating mode"]
pub type UcagmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDM` reader - Fractional Division Mode"]
pub type FdmR = crate::BitReader;
#[doc = "Field `FDM` writer - Fractional Division Mode"]
pub type FdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESM` reader - RX Dual Edge Sampling Mode"]
pub type DesmR = crate::BitReader;
#[doc = "Field `DESM` writer - RX Dual Edge Sampling Mode"]
pub type DesmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
    #[inline(always)]
    pub fn rx_thd_val(&self) -> RxThdValR {
        RxThdValR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Parity Enable"]
    #[inline(always)]
    pub fn par_en(&self) -> ParEnR {
        ParEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
    #[inline(always)]
    pub fn par_eo(&self) -> ParEoR {
        ParEoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects parity based on 1s or 0s count (when PAREN=1)"]
    #[inline(always)]
    pub fn par_md(&self) -> ParMdR {
        ParMdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS Sampling Disable"]
    #[inline(always)]
    pub fn cts_dis(&self) -> CtsDisR {
        CtsDisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn tx_flush(&self) -> TxFlushR {
        TxFlushR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn rx_flush(&self) -> RxFlushR {
        RxFlushR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Selects UART character size"]
    #[inline(always)]
    pub fn char_size(&self) -> CharSizeR {
        CharSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Selects the number of stop bits that will be generated"]
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables/disables hardware flow control"]
    #[inline(always)]
    pub fn hfc_en(&self) -> HfcEnR {
        HfcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Flow Control RTS Mode"]
    #[inline(always)]
    pub fn rtsdc(&self) -> RtsdcR {
        RtsdcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Baud clock enable"]
    #[inline(always)]
    pub fn bclken(&self) -> BclkenR {
        BclkenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
    #[inline(always)]
    pub fn bclksrc(&self) -> BclksrcR {
        BclksrcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Data/Parity bit frame error detection enable"]
    #[inline(always)]
    pub fn dpfe_en(&self) -> DpfeEnR {
        DpfeEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Baud clock Ready read only bit"]
    #[inline(always)]
    pub fn bclkrdy(&self) -> BclkrdyR {
        BclkrdyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART Clock Auto Gating mode"]
    #[inline(always)]
    pub fn ucagm(&self) -> UcagmR {
        UcagmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fractional Division Mode"]
    #[inline(always)]
    pub fn fdm(&self) -> FdmR {
        FdmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RX Dual Edge Sampling Mode"]
    #[inline(always)]
    pub fn desm(&self) -> DesmR {
        DesmR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field specifies the depth of receive FIFO for interrupt generation (value 0 and > 16 are ignored)"]
    #[inline(always)]
    pub fn rx_thd_val(&mut self) -> RxThdValW<CtrlSpec> {
        RxThdValW::new(self, 0)
    }
    #[doc = "Bit 4 - Parity Enable"]
    #[inline(always)]
    pub fn par_en(&mut self) -> ParEnW<CtrlSpec> {
        ParEnW::new(self, 4)
    }
    #[doc = "Bit 5 - when PAREN=1 selects odd or even parity odd is 1 even is 0"]
    #[inline(always)]
    pub fn par_eo(&mut self) -> ParEoW<CtrlSpec> {
        ParEoW::new(self, 5)
    }
    #[doc = "Bit 6 - Selects parity based on 1s or 0s count (when PAREN=1)"]
    #[inline(always)]
    pub fn par_md(&mut self) -> ParMdW<CtrlSpec> {
        ParMdW::new(self, 6)
    }
    #[doc = "Bit 7 - CTS Sampling Disable"]
    #[inline(always)]
    pub fn cts_dis(&mut self) -> CtsDisW<CtrlSpec> {
        CtsDisW::new(self, 7)
    }
    #[doc = "Bit 8 - Flushes the TX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn tx_flush(&mut self) -> TxFlushW<CtrlSpec> {
        TxFlushW::new(self, 8)
    }
    #[doc = "Bit 9 - Flushes the RX FIFO buffer. This bit is automatically cleared by hardware when flush is completed."]
    #[inline(always)]
    pub fn rx_flush(&mut self) -> RxFlushW<CtrlSpec> {
        RxFlushW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Selects UART character size"]
    #[inline(always)]
    pub fn char_size(&mut self) -> CharSizeW<CtrlSpec> {
        CharSizeW::new(self, 10)
    }
    #[doc = "Bit 12 - Selects the number of stop bits that will be generated"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> StopbitsW<CtrlSpec> {
        StopbitsW::new(self, 12)
    }
    #[doc = "Bit 13 - Enables/disables hardware flow control"]
    #[inline(always)]
    pub fn hfc_en(&mut self) -> HfcEnW<CtrlSpec> {
        HfcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Hardware Flow Control RTS Mode"]
    #[inline(always)]
    pub fn rtsdc(&mut self) -> RtsdcW<CtrlSpec> {
        RtsdcW::new(self, 14)
    }
    #[doc = "Bit 15 - Baud clock enable"]
    #[inline(always)]
    pub fn bclken(&mut self) -> BclkenW<CtrlSpec> {
        BclkenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - To select the UART clock source for the UART engine (except APB registers). Secondary clock (used for baud rate generator) can be asynchronous from APB clock."]
    #[inline(always)]
    pub fn bclksrc(&mut self) -> BclksrcW<CtrlSpec> {
        BclksrcW::new(self, 16)
    }
    #[doc = "Bit 18 - Data/Parity bit frame error detection enable"]
    #[inline(always)]
    pub fn dpfe_en(&mut self) -> DpfeEnW<CtrlSpec> {
        DpfeEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Baud clock Ready read only bit"]
    #[inline(always)]
    pub fn bclkrdy(&mut self) -> BclkrdyW<CtrlSpec> {
        BclkrdyW::new(self, 19)
    }
    #[doc = "Bit 20 - UART Clock Auto Gating mode"]
    #[inline(always)]
    pub fn ucagm(&mut self) -> UcagmW<CtrlSpec> {
        UcagmW::new(self, 20)
    }
    #[doc = "Bit 21 - Fractional Division Mode"]
    #[inline(always)]
    pub fn fdm(&mut self) -> FdmW<CtrlSpec> {
        FdmW::new(self, 21)
    }
    #[doc = "Bit 22 - RX Dual Edge Sampling Mode"]
    #[inline(always)]
    pub fn desm(&mut self) -> DesmW<CtrlSpec> {
        DesmW::new(self, 22)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
