#[doc = "Register `INTEN1` reader"]
pub type R = crate::R<Inten1Spec>;
#[doc = "Register `INTEN1` writer"]
pub type W = crate::W<Inten1Spec>;
#[doc = "Receiver Overflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOv {
    #[doc = "0: No Interrupt is Pending."]
    Dis = 0,
    #[doc = "1: An interrupt is pending."]
    En = 1,
}
impl From<RxOv> for bool {
    #[inline(always)]
    fn from(variant: RxOv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OV` reader - Receiver Overflow Interrupt Enable."]
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
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxOv::Dis
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxOv::En
    }
}
#[doc = "Field `RX_OV` writer - Receiver Overflow Interrupt Enable."]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG, RxOv>;
impl<'a, REG> RxOvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::Dis)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxOv::En)
    }
}
#[doc = "Transmit Underflow Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxUn {
    #[doc = "0: No Interrupt is Pending."]
    Dis = 0,
    #[doc = "1: An interrupt is pending."]
    En = 1,
}
impl From<TxUn> for bool {
    #[inline(always)]
    fn from(variant: TxUn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UN` reader - Transmit Underflow Interrupt Enable."]
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
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxUn::Dis
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxUn::En
    }
}
#[doc = "Field `TX_UN` writer - Transmit Underflow Interrupt Enable."]
pub type TxUnW<'a, REG> = crate::BitWriter<'a, REG, TxUn>;
impl<'a, REG> TxUnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::Dis)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxUn::En)
    }
}
#[doc = "Field `START` reader - START Condition Interrupt Enable."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START Condition Interrupt Enable."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn tx_un(&self) -> TxUnR {
        TxUnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - START Condition Interrupt Enable."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt Enable."]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<Inten1Spec> {
        RxOvW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt Enable."]
    #[inline(always)]
    pub fn tx_un(&mut self) -> TxUnW<Inten1Spec> {
        TxUnW::new(self, 1)
    }
    #[doc = "Bit 2 - START Condition Interrupt Enable."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Inten1Spec> {
        StartW::new(self, 2)
    }
}
#[doc = "Interrupt Staus Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inten1Spec;
impl crate::RegisterSpec for Inten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten1::R`](R) reader structure"]
impl crate::Readable for Inten1Spec {}
#[doc = "`write(|w| ..)` method takes [`inten1::W`](W) writer structure"]
impl crate::Writable for Inten1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN1 to value 0"]
impl crate::Resettable for Inten1Spec {
    const RESET_VALUE: u32 = 0;
}
