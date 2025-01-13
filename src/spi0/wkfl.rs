#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WkflSpec>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WkflSpec>;
#[doc = "Wake on TX FIFO Threshold Crossed.\n\nValue on reset: 0"]
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
#[doc = "Field `TX_THD` reader - Wake on TX FIFO Threshold Crossed."]
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
#[doc = "Field `TX_THD` writer - Wake on TX FIFO Threshold Crossed."]
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
#[doc = "Wake on TX FIFO Empty.\n\nValue on reset: 0"]
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
#[doc = "Field `TX_EM` reader - Wake on TX FIFO Empty."]
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
#[doc = "Field `TX_EM` writer - Wake on TX FIFO Empty."]
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
#[doc = "Wake on RX FIFO Threshold Crossed.\n\nValue on reset: 0"]
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
#[doc = "Field `RX_THD` reader - Wake on RX FIFO Threshold Crossed."]
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
#[doc = "Field `RX_THD` writer - Wake on RX FIFO Threshold Crossed."]
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
#[doc = "Wake on RX FIFO Full.\n\nValue on reset: 0"]
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
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full."]
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
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full."]
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
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<WkflSpec> {
        TxThdW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TxEmW<WkflSpec> {
        TxEmW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<WkflSpec> {
        RxThdW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<WkflSpec> {
        RxFullW::new(self, 3)
    }
}
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkflSpec;
impl crate::RegisterSpec for WkflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WkflSpec {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WkflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WkflSpec {
    const RESET_VALUE: u32 = 0;
}
