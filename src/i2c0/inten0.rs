#[doc = "Register `INTEN0` reader"]
pub type R = crate::R<Inten0Spec>;
#[doc = "Register `INTEN0` writer"]
pub type W = crate::W<Inten0Spec>;
#[doc = "Transfer Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled when DONE = 1."]
    En = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Transfer Done Interrupt Enable."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Dis,
            true => Done::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Done::Dis
    }
    #[doc = "Interrupt enabled when DONE = 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Done::En
    }
}
#[doc = "Field `DONE` writer - Transfer Done Interrupt Enable."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Dis)
    }
    #[doc = "Interrupt enabled when DONE = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Done::En)
    }
}
#[doc = "Description not available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irxm {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled when RX_MODE = 1."]
    En = 1,
}
impl From<Irxm> for bool {
    #[inline(always)]
    fn from(variant: Irxm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRXM` reader - Description not available."]
pub type IrxmR = crate::BitReader<Irxm>;
impl IrxmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irxm {
        match self.bits {
            false => Irxm::Dis,
            true => Irxm::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Irxm::Dis
    }
    #[doc = "Interrupt enabled when RX_MODE = 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Irxm::En
    }
}
#[doc = "Field `IRXM` writer - Description not available."]
pub type IrxmW<'a, REG> = crate::BitWriter<'a, REG, Irxm>;
impl<'a, REG> IrxmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Irxm::Dis)
    }
    #[doc = "Interrupt enabled when RX_MODE = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Irxm::En)
    }
}
#[doc = "Slave mode general call address match received input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcAddrMatch {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled when GEN_CTRL_ADDR = 1."]
    En = 1,
}
impl From<GcAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: GcAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC_ADDR_MATCH` reader - Slave mode general call address match received input enable."]
pub type GcAddrMatchR = crate::BitReader<GcAddrMatch>;
impl GcAddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GcAddrMatch {
        match self.bits {
            false => GcAddrMatch::Dis,
            true => GcAddrMatch::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GcAddrMatch::Dis
    }
    #[doc = "Interrupt enabled when GEN_CTRL_ADDR = 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GcAddrMatch::En
    }
}
#[doc = "Field `GC_ADDR_MATCH` writer - Slave mode general call address match received input enable."]
pub type GcAddrMatchW<'a, REG> = crate::BitWriter<'a, REG, GcAddrMatch>;
impl<'a, REG> GcAddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrMatch::Dis)
    }
    #[doc = "Interrupt enabled when GEN_CTRL_ADDR = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrMatch::En)
    }
}
#[doc = "Slave mode incoming address match interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrMatch {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled when ADDR_MATCH = 1."]
    En = 1,
}
impl From<AddrMatch> for bool {
    #[inline(always)]
    fn from(variant: AddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_MATCH` reader - Slave mode incoming address match interrupt."]
pub type AddrMatchR = crate::BitReader<AddrMatch>;
impl AddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrMatch {
        match self.bits {
            false => AddrMatch::Dis,
            true => AddrMatch::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AddrMatch::Dis
    }
    #[doc = "Interrupt enabled when ADDR_MATCH = 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AddrMatch::En
    }
}
#[doc = "Field `ADDR_MATCH` writer - Slave mode incoming address match interrupt."]
pub type AddrMatchW<'a, REG> = crate::BitWriter<'a, REG, AddrMatch>;
impl<'a, REG> AddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::Dis)
    }
    #[doc = "Interrupt enabled when ADDR_MATCH = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AddrMatch::En)
    }
}
#[doc = "RX FIFO Above Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxThd {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<RxThd> for bool {
    #[inline(always)]
    fn from(variant: RxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_THD` reader - RX FIFO Above Treshold Level Interrupt Enable."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxThd::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxThd::En
    }
}
#[doc = "Field `RX_THD` writer - RX FIFO Above Treshold Level Interrupt Enable."]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG, RxThd>;
impl<'a, REG> RxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::En)
    }
}
#[doc = "TX FIFO Below Treshold Level Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxThd {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<TxThd> for bool {
    #[inline(always)]
    fn from(variant: TxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_THD` reader - TX FIFO Below Treshold Level Interrupt Enable."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxThd::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxThd::En
    }
}
#[doc = "Field `TX_THD` writer - TX FIFO Below Treshold Level Interrupt Enable."]
pub type TxThdW<'a, REG> = crate::BitWriter<'a, REG, TxThd>;
impl<'a, REG> TxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::En)
    }
}
#[doc = "Stop Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled when STOP = 1."]
    En = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop Interrupt Enable"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::Dis,
            true => Stop::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Stop::Dis
    }
    #[doc = "Interrupt enabled when STOP = 1."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Stop::En
    }
}
#[doc = "Field `STOP` writer - Stop Interrupt Enable"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Dis)
    }
    #[doc = "Interrupt enabled when STOP = 1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::En)
    }
}
#[doc = "Received Address ACK from Slave Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrAck {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<AddrAck> for bool {
    #[inline(always)]
    fn from(variant: AddrAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_ACK` reader - Received Address ACK from Slave Interrupt."]
pub type AddrAckR = crate::BitReader<AddrAck>;
impl AddrAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrAck {
        match self.bits {
            false => AddrAck::Dis,
            true => AddrAck::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AddrAck::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AddrAck::En
    }
}
#[doc = "Field `ADDR_ACK` writer - Received Address ACK from Slave Interrupt."]
pub type AddrAckW<'a, REG> = crate::BitWriter<'a, REG, AddrAck>;
impl<'a, REG> AddrAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AddrAck::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AddrAck::En)
    }
}
#[doc = "Master Mode Arbitration Lost Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArbErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<ArbErr> for bool {
    #[inline(always)]
    fn from(variant: ArbErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB_ERR` reader - Master Mode Arbitration Lost Interrupt."]
pub type ArbErrR = crate::BitReader<ArbErr>;
impl ArbErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArbErr {
        match self.bits {
            false => ArbErr::Dis,
            true => ArbErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ArbErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ArbErr::En
    }
}
#[doc = "Field `ARB_ERR` writer - Master Mode Arbitration Lost Interrupt."]
pub type ArbErrW<'a, REG> = crate::BitWriter<'a, REG, ArbErr>;
impl<'a, REG> ArbErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ArbErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ArbErr::En)
    }
}
#[doc = "Timeout Error Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<ToErr> for bool {
    #[inline(always)]
    fn from(variant: ToErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TO_ERR` reader - Timeout Error Interrupt Enable."]
pub type ToErrR = crate::BitReader<ToErr>;
impl ToErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ToErr {
        match self.bits {
            false => ToErr::Dis,
            true => ToErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ToErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ToErr::En
    }
}
#[doc = "Field `TO_ERR` writer - Timeout Error Interrupt Enable."]
pub type ToErrW<'a, REG> = crate::BitWriter<'a, REG, ToErr>;
impl<'a, REG> ToErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ToErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ToErr::En)
    }
}
#[doc = "Master Mode Address NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrNackErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<AddrNackErr> for bool {
    #[inline(always)]
    fn from(variant: AddrNackErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR_NACK_ERR` reader - Master Mode Address NACK Received Interrupt."]
pub type AddrNackErrR = crate::BitReader<AddrNackErr>;
impl AddrNackErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AddrNackErr {
        match self.bits {
            false => AddrNackErr::Dis,
            true => AddrNackErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AddrNackErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AddrNackErr::En
    }
}
#[doc = "Field `ADDR_NACK_ERR` writer - Master Mode Address NACK Received Interrupt."]
pub type AddrNackErrW<'a, REG> = crate::BitWriter<'a, REG, AddrNackErr>;
impl<'a, REG> AddrNackErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AddrNackErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AddrNackErr::En)
    }
}
#[doc = "Master Mode Data NACK Received Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<DataErr> for bool {
    #[inline(always)]
    fn from(variant: DataErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_ERR` reader - Master Mode Data NACK Received Interrupt."]
pub type DataErrR = crate::BitReader<DataErr>;
impl DataErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataErr {
        match self.bits {
            false => DataErr::Dis,
            true => DataErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DataErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DataErr::En
    }
}
#[doc = "Field `DATA_ERR` writer - Master Mode Data NACK Received Interrupt."]
pub type DataErrW<'a, REG> = crate::BitWriter<'a, REG, DataErr>;
impl<'a, REG> DataErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DataErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DataErr::En)
    }
}
#[doc = "Slave Mode Do Not Respond Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DnrErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<DnrErr> for bool {
    #[inline(always)]
    fn from(variant: DnrErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNR_ERR` reader - Slave Mode Do Not Respond Interrupt."]
pub type DnrErrR = crate::BitReader<DnrErr>;
impl DnrErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DnrErr {
        match self.bits {
            false => DnrErr::Dis,
            true => DnrErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DnrErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DnrErr::En
    }
}
#[doc = "Field `DNR_ERR` writer - Slave Mode Do Not Respond Interrupt."]
pub type DnrErrW<'a, REG> = crate::BitWriter<'a, REG, DnrErr>;
impl<'a, REG> DnrErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DnrErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DnrErr::En)
    }
}
#[doc = "Out of Sequence START condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<StartErr> for bool {
    #[inline(always)]
    fn from(variant: StartErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_ERR` reader - Out of Sequence START condition detected interrupt."]
pub type StartErrR = crate::BitReader<StartErr>;
impl StartErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StartErr {
        match self.bits {
            false => StartErr::Dis,
            true => StartErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == StartErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == StartErr::En
    }
}
#[doc = "Field `START_ERR` writer - Out of Sequence START condition detected interrupt."]
pub type StartErrW<'a, REG> = crate::BitWriter<'a, REG, StartErr>;
impl<'a, REG> StartErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(StartErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(StartErr::En)
    }
}
#[doc = "Out of Sequence STOP condition detected interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopErr {
    #[doc = "0: Interrupt disabled."]
    Dis = 0,
    #[doc = "1: Interrupt enabled."]
    En = 1,
}
impl From<StopErr> for bool {
    #[inline(always)]
    fn from(variant: StopErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_ERR` reader - Out of Sequence STOP condition detected interrupt."]
pub type StopErrR = crate::BitReader<StopErr>;
impl StopErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopErr {
        match self.bits {
            false => StopErr::Dis,
            true => StopErr::En,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == StopErr::Dis
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == StopErr::En
    }
}
#[doc = "Field `STOP_ERR` writer - Out of Sequence STOP condition detected interrupt."]
pub type StopErrW<'a, REG> = crate::BitWriter<'a, REG, StopErr>;
impl<'a, REG> StopErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(StopErr::Dis)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(StopErr::En)
    }
}
#[doc = "Field `TX_LOCKOUT` reader - TX FIFO Locked Out Interrupt."]
pub type TxLockoutR = crate::BitReader;
#[doc = "Field `TX_LOCKOUT` writer - TX FIFO Locked Out Interrupt."]
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
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    pub fn irxm(&self) -> IrxmR {
        IrxmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    pub fn gc_addr_match(&self) -> GcAddrMatchR {
        GcAddrMatchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    pub fn addr_match(&self) -> AddrMatchR {
        AddrMatchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&self) -> AddrAckR {
        AddrAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    pub fn arb_err(&self) -> ArbErrR {
        ArbErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    pub fn to_err(&self) -> ToErrR {
        ToErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&self) -> AddrNackErrR {
        AddrNackErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    pub fn data_err(&self) -> DataErrR {
        DataErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&self) -> DnrErrR {
        DnrErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    pub fn start_err(&self) -> StartErrR {
        StartErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    pub fn stop_err(&self) -> StopErrR {
        StopErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
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
    #[doc = "Bit 0 - Transfer Done Interrupt Enable."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<Inten0Spec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Description not available."]
    #[inline(always)]
    pub fn irxm(&mut self) -> IrxmW<Inten0Spec> {
        IrxmW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave mode general call address match received input enable."]
    #[inline(always)]
    pub fn gc_addr_match(&mut self) -> GcAddrMatchW<Inten0Spec> {
        GcAddrMatchW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave mode incoming address match interrupt."]
    #[inline(always)]
    pub fn addr_match(&mut self) -> AddrMatchW<Inten0Spec> {
        AddrMatchW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Above Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<Inten0Spec> {
        RxThdW::new(self, 4)
    }
    #[doc = "Bit 5 - TX FIFO Below Treshold Level Interrupt Enable."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<Inten0Spec> {
        TxThdW::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Inten0Spec> {
        StopW::new(self, 6)
    }
    #[doc = "Bit 7 - Received Address ACK from Slave Interrupt."]
    #[inline(always)]
    pub fn addr_ack(&mut self) -> AddrAckW<Inten0Spec> {
        AddrAckW::new(self, 7)
    }
    #[doc = "Bit 8 - Master Mode Arbitration Lost Interrupt."]
    #[inline(always)]
    pub fn arb_err(&mut self) -> ArbErrW<Inten0Spec> {
        ArbErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Timeout Error Interrupt Enable."]
    #[inline(always)]
    pub fn to_err(&mut self) -> ToErrW<Inten0Spec> {
        ToErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Master Mode Address NACK Received Interrupt."]
    #[inline(always)]
    pub fn addr_nack_err(&mut self) -> AddrNackErrW<Inten0Spec> {
        AddrNackErrW::new(self, 10)
    }
    #[doc = "Bit 11 - Master Mode Data NACK Received Interrupt."]
    #[inline(always)]
    pub fn data_err(&mut self) -> DataErrW<Inten0Spec> {
        DataErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Slave Mode Do Not Respond Interrupt."]
    #[inline(always)]
    pub fn dnr_err(&mut self) -> DnrErrW<Inten0Spec> {
        DnrErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Out of Sequence START condition detected interrupt."]
    #[inline(always)]
    pub fn start_err(&mut self) -> StartErrW<Inten0Spec> {
        StartErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Out of Sequence STOP condition detected interrupt."]
    #[inline(always)]
    pub fn stop_err(&mut self) -> StopErrW<Inten0Spec> {
        StopErrW::new(self, 14)
    }
    #[doc = "Bit 15 - TX FIFO Locked Out Interrupt."]
    #[inline(always)]
    pub fn tx_lockout(&mut self) -> TxLockoutW<Inten0Spec> {
        TxLockoutW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Multiple Address Match Interrupt"]
    #[inline(always)]
    pub fn mami(&mut self) -> MamiW<Inten0Spec> {
        MamiW::new(self, 16)
    }
    #[doc = "Bit 22 - Slave Read Address Match Interrupt"]
    #[inline(always)]
    pub fn rd_addr_match(&mut self) -> RdAddrMatchW<Inten0Spec> {
        RdAddrMatchW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave Write Address Match Interrupt"]
    #[inline(always)]
    pub fn wr_addr_match(&mut self) -> WrAddrMatchW<Inten0Spec> {
        WrAddrMatchW::new(self, 23)
    }
}
#[doc = "Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inten0Spec;
impl crate::RegisterSpec for Inten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten0::R`](R) reader structure"]
impl crate::Readable for Inten0Spec {}
#[doc = "`write(|w| ..)` method takes [`inten0::W`](W) writer structure"]
impl crate::Writable for Inten0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN0 to value 0"]
impl crate::Resettable for Inten0Spec {
    const RESET_VALUE: u32 = 0;
}
