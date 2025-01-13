#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxThd {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<TxThd> for bool {
    #[inline(always)]
    fn from(variant: TxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_THD` reader - TX FIFO Threshold Crossed."]
pub type TxThdR = crate::BitReader<TxThd>;
impl TxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxThd> {
        match self.bits {
            true => Some(TxThd::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TxThd::Clear
    }
}
#[doc = "Field `TX_THD` writer - TX FIFO Threshold Crossed."]
pub type TxThdW<'a, REG> = crate::BitWriter<'a, REG, TxThd>;
impl<'a, REG> TxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Clear)
    }
}
#[doc = "TX FIFO Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEm {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<TxEm> for bool {
    #[inline(always)]
    fn from(variant: TxEm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EM` reader - TX FIFO Empty."]
pub type TxEmR = crate::BitReader<TxEm>;
impl TxEmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxEm> {
        match self.bits {
            true => Some(TxEm::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TxEm::Clear
    }
}
#[doc = "Field `TX_EM` writer - TX FIFO Empty."]
pub type TxEmW<'a, REG> = crate::BitWriter<'a, REG, TxEm>;
impl<'a, REG> TxEmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::Clear)
    }
}
#[doc = "RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxThd {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<RxThd> for bool {
    #[inline(always)]
    fn from(variant: RxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Threshold Crossed."]
pub type RxThdR = crate::BitReader<RxThd>;
impl RxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxThd> {
        match self.bits {
            true => Some(RxThd::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RxThd::Clear
    }
}
#[doc = "Field `RX_THD` writer - RX FIFO Threshold Crossed."]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG, RxThd>;
impl<'a, REG> RxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Clear)
    }
}
#[doc = "RX FIFO FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFull {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<RxFull> for bool {
    #[inline(always)]
    fn from(variant: RxFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL."]
pub type RxFullR = crate::BitReader<RxFull>;
impl RxFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxFull> {
        match self.bits {
            true => Some(RxFull::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RxFull::Clear
    }
}
#[doc = "Field `RX_FULL` writer - RX FIFO FULL."]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG, RxFull>;
impl<'a, REG> RxFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxFull::Clear)
    }
}
#[doc = "Slave Select Asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssa {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<Ssa> for bool {
    #[inline(always)]
    fn from(variant: Ssa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted."]
pub type SsaR = crate::BitReader<Ssa>;
impl SsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssa> {
        match self.bits {
            true => Some(Ssa::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ssa::Clear
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted."]
pub type SsaW<'a, REG> = crate::BitWriter<'a, REG, Ssa>;
impl<'a, REG> SsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ssa::Clear)
    }
}
#[doc = "Slave Select Deasserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssd {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<Ssd> for bool {
    #[inline(always)]
    fn from(variant: Ssd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted."]
pub type SsdR = crate::BitReader<Ssd>;
impl SsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssd> {
        match self.bits {
            true => Some(Ssd::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ssd::Clear
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted."]
pub type SsdW<'a, REG> = crate::BitWriter<'a, REG, Ssd>;
impl<'a, REG> SsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::Clear)
    }
}
#[doc = "Multi-Master Mode Fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT` reader - Multi-Master Mode Fault."]
pub type FaultR = crate::BitReader<Fault>;
impl FaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fault> {
        match self.bits {
            true => Some(Fault::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Fault::Clear
    }
}
#[doc = "Field `FAULT` writer - Multi-Master Mode Fault."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG, Fault>;
impl<'a, REG> FaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::Clear)
    }
}
#[doc = "Slave Abort Detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abort {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<Abort> for bool {
    #[inline(always)]
    fn from(variant: Abort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected."]
pub type AbortR = crate::BitReader<Abort>;
impl AbortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Abort> {
        match self.bits {
            true => Some(Abort::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Abort::Clear
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG, Abort>;
impl<'a, REG> AbortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Abort::Clear)
    }
}
#[doc = "Master Done, set when SPI Master has completed any transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MstDone {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<MstDone> for bool {
    #[inline(always)]
    fn from(variant: MstDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST_DONE` reader - Master Done, set when SPI Master has completed any transactions."]
pub type MstDoneR = crate::BitReader<MstDone>;
impl MstDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MstDone> {
        match self.bits {
            true => Some(MstDone::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MstDone::Clear
    }
}
#[doc = "Field `MST_DONE` writer - Master Done, set when SPI Master has completed any transactions."]
pub type MstDoneW<'a, REG> = crate::BitWriter<'a, REG, MstDone>;
impl<'a, REG> MstDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MstDone::Clear)
    }
}
#[doc = "Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOv {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<TxOv> for bool {
    #[inline(always)]
    fn from(variant: TxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OV` reader - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TxOvR = crate::BitReader<TxOv>;
impl TxOvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxOv> {
        match self.bits {
            true => Some(TxOv::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TxOv::Clear
    }
}
#[doc = "Field `TX_OV` writer - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
pub type TxOvW<'a, REG> = crate::BitWriter<'a, REG, TxOv>;
impl<'a, REG> TxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxOv::Clear)
    }
}
#[doc = "Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxUn {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<TxUn> for bool {
    #[inline(always)]
    fn from(variant: TxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UN` reader - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TxUnR = crate::BitReader<TxUn>;
impl TxUnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxUn> {
        match self.bits {
            true => Some(TxUn::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TxUn::Clear
    }
}
#[doc = "Field `TX_UN` writer - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
pub type TxUnW<'a, REG> = crate::BitWriter<'a, REG, TxUn>;
impl<'a, REG> TxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::Clear)
    }
}
#[doc = "Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOv {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<RxOv> for bool {
    #[inline(always)]
    fn from(variant: RxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OV` reader - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RxOvR = crate::BitReader<RxOv>;
impl RxOvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxOv> {
        match self.bits {
            true => Some(RxOv::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RxOv::Clear
    }
}
#[doc = "Field `RX_OV` writer - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG, RxOv>;
impl<'a, REG> RxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::Clear)
    }
}
#[doc = "Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxUn {
    #[doc = "1: Flag is set when value read is 1. Write 1 to clear this flag."]
    Clear = 1,
}
impl From<RxUn> for bool {
    #[inline(always)]
    fn from(variant: RxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_UN` reader - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RxUnR = crate::BitReader<RxUn>;
impl RxUnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxUn> {
        match self.bits {
            true => Some(RxUn::Clear),
            _ => None,
        }
    }
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RxUn::Clear
    }
}
#[doc = "Field `RX_UN` writer - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
pub type RxUnW<'a, REG> = crate::BitWriter<'a, REG, RxUn>;
impl<'a, REG> RxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag is set when value read is 1. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RxUn::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn mst_done(&self) -> MstDoneR {
        MstDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ov(&self) -> TxOvR {
        TxOvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_un(&self) -> TxUnR {
        TxUnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_un(&self) -> RxUnR {
        RxUnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<IntflSpec> {
        TxThdW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TxEmW<IntflSpec> {
        TxEmW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<IntflSpec> {
        RxThdW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO FULL."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<IntflSpec> {
        RxFullW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Select Asserted."]
    #[inline(always)]
    pub fn ssa(&mut self) -> SsaW<IntflSpec> {
        SsaW::new(self, 4)
    }
    #[doc = "Bit 5 - Slave Select Deasserted."]
    #[inline(always)]
    pub fn ssd(&mut self) -> SsdW<IntflSpec> {
        SsdW::new(self, 5)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault."]
    #[inline(always)]
    pub fn fault(&mut self) -> FaultW<IntflSpec> {
        FaultW::new(self, 8)
    }
    #[doc = "Bit 9 - Slave Abort Detected."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<IntflSpec> {
        AbortW::new(self, 9)
    }
    #[doc = "Bit 11 - Master Done, set when SPI Master has completed any transactions."]
    #[inline(always)]
    pub fn mst_done(&mut self) -> MstDoneW<IntflSpec> {
        MstDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun, set when the AMBA side attempts to write data to a full transmit FIFO."]
    #[inline(always)]
    pub fn tx_ov(&mut self) -> TxOvW<IntflSpec> {
        TxOvW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun, set when the SPI side attempts to read data from an empty transmit FIFO."]
    #[inline(always)]
    pub fn tx_un(&mut self) -> TxUnW<IntflSpec> {
        TxUnW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun, set when the SPI side attempts to write to a full receive FIFO."]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<IntflSpec> {
        RxOvW::new(self, 14)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun, set when the AMBA side attempts to read data from an empty receive FIFO."]
    #[inline(always)]
    pub fn rx_un(&mut self) -> RxUnW<IntflSpec> {
        RxUnW::new(self, 15)
    }
}
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {
    const RESET_VALUE: u32 = 0;
}
