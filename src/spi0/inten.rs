#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "TX FIFO Threshold interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxThd {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<TxThd> for bool {
    #[inline(always)]
    fn from(variant: TxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_THD` reader - TX FIFO Threshold interrupt enable."]
pub type TxThdR = crate::BitReader<TxThd>;
impl TxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxThd {
        match self.bits {
            false => TxThd::Dis,
            true => TxThd::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxThd::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxThd::En
    }
}
#[doc = "Field `TX_THD` writer - TX FIFO Threshold interrupt enable."]
pub type TxThdW<'a, REG> = crate::BitWriter<'a, REG, TxThd>;
impl<'a, REG> TxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::En)
    }
}
#[doc = "TX FIFO Empty interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEm {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<TxEm> for bool {
    #[inline(always)]
    fn from(variant: TxEm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EM` reader - TX FIFO Empty interrupt enable."]
pub type TxEmR = crate::BitReader<TxEm>;
impl TxEmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEm {
        match self.bits {
            false => TxEm::Dis,
            true => TxEm::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxEm::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxEm::En
    }
}
#[doc = "Field `TX_EM` writer - TX FIFO Empty interrupt enable."]
pub type TxEmW<'a, REG> = crate::BitWriter<'a, REG, TxEm>;
impl<'a, REG> TxEmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::En)
    }
}
#[doc = "RX FIFO Threshold Crossed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxThd {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<RxThd> for bool {
    #[inline(always)]
    fn from(variant: RxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Threshold Crossed interrupt enable."]
pub type RxThdR = crate::BitReader<RxThd>;
impl RxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxThd {
        match self.bits {
            false => RxThd::Dis,
            true => RxThd::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxThd::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxThd::En
    }
}
#[doc = "Field `RX_THD` writer - RX FIFO Threshold Crossed interrupt enable."]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG, RxThd>;
impl<'a, REG> RxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::En)
    }
}
#[doc = "RX FIFO FULL interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFull {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<RxFull> for bool {
    #[inline(always)]
    fn from(variant: RxFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - RX FIFO FULL interrupt enable."]
pub type RxFullR = crate::BitReader<RxFull>;
impl RxFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFull {
        match self.bits {
            false => RxFull::Dis,
            true => RxFull::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxFull::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxFull::En
    }
}
#[doc = "Field `RX_FULL` writer - RX FIFO FULL interrupt enable."]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG, RxFull>;
impl<'a, REG> RxFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxFull::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxFull::En)
    }
}
#[doc = "Slave Select Asserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssa {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<Ssa> for bool {
    #[inline(always)]
    fn from(variant: Ssa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSA` reader - Slave Select Asserted interrupt enable."]
pub type SsaR = crate::BitReader<Ssa>;
impl SsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssa {
        match self.bits {
            false => Ssa::Dis,
            true => Ssa::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ssa::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ssa::En
    }
}
#[doc = "Field `SSA` writer - Slave Select Asserted interrupt enable."]
pub type SsaW<'a, REG> = crate::BitWriter<'a, REG, Ssa>;
impl<'a, REG> SsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ssa::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ssa::En)
    }
}
#[doc = "Slave Select Deasserted interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssd {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<Ssd> for bool {
    #[inline(always)]
    fn from(variant: Ssd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSD` reader - Slave Select Deasserted interrupt enable."]
pub type SsdR = crate::BitReader<Ssd>;
impl SsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssd {
        match self.bits {
            false => Ssd::Dis,
            true => Ssd::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ssd::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ssd::En
    }
}
#[doc = "Field `SSD` writer - Slave Select Deasserted interrupt enable."]
pub type SsdW<'a, REG> = crate::BitWriter<'a, REG, Ssd>;
impl<'a, REG> SsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::En)
    }
}
#[doc = "Multi-Master Mode Fault interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<Fault> for bool {
    #[inline(always)]
    fn from(variant: Fault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT` reader - Multi-Master Mode Fault interrupt enable."]
pub type FaultR = crate::BitReader<Fault>;
impl FaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault {
        match self.bits {
            false => Fault::Dis,
            true => Fault::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Fault::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Fault::En
    }
}
#[doc = "Field `FAULT` writer - Multi-Master Mode Fault interrupt enable."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG, Fault>;
impl<'a, REG> FaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Fault::En)
    }
}
#[doc = "Slave Abort Detected interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abort {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<Abort> for bool {
    #[inline(always)]
    fn from(variant: Abort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Slave Abort Detected interrupt enable."]
pub type AbortR = crate::BitReader<Abort>;
impl AbortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abort {
        match self.bits {
            false => Abort::Dis,
            true => Abort::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Abort::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Abort::En
    }
}
#[doc = "Field `ABORT` writer - Slave Abort Detected interrupt enable."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG, Abort>;
impl<'a, REG> AbortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Abort::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Abort::En)
    }
}
#[doc = "Master Done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MstDone {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<MstDone> for bool {
    #[inline(always)]
    fn from(variant: MstDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST_DONE` reader - Master Done interrupt enable."]
pub type MstDoneR = crate::BitReader<MstDone>;
impl MstDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MstDone {
        match self.bits {
            false => MstDone::Dis,
            true => MstDone::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MstDone::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MstDone::En
    }
}
#[doc = "Field `MST_DONE` writer - Master Done interrupt enable."]
pub type MstDoneW<'a, REG> = crate::BitWriter<'a, REG, MstDone>;
impl<'a, REG> MstDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(MstDone::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(MstDone::En)
    }
}
#[doc = "Transmit FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOv {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<TxOv> for bool {
    #[inline(always)]
    fn from(variant: TxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OV` reader - Transmit FIFO Overrun interrupt enable."]
pub type TxOvR = crate::BitReader<TxOv>;
impl TxOvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOv {
        match self.bits {
            false => TxOv::Dis,
            true => TxOv::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxOv::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxOv::En
    }
}
#[doc = "Field `TX_OV` writer - Transmit FIFO Overrun interrupt enable."]
pub type TxOvW<'a, REG> = crate::BitWriter<'a, REG, TxOv>;
impl<'a, REG> TxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxOv::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxOv::En)
    }
}
#[doc = "Transmit FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxUn {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<TxUn> for bool {
    #[inline(always)]
    fn from(variant: TxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UN` reader - Transmit FIFO Underrun interrupt enable."]
pub type TxUnR = crate::BitReader<TxUn>;
impl TxUnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxUn {
        match self.bits {
            false => TxUn::Dis,
            true => TxUn::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxUn::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxUn::En
    }
}
#[doc = "Field `TX_UN` writer - Transmit FIFO Underrun interrupt enable."]
pub type TxUnW<'a, REG> = crate::BitWriter<'a, REG, TxUn>;
impl<'a, REG> TxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::En)
    }
}
#[doc = "Receive FIFO Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOv {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<RxOv> for bool {
    #[inline(always)]
    fn from(variant: RxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OV` reader - Receive FIFO Overrun interrupt enable."]
pub type RxOvR = crate::BitReader<RxOv>;
impl RxOvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxOv {
        match self.bits {
            false => RxOv::Dis,
            true => RxOv::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxOv::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxOv::En
    }
}
#[doc = "Field `RX_OV` writer - Receive FIFO Overrun interrupt enable."]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG, RxOv>;
impl<'a, REG> RxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::En)
    }
}
#[doc = "Receive FIFO Underrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxUn {
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
    #[doc = "1: Interrupt is enabled."]
    En = 1,
}
impl From<RxUn> for bool {
    #[inline(always)]
    fn from(variant: RxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_UN` reader - Receive FIFO Underrun interrupt enable."]
pub type RxUnR = crate::BitReader<RxUn>;
impl RxUnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxUn {
        match self.bits {
            false => RxUn::Dis,
            true => RxUn::En,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxUn::Dis
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxUn::En
    }
}
#[doc = "Field `RX_UN` writer - Receive FIFO Underrun interrupt enable."]
pub type RxUnW<'a, REG> = crate::BitWriter<'a, REG, RxUn>;
impl<'a, REG> RxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxUn::Dis)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxUn::En)
    }
}
impl R {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&self) -> SsaR {
        SsaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn mst_done(&self) -> MstDoneR {
        MstDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ov(&self) -> TxOvR {
        TxOvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_un(&self) -> TxUnR {
        TxUnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_un(&self) -> RxUnR {
        RxUnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Threshold interrupt enable."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<IntenSpec> {
        TxThdW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Empty interrupt enable."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TxEmW<IntenSpec> {
        TxEmW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Threshold Crossed interrupt enable."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<IntenSpec> {
        RxThdW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO FULL interrupt enable."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<IntenSpec> {
        RxFullW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Select Asserted interrupt enable."]
    #[inline(always)]
    pub fn ssa(&mut self) -> SsaW<IntenSpec> {
        SsaW::new(self, 4)
    }
    #[doc = "Bit 5 - Slave Select Deasserted interrupt enable."]
    #[inline(always)]
    pub fn ssd(&mut self) -> SsdW<IntenSpec> {
        SsdW::new(self, 5)
    }
    #[doc = "Bit 8 - Multi-Master Mode Fault interrupt enable."]
    #[inline(always)]
    pub fn fault(&mut self) -> FaultW<IntenSpec> {
        FaultW::new(self, 8)
    }
    #[doc = "Bit 9 - Slave Abort Detected interrupt enable."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<IntenSpec> {
        AbortW::new(self, 9)
    }
    #[doc = "Bit 11 - Master Done interrupt enable."]
    #[inline(always)]
    pub fn mst_done(&mut self) -> MstDoneW<IntenSpec> {
        MstDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn tx_ov(&mut self) -> TxOvW<IntenSpec> {
        TxOvW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn tx_un(&mut self) -> TxUnW<IntenSpec> {
        TxUnW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive FIFO Overrun interrupt enable."]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<IntenSpec> {
        RxOvW::new(self, 14)
    }
    #[doc = "Bit 15 - Receive FIFO Underrun interrupt enable."]
    #[inline(always)]
    pub fn rx_un(&mut self) -> RxUnW<IntenSpec> {
        RxUnW::new(self, 15)
    }
}
#[doc = "Register for enabling interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
