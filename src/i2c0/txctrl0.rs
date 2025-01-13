#[doc = "Register `TXCTRL0` reader"]
pub type R = crate::R<Txctrl0Spec>;
#[doc = "Register `TXCTRL0` writer"]
pub type W = crate::W<Txctrl0Spec>;
#[doc = "Field `PRELOAD_MODE` reader - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub type PreloadModeR = crate::BitReader;
#[doc = "Field `PRELOAD_MODE` writer - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
pub type PreloadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit FIFO Ready Manual Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxReadyMode {
    #[doc = "0: HW control of I2CTXRDY enabled."]
    En = 0,
    #[doc = "1: HW control of I2CTXRDY disabled."]
    Dis = 1,
}
impl From<TxReadyMode> for bool {
    #[inline(always)]
    fn from(variant: TxReadyMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_READY_MODE` reader - Transmit FIFO Ready Manual Mode."]
pub type TxReadyModeR = crate::BitReader<TxReadyMode>;
impl TxReadyModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxReadyMode {
        match self.bits {
            false => TxReadyMode::En,
            true => TxReadyMode::Dis,
        }
    }
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxReadyMode::En
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxReadyMode::Dis
    }
}
#[doc = "Field `TX_READY_MODE` writer - Transmit FIFO Ready Manual Mode."]
pub type TxReadyModeW<'a, REG> = crate::BitWriter<'a, REG, TxReadyMode>;
impl<'a, REG> TxReadyModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW control of I2CTXRDY enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxReadyMode::En)
    }
    #[doc = "HW control of I2CTXRDY disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxReadyMode::Dis)
    }
}
#[doc = "TX FIFO General Call Address Match Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcAddrFlushDis {
    #[doc = "0: Enabled."]
    En = 0,
    #[doc = "1: Disabled."]
    Dis = 1,
}
impl From<GcAddrFlushDis> for bool {
    #[inline(always)]
    fn from(variant: GcAddrFlushDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC_ADDR_FLUSH_DIS` reader - TX FIFO General Call Address Match Auto Flush Disable."]
pub type GcAddrFlushDisR = crate::BitReader<GcAddrFlushDis>;
impl GcAddrFlushDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GcAddrFlushDis {
        match self.bits {
            false => GcAddrFlushDis::En,
            true => GcAddrFlushDis::Dis,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GcAddrFlushDis::En
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GcAddrFlushDis::Dis
    }
}
#[doc = "Field `GC_ADDR_FLUSH_DIS` writer - TX FIFO General Call Address Match Auto Flush Disable."]
pub type GcAddrFlushDisW<'a, REG> = crate::BitWriter<'a, REG, GcAddrFlushDis>;
impl<'a, REG> GcAddrFlushDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrFlushDis::En)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrFlushDis::Dis)
    }
}
#[doc = "TX FIFO Slave Address Match Write Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrAddrFlushDis {
    #[doc = "0: Enabled."]
    En = 0,
    #[doc = "1: Disabled."]
    Dis = 1,
}
impl From<WrAddrFlushDis> for bool {
    #[inline(always)]
    fn from(variant: WrAddrFlushDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_ADDR_FLUSH_DIS` reader - TX FIFO Slave Address Match Write Auto Flush Disable."]
pub type WrAddrFlushDisR = crate::BitReader<WrAddrFlushDis>;
impl WrAddrFlushDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrAddrFlushDis {
        match self.bits {
            false => WrAddrFlushDis::En,
            true => WrAddrFlushDis::Dis,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WrAddrFlushDis::En
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WrAddrFlushDis::Dis
    }
}
#[doc = "Field `WR_ADDR_FLUSH_DIS` writer - TX FIFO Slave Address Match Write Auto Flush Disable."]
pub type WrAddrFlushDisW<'a, REG> = crate::BitWriter<'a, REG, WrAddrFlushDis>;
impl<'a, REG> WrAddrFlushDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WrAddrFlushDis::En)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WrAddrFlushDis::Dis)
    }
}
#[doc = "TX FIFO Slave Address Match Read Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdAddrFlushDis {
    #[doc = "0: Enabled."]
    En = 0,
    #[doc = "1: Disabled."]
    Dis = 1,
}
impl From<RdAddrFlushDis> for bool {
    #[inline(always)]
    fn from(variant: RdAddrFlushDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_ADDR_FLUSH_DIS` reader - TX FIFO Slave Address Match Read Auto Flush Disable."]
pub type RdAddrFlushDisR = crate::BitReader<RdAddrFlushDis>;
impl RdAddrFlushDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdAddrFlushDis {
        match self.bits {
            false => RdAddrFlushDis::En,
            true => RdAddrFlushDis::Dis,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RdAddrFlushDis::En
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RdAddrFlushDis::Dis
    }
}
#[doc = "Field `RD_ADDR_FLUSH_DIS` writer - TX FIFO Slave Address Match Read Auto Flush Disable."]
pub type RdAddrFlushDisW<'a, REG> = crate::BitWriter<'a, REG, RdAddrFlushDis>;
impl<'a, REG> RdAddrFlushDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RdAddrFlushDis::En)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RdAddrFlushDis::Dis)
    }
}
#[doc = "TX FIFO received NACK Auto Flush Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NackFlushDis {
    #[doc = "0: Enabled."]
    En = 0,
    #[doc = "1: Disabled."]
    Dis = 1,
}
impl From<NackFlushDis> for bool {
    #[inline(always)]
    fn from(variant: NackFlushDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK_FLUSH_DIS` reader - TX FIFO received NACK Auto Flush Disable."]
pub type NackFlushDisR = crate::BitReader<NackFlushDis>;
impl NackFlushDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NackFlushDis {
        match self.bits {
            false => NackFlushDis::En,
            true => NackFlushDis::Dis,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == NackFlushDis::En
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == NackFlushDis::Dis
    }
}
#[doc = "Field `NACK_FLUSH_DIS` writer - TX FIFO received NACK Auto Flush Disable."]
pub type NackFlushDisW<'a, REG> = crate::BitWriter<'a, REG, NackFlushDis>;
impl<'a, REG> NackFlushDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(NackFlushDis::En)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(NackFlushDis::Dis)
    }
}
#[doc = "Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: FIFO not flushed."]
    NotFlushed = 0,
    #[doc = "1: Flush TX_FIFO."]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` reader - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub type FlushR = crate::BitReader<Flush>;
impl FlushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flush {
        match self.bits {
            false => Flush::NotFlushed,
            true => Flush::Flush,
        }
    }
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn is_not_flushed(&self) -> bool {
        *self == Flush::NotFlushed
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Flush::Flush
    }
}
#[doc = "Field `FLUSH` writer - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO not flushed."]
    #[inline(always)]
    pub fn not_flushed(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::NotFlushed)
    }
    #[doc = "Flush TX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
#[doc = "Field `THD_VAL` reader - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub type ThdValR = crate::FieldReader;
#[doc = "Field `THD_VAL` writer - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
pub type ThdValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn preload_mode(&self) -> PreloadModeR {
        PreloadModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&self) -> TxReadyModeR {
        TxReadyModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO General Call Address Match Auto Flush Disable."]
    #[inline(always)]
    pub fn gc_addr_flush_dis(&self) -> GcAddrFlushDisR {
        GcAddrFlushDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Slave Address Match Write Auto Flush Disable."]
    #[inline(always)]
    pub fn wr_addr_flush_dis(&self) -> WrAddrFlushDisR {
        WrAddrFlushDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Slave Address Match Read Auto Flush Disable."]
    #[inline(always)]
    pub fn rd_addr_flush_dis(&self) -> RdAddrFlushDisR {
        RdAddrFlushDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO received NACK Auto Flush Disable."]
    #[inline(always)]
    pub fn nack_flush_dis(&self) -> NackFlushDisR {
        NackFlushDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_val(&self) -> ThdValR {
        ThdValR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preaload Mode. Setting this bit will allow for high speed application to preload the transmit FIFO prior to Slave Address Match."]
    #[inline(always)]
    pub fn preload_mode(&mut self) -> PreloadModeW<Txctrl0Spec> {
        PreloadModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Ready Manual Mode."]
    #[inline(always)]
    pub fn tx_ready_mode(&mut self) -> TxReadyModeW<Txctrl0Spec> {
        TxReadyModeW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO General Call Address Match Auto Flush Disable."]
    #[inline(always)]
    pub fn gc_addr_flush_dis(&mut self) -> GcAddrFlushDisW<Txctrl0Spec> {
        GcAddrFlushDisW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO Slave Address Match Write Auto Flush Disable."]
    #[inline(always)]
    pub fn wr_addr_flush_dis(&mut self) -> WrAddrFlushDisW<Txctrl0Spec> {
        WrAddrFlushDisW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO Slave Address Match Read Auto Flush Disable."]
    #[inline(always)]
    pub fn rd_addr_flush_dis(&mut self) -> RdAddrFlushDisW<Txctrl0Spec> {
        RdAddrFlushDisW::new(self, 4)
    }
    #[doc = "Bit 5 - TX FIFO received NACK Auto Flush Disable."]
    #[inline(always)]
    pub fn nack_flush_dis(&mut self) -> NackFlushDisW<Txctrl0Spec> {
        NackFlushDisW::new(self, 5)
    }
    #[doc = "Bit 7 - Transmit FIFO Flush. This bit is automatically cleared to 0 after the operation."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<Txctrl0Spec> {
        FlushW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Threshold. These bits define the TX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_val(&mut self) -> ThdValW<Txctrl0Spec> {
        ThdValW::new(self, 8)
    }
}
#[doc = "Transmit Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txctrl0Spec;
impl crate::RegisterSpec for Txctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl0::R`](R) reader structure"]
impl crate::Readable for Txctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`txctrl0::W`](W) writer structure"]
impl crate::Writable for Txctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTRL0 to value 0"]
impl crate::Resettable for Txctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
