#[doc = "Register `INTFL1` reader"]
pub type R = crate::R<Intfl1Spec>;
#[doc = "Register `INTFL1` writer"]
pub type W = crate::W<Intfl1Spec>;
#[doc = "Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOv {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<RxOv> for bool {
    #[inline(always)]
    fn from(variant: RxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OV` reader - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
pub type RxOvR = crate::BitReader<RxOv>;
impl RxOvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxOv {
        match self.bits {
            false => RxOv::Inactive,
            true => RxOv::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RxOv::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RxOv::Pending
    }
}
#[doc = "Field `RX_OV` writer - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG, RxOv>;
impl<'a, REG> RxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::Pending)
    }
}
#[doc = "Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxUn {
    #[doc = "0: No Interrupt is Pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<TxUn> for bool {
    #[inline(always)]
    fn from(variant: TxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UN` reader - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
pub type TxUnR = crate::BitReader<TxUn>;
impl TxUnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxUn {
        match self.bits {
            false => TxUn::Inactive,
            true => TxUn::Pending,
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TxUn::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TxUn::Pending
    }
}
#[doc = "Field `TX_UN` writer - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
pub type TxUnW<'a, REG> = crate::BitWriter<'a, REG, TxUn>;
impl<'a, REG> TxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::Pending)
    }
}
#[doc = "Field `START` reader - START Condition Status Flag."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START Condition Status Flag."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    pub fn tx_un(&self) -> TxUnR {
        TxUnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition Status Flag."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<Intfl1Spec> {
        RxOvW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    pub fn tx_un(&mut self) -> TxUnW<Intfl1Spec> {
        TxUnW::new(self, 1)
    }
    #[doc = "Bit 2 - START Condition Status Flag."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Intfl1Spec> {
        StartW::new(self, 2)
    }
}
#[doc = "Interrupt Status Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intfl1Spec;
impl crate::RegisterSpec for Intfl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl1::R`](R) reader structure"]
impl crate::Readable for Intfl1Spec {}
#[doc = "`write(|w| ..)` method takes [`intfl1::W`](W) writer structure"]
impl crate::Writable for Intfl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL1 to value 0"]
impl crate::Resettable for Intfl1Spec {
    const RESET_VALUE: u32 = 0;
}
