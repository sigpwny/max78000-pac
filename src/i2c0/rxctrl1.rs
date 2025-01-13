#[doc = "Register `RXCTRL1` reader"]
pub type R = crate::R<Rxctrl1Spec>;
#[doc = "Register `RXCTRL1` writer"]
pub type W = crate::W<Rxctrl1Spec>;
#[doc = "Field `CNT` reader - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LVL` reader - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
pub type LvlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<Rxctrl1Spec> {
        CntW::new(self, 0)
    }
}
#[doc = "Receive Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxctrl1Spec;
impl crate::RegisterSpec for Rxctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrl1::R`](R) reader structure"]
impl crate::Readable for Rxctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rxctrl1::W`](W) writer structure"]
impl crate::Writable for Rxctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTRL1 to value 0"]
impl crate::Resettable for Rxctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
