#[doc = "Register `EVENTEN` reader"]
pub type R = crate::R<EventenSpec>;
#[doc = "Register `EVENTEN` writer"]
pub type W = crate::W<EventenSpec>;
#[doc = "Field `DMA` reader - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` reader - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TxR = crate::BitReader;
#[doc = "Field `TX` writer - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<EventenSpec> {
        DmaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO1.8 will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rx(&mut self) -> RxW<EventenSpec> {
        RxW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO1.9."]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<EventenSpec> {
        TxW::new(self, 2)
    }
}
#[doc = "Event Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`eventen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventenSpec;
impl crate::RegisterSpec for EventenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eventen::R`](R) reader structure"]
impl crate::Readable for EventenSpec {}
#[doc = "`write(|w| ..)` method takes [`eventen::W`](W) writer structure"]
impl crate::Writable for EventenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTEN to value 0"]
impl crate::Resettable for EventenSpec {
    const RESET_VALUE: u32 = 0;
}
