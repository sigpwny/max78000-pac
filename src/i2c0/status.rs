#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Bus Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: I2C Bus Idle."]
    Idle = 0,
    #[doc = "1: I2C Bus Busy."]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Bus Status."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Busy,
        }
    }
    #[doc = "I2C Bus Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "I2C Bus Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "RX empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxEm {
    #[doc = "0: Not Empty."]
    NotEmpty = 0,
    #[doc = "1: Empty."]
    Empty = 1,
}
impl From<RxEm> for bool {
    #[inline(always)]
    fn from(variant: RxEm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EM` reader - RX empty."]
pub type RxEmR = crate::BitReader<RxEm>;
impl RxEmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxEm {
        match self.bits {
            false => RxEm::NotEmpty,
            true => RxEm::Empty,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RxEm::NotEmpty
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RxEm::Empty
    }
}
#[doc = "RX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFull {
    #[doc = "0: Not Full."]
    NotFull = 0,
    #[doc = "1: Full."]
    Full = 1,
}
impl From<RxFull> for bool {
    #[inline(always)]
    fn from(variant: RxFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - RX Full."]
pub type RxFullR = crate::BitReader<RxFull>;
impl RxFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFull {
        match self.bits {
            false => RxFull::NotFull,
            true => RxFull::Full,
        }
    }
    #[doc = "Not Full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RxFull::NotFull
    }
    #[doc = "Full."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RxFull::Full
    }
}
#[doc = "TX Empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEm {
    #[doc = "0: Not Empty."]
    NotEmpty = 0,
    #[doc = "1: Empty."]
    Empty = 1,
}
impl From<TxEm> for bool {
    #[inline(always)]
    fn from(variant: TxEm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EM` reader - TX Empty."]
pub type TxEmR = crate::BitReader<TxEm>;
impl TxEmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEm {
        match self.bits {
            false => TxEm::NotEmpty,
            true => TxEm::Empty,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TxEm::NotEmpty
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TxEm::Empty
    }
}
#[doc = "Field `TX_EM` writer - TX Empty."]
pub type TxEmW<'a, REG> = crate::BitWriter<'a, REG, TxEm>;
impl<'a, REG> TxEmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::NotEmpty)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::Empty)
    }
}
#[doc = "TX Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFull {
    #[doc = "0: Not Empty."]
    NotEmpty = 0,
    #[doc = "1: Empty."]
    Empty = 1,
}
impl From<TxFull> for bool {
    #[inline(always)]
    fn from(variant: TxFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FULL` reader - TX Full."]
pub type TxFullR = crate::BitReader<TxFull>;
impl TxFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFull {
        match self.bits {
            false => TxFull::NotEmpty,
            true => TxFull::Empty,
        }
    }
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TxFull::NotEmpty
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TxFull::Empty
    }
}
#[doc = "Field `TX_FULL` writer - TX Full."]
pub type TxFullW<'a, REG> = crate::BitWriter<'a, REG, TxFull>;
impl<'a, REG> TxFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(TxFull::NotEmpty)
    }
    #[doc = "Empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TxFull::Empty)
    }
}
#[doc = "Clock Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MstBusy {
    #[doc = "0: Device not actively driving SCL clock cycles."]
    NotActivelyDrivingSclClock = 0,
    #[doc = "1: Device operating as master and actively driving SCL clock cycles."]
    ActivelyDrivingSclClock = 1,
}
impl From<MstBusy> for bool {
    #[inline(always)]
    fn from(variant: MstBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST_BUSY` reader - Clock Mode."]
pub type MstBusyR = crate::BitReader<MstBusy>;
impl MstBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MstBusy {
        match self.bits {
            false => MstBusy::NotActivelyDrivingSclClock,
            true => MstBusy::ActivelyDrivingSclClock,
        }
    }
    #[doc = "Device not actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_not_actively_driving_scl_clock(&self) -> bool {
        *self == MstBusy::NotActivelyDrivingSclClock
    }
    #[doc = "Device operating as master and actively driving SCL clock cycles."]
    #[inline(always)]
    pub fn is_actively_driving_scl_clock(&self) -> bool {
        *self == MstBusy::ActivelyDrivingSclClock
    }
}
impl R {
    #[doc = "Bit 0 - Bus Status."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX empty."]
    #[inline(always)]
    pub fn rx_em(&self) -> RxEmR {
        RxEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn tx_full(&self) -> TxFullR {
        TxFullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Mode."]
    #[inline(always)]
    pub fn mst_busy(&self) -> MstBusyR {
        MstBusyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TX Empty."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TxEmW<StatusSpec> {
        TxEmW::new(self, 3)
    }
    #[doc = "Bit 4 - TX Full."]
    #[inline(always)]
    pub fn tx_full(&mut self) -> TxFullW<StatusSpec> {
        TxFullW::new(self, 4)
    }
}
#[doc = "Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
