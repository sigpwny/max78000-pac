#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WkenSpec>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WkenSpec>;
#[doc = "Wake on TX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxThd {
    #[doc = "0: Wakeup source disabled."]
    Dis = 0,
    #[doc = "1: Wakeup source enabled."]
    En = 1,
}
impl From<TxThd> for bool {
    #[inline(always)]
    fn from(variant: TxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_THD` reader - Wake on TX FIFO Threshold Crossed Enable."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxThd::Dis
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxThd::En
    }
}
#[doc = "Field `TX_THD` writer - Wake on TX FIFO Threshold Crossed Enable."]
pub type TxThdW<'a, REG> = crate::BitWriter<'a, REG, TxThd>;
impl<'a, REG> TxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::Dis)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxThd::En)
    }
}
#[doc = "Wake on TX FIFO Empty Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEm {
    #[doc = "0: Wakeup source disabled."]
    Dis = 0,
    #[doc = "1: Wakeup source enabled."]
    En = 1,
}
impl From<TxEm> for bool {
    #[inline(always)]
    fn from(variant: TxEm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EM` reader - Wake on TX FIFO Empty Enable."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxEm::Dis
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxEm::En
    }
}
#[doc = "Field `TX_EM` writer - Wake on TX FIFO Empty Enable."]
pub type TxEmW<'a, REG> = crate::BitWriter<'a, REG, TxEm>;
impl<'a, REG> TxEmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::Dis)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxEm::En)
    }
}
#[doc = "Wake on RX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxThd {
    #[doc = "0: Wakeup source disabled."]
    Dis = 0,
    #[doc = "1: Wakeup source enabled."]
    En = 1,
}
impl From<RxThd> for bool {
    #[inline(always)]
    fn from(variant: RxThd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_THD` reader - Wake on RX FIFO Threshold Crossed Enable."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxThd::Dis
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxThd::En
    }
}
#[doc = "Field `RX_THD` writer - Wake on RX FIFO Threshold Crossed Enable."]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG, RxThd>;
impl<'a, REG> RxThdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::Dis)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxThd::En)
    }
}
#[doc = "Wake on RX FIFO Full Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFull {
    #[doc = "0: Wakeup source disabled."]
    Dis = 0,
    #[doc = "1: Wakeup source enabled."]
    En = 1,
}
impl From<RxFull> for bool {
    #[inline(always)]
    fn from(variant: RxFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FULL` reader - Wake on RX FIFO Full Enable."]
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
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxFull::Dis
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxFull::En
    }
}
#[doc = "Field `RX_FULL` writer - Wake on RX FIFO Full Enable."]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG, RxFull>;
impl<'a, REG> RxFullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxFull::Dis)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxFull::En)
    }
}
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_thd(&self) -> TxThdR {
        TxThdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_em(&self) -> TxEmR {
        TxEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_thd(&mut self) -> TxThdW<WkenSpec> {
        TxThdW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_em(&mut self) -> TxEmW<WkenSpec> {
        TxEmW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<WkenSpec> {
        RxThdW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<WkenSpec> {
        RxFullW::new(self, 3)
    }
}
#[doc = "Register for wake up enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkenSpec;
impl crate::RegisterSpec for WkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WkenSpec {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WkenSpec {
    const RESET_VALUE: u32 = 0;
}
