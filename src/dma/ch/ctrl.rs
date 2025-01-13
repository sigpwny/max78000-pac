#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Dis,
            true => En::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
#[doc = "Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rlden {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Rlden> for bool {
    #[inline(always)]
    fn from(variant: Rlden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDEN` reader - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RldenR = crate::BitReader<Rlden>;
impl RldenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rlden {
        match self.bits {
            false => Rlden::Dis,
            true => Rlden::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rlden::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rlden::En
    }
}
#[doc = "Field `RLDEN` writer - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
pub type RldenW<'a, REG> = crate::BitWriter<'a, REG, Rlden>;
impl<'a, REG> RldenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rlden::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rlden::En)
    }
}
#[doc = "DMA Priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pri {
    #[doc = "0: Highest Priority."]
    High = 0,
    #[doc = "1: Medium High Priority."]
    MedHigh = 1,
    #[doc = "2: Medium Low Priority."]
    MedLow = 2,
    #[doc = "3: Lowest Priority."]
    Low = 3,
}
impl From<Pri> for u8 {
    #[inline(always)]
    fn from(variant: Pri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pri {
    type Ux = u8;
}
impl crate::IsEnum for Pri {}
#[doc = "Field `PRI` reader - DMA Priority."]
pub type PriR = crate::FieldReader<Pri>;
impl PriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pri {
        match self.bits {
            0 => Pri::High,
            1 => Pri::MedHigh,
            2 => Pri::MedLow,
            3 => Pri::Low,
            _ => unreachable!(),
        }
    }
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pri::High
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == Pri::MedHigh
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == Pri::MedLow
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pri::Low
    }
}
#[doc = "Field `PRI` writer - DMA Priority."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pri, crate::Safe>;
impl<'a, REG> PriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest Priority."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::High)
    }
    #[doc = "Medium High Priority."]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::MedHigh)
    }
    #[doc = "Medium Low Priority."]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::MedLow)
    }
    #[doc = "Lowest Priority."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Pri::Low)
    }
}
#[doc = "Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Request {
    #[doc = "0: Memory To Memory"]
    Memtomem = 0,
    #[doc = "1: SPI1 RX"]
    Spi1rx = 1,
    #[doc = "4: UART0 RX"]
    Uart0rx = 4,
    #[doc = "5: UART1 RX"]
    Uart1rx = 5,
    #[doc = "7: I2C0 RX"]
    I2c0rx = 7,
    #[doc = "8: I2C1 RX"]
    I2c1rx = 8,
    #[doc = "9: ADC"]
    Adc = 9,
    #[doc = "10: I2C2 RX"]
    I2c2rx = 10,
    #[doc = "14: UART2 RX"]
    Uart2rx = 14,
    #[doc = "15: SPI0 RX"]
    Spi0rx = 15,
    #[doc = "16: AES RX"]
    Aesrx = 16,
    #[doc = "28: UART3 RX"]
    Uart3rx = 28,
    #[doc = "30: I2S RX"]
    I2srx = 30,
    #[doc = "33: SPI1 TX"]
    Spi1tx = 33,
    #[doc = "36: UART0 TX"]
    Uart0tx = 36,
    #[doc = "37: UART1 TX"]
    Uart1tx = 37,
    #[doc = "39: I2C0 TX"]
    I2c0tx = 39,
    #[doc = "40: I2C1 TX"]
    I2c1tx = 40,
    #[doc = "42: I2C2 TX"]
    I2c2tx = 42,
    #[doc = "44: CRC TX"]
    Crctx = 44,
    #[doc = "45: PCIF TX"]
    Pciftx = 45,
    #[doc = "46: UART2 TX"]
    Uart2tx = 46,
    #[doc = "47: SPI0 TX"]
    Spi0tx = 47,
    #[doc = "48: AES TX"]
    Aestx = 48,
    #[doc = "60: UART3 TX"]
    Uart3tx = 60,
    #[doc = "62: I2S TX"]
    I2stx = 62,
}
impl From<Request> for u8 {
    #[inline(always)]
    fn from(variant: Request) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Request {
    type Ux = u8;
}
impl crate::IsEnum for Request {}
#[doc = "Field `REQUEST` reader - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type RequestR = crate::FieldReader<Request>;
impl RequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Request> {
        match self.bits {
            0 => Some(Request::Memtomem),
            1 => Some(Request::Spi1rx),
            4 => Some(Request::Uart0rx),
            5 => Some(Request::Uart1rx),
            7 => Some(Request::I2c0rx),
            8 => Some(Request::I2c1rx),
            9 => Some(Request::Adc),
            10 => Some(Request::I2c2rx),
            14 => Some(Request::Uart2rx),
            15 => Some(Request::Spi0rx),
            16 => Some(Request::Aesrx),
            28 => Some(Request::Uart3rx),
            30 => Some(Request::I2srx),
            33 => Some(Request::Spi1tx),
            36 => Some(Request::Uart0tx),
            37 => Some(Request::Uart1tx),
            39 => Some(Request::I2c0tx),
            40 => Some(Request::I2c1tx),
            42 => Some(Request::I2c2tx),
            44 => Some(Request::Crctx),
            45 => Some(Request::Pciftx),
            46 => Some(Request::Uart2tx),
            47 => Some(Request::Spi0tx),
            48 => Some(Request::Aestx),
            60 => Some(Request::Uart3tx),
            62 => Some(Request::I2stx),
            _ => None,
        }
    }
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn is_memtomem(&self) -> bool {
        *self == Request::Memtomem
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        *self == Request::Spi1rx
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == Request::Uart0rx
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == Request::Uart1rx
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn is_i2c0rx(&self) -> bool {
        *self == Request::I2c0rx
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        *self == Request::I2c1rx
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Request::Adc
    }
    #[doc = "I2C2 RX"]
    #[inline(always)]
    pub fn is_i2c2rx(&self) -> bool {
        *self == Request::I2c2rx
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn is_uart2rx(&self) -> bool {
        *self == Request::Uart2rx
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn is_spi0rx(&self) -> bool {
        *self == Request::Spi0rx
    }
    #[doc = "AES RX"]
    #[inline(always)]
    pub fn is_aesrx(&self) -> bool {
        *self == Request::Aesrx
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn is_uart3rx(&self) -> bool {
        *self == Request::Uart3rx
    }
    #[doc = "I2S RX"]
    #[inline(always)]
    pub fn is_i2srx(&self) -> bool {
        *self == Request::I2srx
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        *self == Request::Spi1tx
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == Request::Uart0tx
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == Request::Uart1tx
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn is_i2c0tx(&self) -> bool {
        *self == Request::I2c0tx
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        *self == Request::I2c1tx
    }
    #[doc = "I2C2 TX"]
    #[inline(always)]
    pub fn is_i2c2tx(&self) -> bool {
        *self == Request::I2c2tx
    }
    #[doc = "CRC TX"]
    #[inline(always)]
    pub fn is_crctx(&self) -> bool {
        *self == Request::Crctx
    }
    #[doc = "PCIF TX"]
    #[inline(always)]
    pub fn is_pciftx(&self) -> bool {
        *self == Request::Pciftx
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn is_uart2tx(&self) -> bool {
        *self == Request::Uart2tx
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn is_spi0tx(&self) -> bool {
        *self == Request::Spi0tx
    }
    #[doc = "AES TX"]
    #[inline(always)]
    pub fn is_aestx(&self) -> bool {
        *self == Request::Aestx
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn is_uart3tx(&self) -> bool {
        *self == Request::Uart3tx
    }
    #[doc = "I2S TX"]
    #[inline(always)]
    pub fn is_i2stx(&self) -> bool {
        *self == Request::I2stx
    }
}
#[doc = "Field `REQUEST` writer - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
pub type RequestW<'a, REG> = crate::FieldWriter<'a, REG, 6, Request>;
impl<'a, REG> RequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory To Memory"]
    #[inline(always)]
    pub fn memtomem(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Memtomem)
    }
    #[doc = "SPI1 RX"]
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Spi1rx)
    }
    #[doc = "UART0 RX"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart0rx)
    }
    #[doc = "UART1 RX"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart1rx)
    }
    #[doc = "I2C0 RX"]
    #[inline(always)]
    pub fn i2c0rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c0rx)
    }
    #[doc = "I2C1 RX"]
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c1rx)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Adc)
    }
    #[doc = "I2C2 RX"]
    #[inline(always)]
    pub fn i2c2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c2rx)
    }
    #[doc = "UART2 RX"]
    #[inline(always)]
    pub fn uart2rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart2rx)
    }
    #[doc = "SPI0 RX"]
    #[inline(always)]
    pub fn spi0rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Spi0rx)
    }
    #[doc = "AES RX"]
    #[inline(always)]
    pub fn aesrx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Aesrx)
    }
    #[doc = "UART3 RX"]
    #[inline(always)]
    pub fn uart3rx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart3rx)
    }
    #[doc = "I2S RX"]
    #[inline(always)]
    pub fn i2srx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2srx)
    }
    #[doc = "SPI1 TX"]
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Spi1tx)
    }
    #[doc = "UART0 TX"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart0tx)
    }
    #[doc = "UART1 TX"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart1tx)
    }
    #[doc = "I2C0 TX"]
    #[inline(always)]
    pub fn i2c0tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c0tx)
    }
    #[doc = "I2C1 TX"]
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c1tx)
    }
    #[doc = "I2C2 TX"]
    #[inline(always)]
    pub fn i2c2tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2c2tx)
    }
    #[doc = "CRC TX"]
    #[inline(always)]
    pub fn crctx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Crctx)
    }
    #[doc = "PCIF TX"]
    #[inline(always)]
    pub fn pciftx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Pciftx)
    }
    #[doc = "UART2 TX"]
    #[inline(always)]
    pub fn uart2tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart2tx)
    }
    #[doc = "SPI0 TX"]
    #[inline(always)]
    pub fn spi0tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Spi0tx)
    }
    #[doc = "AES TX"]
    #[inline(always)]
    pub fn aestx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Aestx)
    }
    #[doc = "UART3 TX"]
    #[inline(always)]
    pub fn uart3tx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::Uart3tx)
    }
    #[doc = "I2S TX"]
    #[inline(always)]
    pub fn i2stx(self) -> &'a mut crate::W<REG> {
        self.variant(Request::I2stx)
    }
}
#[doc = "Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToWait {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<ToWait> for bool {
    #[inline(always)]
    fn from(variant: ToWait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_WAIT` reader - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type ToWaitR = crate::BitReader<ToWait>;
impl ToWaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToWait {
        match self.bits {
            false => ToWait::Dis,
            true => ToWait::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ToWait::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ToWait::En
    }
}
#[doc = "Field `TO_WAIT` writer - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
pub type ToWaitW<'a, REG> = crate::BitWriter<'a, REG, ToWait>;
impl<'a, REG> ToWaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ToWait::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ToWait::En)
    }
}
#[doc = "Timeout Period Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ToPer {
    #[doc = "0: Timeout of 3 to 4 prescale clocks."]
    To4 = 0,
    #[doc = "1: Timeout of 7 to 8 prescale clocks."]
    To8 = 1,
    #[doc = "2: Timeout of 15 to 16 prescale clocks."]
    To16 = 2,
    #[doc = "3: Timeout of 31 to 32 prescale clocks."]
    To32 = 3,
    #[doc = "4: Timeout of 63 to 64 prescale clocks."]
    To64 = 4,
    #[doc = "5: Timeout of 127 to 128 prescale clocks."]
    To128 = 5,
    #[doc = "6: Timeout of 255 to 256 prescale clocks."]
    To256 = 6,
    #[doc = "7: Timeout of 511 to 512 prescale clocks."]
    To512 = 7,
}
impl From<ToPer> for u8 {
    #[inline(always)]
    fn from(variant: ToPer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ToPer {
    type Ux = u8;
}
impl crate::IsEnum for ToPer {}
#[doc = "Field `TO_PER` reader - Timeout Period Select."]
pub type ToPerR = crate::FieldReader<ToPer>;
impl ToPerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToPer {
        match self.bits {
            0 => ToPer::To4,
            1 => ToPer::To8,
            2 => ToPer::To16,
            3 => ToPer::To32,
            4 => ToPer::To64,
            5 => ToPer::To128,
            6 => ToPer::To256,
            7 => ToPer::To512,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn is_to4(&self) -> bool {
        *self == ToPer::To4
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn is_to8(&self) -> bool {
        *self == ToPer::To8
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn is_to16(&self) -> bool {
        *self == ToPer::To16
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn is_to32(&self) -> bool {
        *self == ToPer::To32
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn is_to64(&self) -> bool {
        *self == ToPer::To64
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn is_to128(&self) -> bool {
        *self == ToPer::To128
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn is_to256(&self) -> bool {
        *self == ToPer::To256
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn is_to512(&self) -> bool {
        *self == ToPer::To512
    }
}
#[doc = "Field `TO_PER` writer - Timeout Period Select."]
pub type ToPerW<'a, REG> = crate::FieldWriter<'a, REG, 3, ToPer, crate::Safe>;
impl<'a, REG> ToPerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout of 3 to 4 prescale clocks."]
    #[inline(always)]
    pub fn to4(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To4)
    }
    #[doc = "Timeout of 7 to 8 prescale clocks."]
    #[inline(always)]
    pub fn to8(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To8)
    }
    #[doc = "Timeout of 15 to 16 prescale clocks."]
    #[inline(always)]
    pub fn to16(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To16)
    }
    #[doc = "Timeout of 31 to 32 prescale clocks."]
    #[inline(always)]
    pub fn to32(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To32)
    }
    #[doc = "Timeout of 63 to 64 prescale clocks."]
    #[inline(always)]
    pub fn to64(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To64)
    }
    #[doc = "Timeout of 127 to 128 prescale clocks."]
    #[inline(always)]
    pub fn to128(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To128)
    }
    #[doc = "Timeout of 255 to 256 prescale clocks."]
    #[inline(always)]
    pub fn to256(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To256)
    }
    #[doc = "Timeout of 511 to 512 prescale clocks."]
    #[inline(always)]
    pub fn to512(self) -> &'a mut crate::W<REG> {
        self.variant(ToPer::To512)
    }
}
#[doc = "Pre-Scale Select. Selects the Pre-Scale divider for timer clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ToClkdiv {
    #[doc = "0: Disable timer."]
    Dis = 0,
    #[doc = "1: hclk / 256."]
    Div256 = 1,
    #[doc = "2: hclk / 64k."]
    Div64k = 2,
    #[doc = "3: hclk / 16M."]
    Div16m = 3,
}
impl From<ToClkdiv> for u8 {
    #[inline(always)]
    fn from(variant: ToClkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ToClkdiv {
    type Ux = u8;
}
impl crate::IsEnum for ToClkdiv {}
#[doc = "Field `TO_CLKDIV` reader - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type ToClkdivR = crate::FieldReader<ToClkdiv>;
impl ToClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToClkdiv {
        match self.bits {
            0 => ToClkdiv::Dis,
            1 => ToClkdiv::Div256,
            2 => ToClkdiv::Div64k,
            3 => ToClkdiv::Div16m,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ToClkdiv::Dis
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == ToClkdiv::Div256
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        *self == ToClkdiv::Div64k
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn is_div16m(&self) -> bool {
        *self == ToClkdiv::Div16m
    }
}
#[doc = "Field `TO_CLKDIV` writer - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
pub type ToClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, ToClkdiv, crate::Safe>;
impl<'a, REG> ToClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable timer."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ToClkdiv::Dis)
    }
    #[doc = "hclk / 256."]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(ToClkdiv::Div256)
    }
    #[doc = "hclk / 64k."]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut crate::W<REG> {
        self.variant(ToClkdiv::Div64k)
    }
    #[doc = "hclk / 16M."]
    #[inline(always)]
    pub fn div16m(self) -> &'a mut crate::W<REG> {
        self.variant(ToClkdiv::Div16m)
    }
}
#[doc = "Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcwd {
    #[doc = "0: Byte."]
    Byte = 0,
    #[doc = "1: Halfword."]
    HalfWord = 1,
    #[doc = "2: Word."]
    Word = 2,
}
impl From<Srcwd> for u8 {
    #[inline(always)]
    fn from(variant: Srcwd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcwd {
    type Ux = u8;
}
impl crate::IsEnum for Srcwd {}
#[doc = "Field `SRCWD` reader - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SrcwdR = crate::FieldReader<Srcwd>;
impl SrcwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcwd> {
        match self.bits {
            0 => Some(Srcwd::Byte),
            1 => Some(Srcwd::HalfWord),
            2 => Some(Srcwd::Word),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Srcwd::Byte
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == Srcwd::HalfWord
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Srcwd::Word
    }
}
#[doc = "Field `SRCWD` writer - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
pub type SrcwdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srcwd>;
impl<'a, REG> SrcwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Srcwd::Byte)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(Srcwd::HalfWord)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Srcwd::Word)
    }
}
#[doc = "Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcinc {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Srcinc> for bool {
    #[inline(always)]
    fn from(variant: Srcinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCINC` reader - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SrcincR = crate::BitReader<Srcinc>;
impl SrcincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcinc {
        match self.bits {
            false => Srcinc::Dis,
            true => Srcinc::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Srcinc::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Srcinc::En
    }
}
#[doc = "Field `SRCINC` writer - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
pub type SrcincW<'a, REG> = crate::BitWriter<'a, REG, Srcinc>;
impl<'a, REG> SrcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::En)
    }
}
#[doc = "Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dstwd {
    #[doc = "0: Byte."]
    Byte = 0,
    #[doc = "1: Halfword."]
    HalfWord = 1,
    #[doc = "2: Word."]
    Word = 2,
}
impl From<Dstwd> for u8 {
    #[inline(always)]
    fn from(variant: Dstwd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dstwd {
    type Ux = u8;
}
impl crate::IsEnum for Dstwd {}
#[doc = "Field `DSTWD` reader - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DstwdR = crate::FieldReader<Dstwd>;
impl DstwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dstwd> {
        match self.bits {
            0 => Some(Dstwd::Byte),
            1 => Some(Dstwd::HalfWord),
            2 => Some(Dstwd::Word),
            _ => None,
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dstwd::Byte
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == Dstwd::HalfWord
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dstwd::Word
    }
}
#[doc = "Field `DSTWD` writer - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
pub type DstwdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dstwd>;
impl<'a, REG> DstwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dstwd::Byte)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(Dstwd::HalfWord)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dstwd::Word)
    }
}
#[doc = "Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstinc {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Dstinc> for bool {
    #[inline(always)]
    fn from(variant: Dstinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTINC` reader - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DstincR = crate::BitReader<Dstinc>;
impl DstincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstinc {
        match self.bits {
            false => Dstinc::Dis,
            true => Dstinc::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dstinc::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dstinc::En
    }
}
#[doc = "Field `DSTINC` writer - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
pub type DstincW<'a, REG> = crate::BitWriter<'a, REG, Dstinc>;
impl<'a, REG> DstincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::En)
    }
}
#[doc = "Field `BURST_SIZE` reader - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BurstSizeR = crate::FieldReader;
#[doc = "Field `BURST_SIZE` writer - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
pub type BurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisIe {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<DisIe> for bool {
    #[inline(always)]
    fn from(variant: DisIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_IE` reader - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DisIeR = crate::BitReader<DisIe>;
impl DisIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisIe {
        match self.bits {
            false => DisIe::Dis,
            true => DisIe::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DisIe::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DisIe::En
    }
}
#[doc = "Field `DIS_IE` writer - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
pub type DisIeW<'a, REG> = crate::BitWriter<'a, REG, DisIe>;
impl<'a, REG> DisIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DisIe::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DisIe::En)
    }
}
#[doc = "Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtzIe {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<CtzIe> for bool {
    #[inline(always)]
    fn from(variant: CtzIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTZ_IE` reader - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CtzIeR = crate::BitReader<CtzIe>;
impl CtzIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtzIe {
        match self.bits {
            false => CtzIe::Dis,
            true => CtzIe::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CtzIe::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CtzIe::En
    }
}
#[doc = "Field `CTZ_IE` writer - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
pub type CtzIeW<'a, REG> = crate::BitWriter<'a, REG, CtzIe>;
impl<'a, REG> CtzIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CtzIe::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CtzIe::En)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&self) -> RldenR {
        RldenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn request(&self) -> RequestR {
        RequestR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn to_wait(&self) -> ToWaitR {
        ToWaitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    pub fn to_per(&self) -> ToPerR {
        ToPerR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn to_clkdiv(&self) -> ToClkdivR {
        ToClkdivR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&self) -> SrcwdR {
        SrcwdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&self) -> DstwdR {
        DstwdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn burst_size(&self) -> BurstSizeR {
        BurstSizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn dis_ie(&self) -> DisIeR {
        DisIeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctz_ie(&self) -> CtzIeR {
        CtzIeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable. This bit is automatically cleared when DMA_ST.CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reload Enable. Setting this bit to 1 enables DMA_SRC, DMA_DST and DMA_CNT to be reloaded with their corresponding reload registers upon count-to-zero. This bit is also writeable in the Count Reload Register. Refer to the description on Buffer Chaining for use of this bit. If buffer chaining is not used this bit must be written with a 0. This bit should be set after the reload registers have been programmed."]
    #[inline(always)]
    pub fn rlden(&mut self) -> RldenW<CtrlSpec> {
        RldenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - DMA Priority."]
    #[inline(always)]
    pub fn pri(&mut self) -> PriW<CtrlSpec> {
        PriW::new(self, 2)
    }
    #[doc = "Bits 4:9 - Request Select. Select DMA request line for this channel. If memory-to-memory is selected, the channel operates as if the request is always active."]
    #[inline(always)]
    pub fn request(&mut self) -> RequestW<CtrlSpec> {
        RequestW::new(self, 4)
    }
    #[doc = "Bit 10 - Request Wait Enable. When enabled, delay timer start until DMA request transitions from active to inactive."]
    #[inline(always)]
    pub fn to_wait(&mut self) -> ToWaitW<CtrlSpec> {
        ToWaitW::new(self, 10)
    }
    #[doc = "Bits 11:13 - Timeout Period Select."]
    #[inline(always)]
    pub fn to_per(&mut self) -> ToPerW<CtrlSpec> {
        ToPerW::new(self, 11)
    }
    #[doc = "Bits 14:15 - Pre-Scale Select. Selects the Pre-Scale divider for timer clock input."]
    #[inline(always)]
    pub fn to_clkdiv(&mut self) -> ToClkdivW<CtrlSpec> {
        ToClkdivW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Source Width. In most cases, this will be the data width of each AHB transactions. However, the width will be reduced in the cases where DMA_CNT indicates a smaller value."]
    #[inline(always)]
    pub fn srcwd(&mut self) -> SrcwdW<CtrlSpec> {
        SrcwdW::new(self, 16)
    }
    #[doc = "Bit 18 - Source Increment Enable. This bit enables DMA_SRC increment upon every AHB transaction. This bit is forced to 0 for DMA receive from peripherals."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<CtrlSpec> {
        SrcincW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Destination Width. Indicates the width of the each AHB transactions to the destination peripheral or memory. (The actual width may be less than this if there are insufficient bytes in the DMA FIFO for the full width)."]
    #[inline(always)]
    pub fn dstwd(&mut self) -> DstwdW<CtrlSpec> {
        DstwdW::new(self, 20)
    }
    #[doc = "Bit 22 - Destination Increment Enable. This bit enables DMA_DST increment upon every AHB transaction. This bit is forced to 0 for DMA transmit to peripherals."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<CtrlSpec> {
        DstincW::new(self, 22)
    }
    #[doc = "Bits 24:28 - Burst Size. The number of bytes to be transferred into and out of the DMA FIFO in a single burst. Burst size equals 1 + value stored in this field."]
    #[inline(always)]
    pub fn burst_size(&mut self) -> BurstSizeW<CtrlSpec> {
        BurstSizeW::new(self, 24)
    }
    #[doc = "Bit 30 - Channel Disable Interrupt Enable. When enabled, the IPEND will be set to 1 whenever CH_ST changes from 1 to 0."]
    #[inline(always)]
    pub fn dis_ie(&mut self) -> DisIeW<CtrlSpec> {
        DisIeW::new(self, 30)
    }
    #[doc = "Bit 31 - Count-to-zero Interrupts Enable. When enabled, the IPEND will be set to 1 whenever a count-to-zero event occurs."]
    #[inline(always)]
    pub fn ctz_ie(&mut self) -> CtzIeW<CtrlSpec> {
        CtzIeW::new(self, 31)
    }
}
#[doc = "DMA Channel Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
