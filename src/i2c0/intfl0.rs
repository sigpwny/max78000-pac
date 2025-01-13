#[doc = "Register `INTFL0` reader"]
pub type R = crate::R<Intfl0Spec>;
#[doc = "Register `INTFL0` writer"]
pub type W = crate::W<Intfl0Spec>;
#[doc = "Transfer Done Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntFl0Done {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<IntFl0Done> for bool {
    #[inline(always)]
    fn from(variant: IntFl0Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Transfer Done Interrupt."]
pub type DoneR = crate::BitReader<IntFl0Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntFl0Done {
        match self.bits {
            false => IntFl0Done::Inactive,
            true => IntFl0Done::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IntFl0Done::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IntFl0Done::Pending
    }
}
#[doc = "Field `DONE` writer - Transfer Done Interrupt."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, IntFl0Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IntFl0Done::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntFl0Done::Pending)
    }
}
#[doc = "Interactive Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irxm {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<Irxm> for bool {
    #[inline(always)]
    fn from(variant: Irxm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRXM` reader - Interactive Receive Interrupt."]
pub type IrxmR = crate::BitReader<Irxm>;
impl IrxmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irxm {
        match self.bits {
            false => Irxm::Inactive,
            true => Irxm::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Irxm::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Irxm::Pending
    }
}
#[doc = "Field `IRXM` writer - Interactive Receive Interrupt."]
pub type IrxmW<'a, REG> = crate::BitWriter<'a, REG, Irxm>;
impl<'a, REG> IrxmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Irxm::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Irxm::Pending)
    }
}
#[doc = "Slave General Call Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcAddrMatch {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<GcAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: GcAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC_ADDR_MATCH` reader - Slave General Call Address Match Interrupt."]
pub type GcAddrMatchR = crate::BitReader<GcAddrMatch>;
impl GcAddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GcAddrMatch {
        match self.bits {
            false => GcAddrMatch::Inactive,
            true => GcAddrMatch::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == GcAddrMatch::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GcAddrMatch::Pending
    }
}
#[doc = "Field `GC_ADDR_MATCH` writer - Slave General Call Address Match Interrupt."]
pub type GcAddrMatchW<'a, REG> = crate::BitWriter<'a, REG, GcAddrMatch>;
impl<'a, REG> GcAddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrMatch::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrMatch::Pending)
    }
}
#[doc = "Slave Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMatch {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<AddrMatch> for bool {
    #[inline(always)]
    fn from(variant: AddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` reader - Slave Address Match Interrupt."]
pub type AddrMatchR = crate::BitReader<AddrMatch>;
impl AddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrMatch {
        match self.bits {
            false => AddrMatch::Inactive,
            true => AddrMatch::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == AddrMatch::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == AddrMatch::Pending
    }
}
#[doc = "Field `ADDR_MATCH` writer - Slave Address Match Interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG, AddrMatch>;
impl<'a, REG> AddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Pending)
    }
}
#[doc = "Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxThd {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    Pending = 1,
}
impl From<RxThd> for bool {
    #[inline(always)]
    fn from(variant: RxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_THD` reader - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
pub type RxThdR = crate::BitReader<RxThd>;
impl RxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxThd {
        match self.bits {
            false => RxThd::Inactive,
            true => RxThd::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RxThd::Inactive
    }
    #[doc = "An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RxThd::Pending
    }
}
#[doc = "Field `RX_THD` writer - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG, RxThd>;
impl<'a, REG> RxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Inactive)
    }
    #[doc = "An interrupt is pending. RX_FIFO equal or more bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Pending)
    }
}
#[doc = "Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxThd {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    Pending = 1,
}
impl From<TxThd> for bool {
    #[inline(always)]
    fn from(variant: TxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_THD` reader - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
pub type TxThdR = crate::BitReader<TxThd>;
impl TxThdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxThd {
        match self.bits {
            false => TxThd::Inactive,
            true => TxThd::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TxThd::Inactive
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TxThd::Pending
    }
}
#[doc = "Field `TX_THD` writer - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
pub type TxThdW<'a, REG> = crate::BitWriter<'a, REG, TxThd>;
impl<'a, REG> TxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Inactive)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Pending)
    }
}
#[doc = "STOP Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    Pending = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - STOP Interrupt."]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::Inactive,
            true => Stop::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Stop::Inactive
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Stop::Pending
    }
}
#[doc = "Field `STOP` writer - STOP Interrupt."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Inactive)
    }
    #[doc = "An interrupt is pending. TX_FIFO has equal or less bytes than the threshold."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Pending)
    }
}
#[doc = "Address Acknowledge Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrAck {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<AddrAck> for bool {
    #[inline(always)]
    fn from(variant: AddrAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_ACK` reader - Address Acknowledge Interrupt."]
pub type AddrAckR = crate::BitReader<AddrAck>;
impl AddrAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrAck {
        match self.bits {
            false => AddrAck::Inactive,
            true => AddrAck::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == AddrAck::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == AddrAck::Pending
    }
}
#[doc = "Field `ADDR_ACK` writer - Address Acknowledge Interrupt."]
pub type AddrAckW<'a, REG> = crate::BitWriter<'a, REG, AddrAck>;
impl<'a, REG> AddrAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(AddrAck::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(AddrAck::Pending)
    }
}
#[doc = "Arbritation error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArbErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<ArbErr> for bool {
    #[inline(always)]
    fn from(variant: ArbErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB_ERR` reader - Arbritation error Interrupt."]
pub type ArbErrR = crate::BitReader<ArbErr>;
impl ArbErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArbErr {
        match self.bits {
            false => ArbErr::Inactive,
            true => ArbErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ArbErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ArbErr::Pending
    }
}
#[doc = "Field `ARB_ERR` writer - Arbritation error Interrupt."]
pub type ArbErrW<'a, REG> = crate::BitWriter<'a, REG, ArbErr>;
impl<'a, REG> ArbErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ArbErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ArbErr::Pending)
    }
}
#[doc = "timeout Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<ToErr> for bool {
    #[inline(always)]
    fn from(variant: ToErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_ERR` reader - timeout Error Interrupt."]
pub type ToErrR = crate::BitReader<ToErr>;
impl ToErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToErr {
        match self.bits {
            false => ToErr::Inactive,
            true => ToErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ToErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ToErr::Pending
    }
}
#[doc = "Field `TO_ERR` writer - timeout Error Interrupt."]
pub type ToErrW<'a, REG> = crate::BitWriter<'a, REG, ToErr>;
impl<'a, REG> ToErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(ToErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ToErr::Pending)
    }
}
#[doc = "Address NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrNackErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<AddrNackErr> for bool {
    #[inline(always)]
    fn from(variant: AddrNackErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_NACK_ERR` reader - Address NACK Error Interrupt."]
pub type AddrNackErrR = crate::BitReader<AddrNackErr>;
impl AddrNackErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrNackErr {
        match self.bits {
            false => AddrNackErr::Inactive,
            true => AddrNackErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == AddrNackErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == AddrNackErr::Pending
    }
}
#[doc = "Field `ADDR_NACK_ERR` writer - Address NACK Error Interrupt."]
pub type AddrNackErrW<'a, REG> = crate::BitWriter<'a, REG, AddrNackErr>;
impl<'a, REG> AddrNackErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(AddrNackErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(AddrNackErr::Pending)
    }
}
#[doc = "Data NACK Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<DataErr> for bool {
    #[inline(always)]
    fn from(variant: DataErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_ERR` reader - Data NACK Error Interrupt."]
pub type DataErrR = crate::BitReader<DataErr>;
impl DataErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataErr {
        match self.bits {
            false => DataErr::Inactive,
            true => DataErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DataErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DataErr::Pending
    }
}
#[doc = "Field `DATA_ERR` writer - Data NACK Error Interrupt."]
pub type DataErrW<'a, REG> = crate::BitWriter<'a, REG, DataErr>;
impl<'a, REG> DataErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DataErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DataErr::Pending)
    }
}
#[doc = "Do Not Respond Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DnrErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<DnrErr> for bool {
    #[inline(always)]
    fn from(variant: DnrErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNR_ERR` reader - Do Not Respond Error Interrupt."]
pub type DnrErrR = crate::BitReader<DnrErr>;
impl DnrErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DnrErr {
        match self.bits {
            false => DnrErr::Inactive,
            true => DnrErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DnrErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DnrErr::Pending
    }
}
#[doc = "Field `DNR_ERR` writer - Do Not Respond Error Interrupt."]
pub type DnrErrW<'a, REG> = crate::BitWriter<'a, REG, DnrErr>;
impl<'a, REG> DnrErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DnrErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(DnrErr::Pending)
    }
}
#[doc = "Start Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<StartErr> for bool {
    #[inline(always)]
    fn from(variant: StartErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_ERR` reader - Start Error Interrupt."]
pub type StartErrR = crate::BitReader<StartErr>;
impl StartErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartErr {
        match self.bits {
            false => StartErr::Inactive,
            true => StartErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == StartErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == StartErr::Pending
    }
}
#[doc = "Field `START_ERR` writer - Start Error Interrupt."]
pub type StartErrW<'a, REG> = crate::BitWriter<'a, REG, StartErr>;
impl<'a, REG> StartErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(StartErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(StartErr::Pending)
    }
}
#[doc = "Stop Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopErr {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<StopErr> for bool {
    #[inline(always)]
    fn from(variant: StopErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_ERR` reader - Stop Error Interrupt."]
pub type StopErrR = crate::BitReader<StopErr>;
impl StopErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopErr {
        match self.bits {
            false => StopErr::Inactive,
            true => StopErr::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == StopErr::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == StopErr::Pending
    }
}
#[doc = "Field `STOP_ERR` writer - Stop Error Interrupt."]
pub type StopErrW<'a, REG> = crate::BitWriter<'a, REG, StopErr>;
impl<'a, REG> StopErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(StopErr::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(StopErr::Pending)
    }
}
#[doc = "Field `TX_LOCKOUT` reader - Transmit Lock Out Interrupt."]
pub type TxLockoutR = crate::BitReader;
#[doc = "Field `TX_LOCKOUT` writer - Transmit Lock Out Interrupt."]
pub type TxLockoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAMI` reader - Multiple Address Match Interrupt"]
pub type MamiR = crate::FieldReader;
#[doc = "Field `MAMI` writer - Multiple Address Match Interrupt"]
pub type MamiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RD_ADDR_MATCH` reader - Slave Read Address Match Interrupt"]
pub type RdAddrMatchR = crate::BitReader;
#[doc = "Field `RD_ADDR_MATCH` writer - Slave Read Address Match Interrupt"]
pub type RdAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ADDR_MATCH` reader - Slave Write Address Match Interrupt"]
pub type WrAddrMatchR = crate::BitReader;
#[doc = "Field `WR_ADDR_MATCH` writer - Slave Write Address Match Interrupt"]
pub type WrAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn irxm(&self) -> IrxmR {
        IrxmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gc_addr_match(&self) -> GcAddrMatchR {
        GcAddrMatchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&self) -> AddrAckR {
        AddrAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arb_err(&self) -> ArbErrR {
        ArbErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn to_err(&self) -> ToErrR {
        ToErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&self) -> AddrNackErrR {
        AddrNackErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn data_err(&self) -> DataErrR {
        DataErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&self) -> DnrErrR {
        DnrErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn start_err(&self) -> StartErrR {
        StartErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stop_err(&self) -> StopErrR {
        StopErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn tx_lockout(&self) -> TxLockoutR {
        TxLockoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    pub fn mami(&self) -> MamiR {
        MamiR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    pub fn rd_addr_match(&self) -> RdAddrMatchR {
        RdAddrMatchR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    pub fn wr_addr_match(&self) -> WrAddrMatchR {
        WrAddrMatchR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Done Interrupt."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<Intfl0Spec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Interactive Receive Interrupt."]
    #[inline(always)]
    pub fn irxm(&mut self) -> IrxmW<Intfl0Spec> {
        IrxmW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave General Call Address Match Interrupt."]
    #[inline(always)]
    pub fn gc_addr_match(&mut self) -> GcAddrMatchW<Intfl0Spec> {
        GcAddrMatchW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Address Match Interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<Intfl0Spec> {
        AddrMatchW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Threshold Interrupt. This bit is automaticcaly cleared when RX_FIFO is below the threshold level."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<Intfl0Spec> {
        RxThdW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Threshold Interrupt. This bit is automaticcaly cleared when TX_FIFO is above the threshold level."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<Intfl0Spec> {
        TxThdW::new(self, 5)
    }
    #[doc = "Bit 6 - STOP Interrupt."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Intfl0Spec> {
        StopW::new(self, 6)
    }
    #[doc = "Bit 7 - Address Acknowledge Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&mut self) -> AddrAckW<Intfl0Spec> {
        AddrAckW::new(self, 7)
    }
    #[doc = "Bit 8 - Arbritation error Interrupt."]
    #[inline(always)]
    pub fn arb_err(&mut self) -> ArbErrW<Intfl0Spec> {
        ArbErrW::new(self, 8)
    }
    #[doc = "Bit 9 - timeout Error Interrupt."]
    #[inline(always)]
    pub fn to_err(&mut self) -> ToErrW<Intfl0Spec> {
        ToErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Address NACK Error Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&mut self) -> AddrNackErrW<Intfl0Spec> {
        AddrNackErrW::new(self, 10)
    }
    #[doc = "Bit 11 - Data NACK Error Interrupt."]
    #[inline(always)]
    pub fn data_err(&mut self) -> DataErrW<Intfl0Spec> {
        DataErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Do Not Respond Error Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&mut self) -> DnrErrW<Intfl0Spec> {
        DnrErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Start Error Interrupt."]
    #[inline(always)]
    pub fn start_err(&mut self) -> StartErrW<Intfl0Spec> {
        StartErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop Error Interrupt."]
    #[inline(always)]
    pub fn stop_err(&mut self) -> StopErrW<Intfl0Spec> {
        StopErrW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Lock Out Interrupt."]
    #[inline(always)]
    pub fn tx_lockout(&mut self) -> TxLockoutW<Intfl0Spec> {
        TxLockoutW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    pub fn mami(&mut self) -> MamiW<Intfl0Spec> {
        MamiW::new(self, 16)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    pub fn rd_addr_match(&mut self) -> RdAddrMatchW<Intfl0Spec> {
        RdAddrMatchW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    pub fn wr_addr_match(&mut self) -> WrAddrMatchW<Intfl0Spec> {
        WrAddrMatchW::new(self, 23)
    }
}
#[doc = "Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intfl0Spec;
impl crate::RegisterSpec for Intfl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl0::R`](R) reader structure"]
impl crate::Readable for Intfl0Spec {}
#[doc = "`write(|w| ..)` method takes [`intfl0::W`](W) writer structure"]
impl crate::Writable for Intfl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL0 to value 0"]
impl crate::Resettable for Intfl0Spec {
    const RESET_VALUE: u32 = 0;
}
