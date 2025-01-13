#[doc = "Register `TXCTRL1` reader"]
pub type R = crate::R<Txctrl1Spec>;
#[doc = "Register `TXCTRL1` writer"]
pub type W = crate::W<Txctrl1Spec>;
#[doc = "Field `PRELOAD_RDY` reader - Transmit FIFO Preload Ready."]
pub type PreloadRdyR = crate::BitReader;
#[doc = "Field `PRELOAD_RDY` writer - Transmit FIFO Preload Ready."]
pub type PreloadRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVL` reader - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
pub type LvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn preload_rdy(&self) -> PreloadRdyR {
        PreloadRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn preload_rdy(&mut self) -> PreloadRdyW<Txctrl1Spec> {
        PreloadRdyW::new(self, 0)
    }
}
#[doc = "Transmit Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txctrl1Spec;
impl crate::RegisterSpec for Txctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl1::R`](R) reader structure"]
impl crate::Readable for Txctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`txctrl1::W`](W) writer structure"]
impl crate::Writable for Txctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTRL1 to value 0"]
impl crate::Resettable for Txctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
