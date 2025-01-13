#[doc = "Register `RXCTRL0` reader"]
pub type R = crate::R<Rxctrl0Spec>;
#[doc = "Register `RXCTRL0` writer"]
pub type W = crate::W<Rxctrl0Spec>;
#[doc = "Do Not Respond.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dnr {
    #[doc = "0: Always respond to address match."]
    Respond = 0,
    #[doc = "1: Do not respond to address match when RX_FIFO is not empty."]
    NotRespondRxFifoEmpty = 1,
}
impl From<Dnr> for bool {
    #[inline(always)]
    fn from(variant: Dnr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNR` reader - Do Not Respond."]
pub type DnrR = crate::BitReader<Dnr>;
impl DnrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnr {
        match self.bits {
            false => Dnr::Respond,
            true => Dnr::NotRespondRxFifoEmpty,
        }
    }
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == Dnr::Respond
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn is_not_respond_rx_fifo_empty(&self) -> bool {
        *self == Dnr::NotRespondRxFifoEmpty
    }
}
#[doc = "Field `DNR` writer - Do Not Respond."]
pub type DnrW<'a, REG> = crate::BitWriter<'a, REG, Dnr>;
impl<'a, REG> DnrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always respond to address match."]
    #[inline(always)]
    pub fn respond(self) -> &'a mut crate::W<REG> {
        self.variant(Dnr::Respond)
    }
    #[doc = "Do not respond to address match when RX_FIFO is not empty."]
    #[inline(always)]
    pub fn not_respond_rx_fifo_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Dnr::NotRespondRxFifoEmpty)
    }
}
#[doc = "Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: FIFO not flushed."]
    NotFlushed = 0,
    #[doc = "1: Flush RX_FIFO."]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` reader - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
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
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Flush::Flush
    }
}
#[doc = "Field `FLUSH` writer - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
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
    #[doc = "Flush RX_FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
#[doc = "Field `THD_LVL` reader - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub type ThdLvlR = crate::FieldReader;
#[doc = "Field `THD_LVL` writer - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
pub type ThdLvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&self) -> DnrR {
        DnrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_lvl(&self) -> ThdLvlR {
        ThdLvlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Do Not Respond."]
    #[inline(always)]
    pub fn dnr(&mut self) -> DnrW<Rxctrl0Spec> {
        DnrW::new(self, 0)
    }
    #[doc = "Bit 7 - Receive FIFO Flush. This bit is automatically cleared to 0 after the operation. Setting this bit to 1 will affect RX_FIFO status."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<Rxctrl0Spec> {
        FlushW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Receive FIFO Threshold. These bits define the RX_FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thd_lvl(&mut self) -> ThdLvlW<Rxctrl0Spec> {
        ThdLvlW::new(self, 8)
    }
}
#[doc = "Receive Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxctrl0Spec;
impl crate::RegisterSpec for Rxctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl0::R`](R) reader structure"]
impl crate::Readable for Rxctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rxctrl0::W`](W) writer structure"]
impl crate::Writable for Rxctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTRL0 to value 0"]
impl crate::Resettable for Rxctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
