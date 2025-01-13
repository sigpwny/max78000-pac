#[doc = "Register `FIFOLEN` reader"]
pub type R = crate::R<FifolenSpec>;
#[doc = "Register `FIFOLEN` writer"]
pub type W = crate::W<FifolenSpec>;
#[doc = "Field `RX_DEPTH` reader - Receive FIFO Length."]
pub type RxDepthR = crate::FieldReader;
#[doc = "Field `TX_DEPTH` reader - Transmit FIFO Length."]
pub type TxDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Length."]
    #[inline(always)]
    pub fn rx_depth(&self) -> RxDepthR {
        RxDepthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Length."]
    #[inline(always)]
    pub fn tx_depth(&self) -> TxDepthR {
        TxDepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "FIFO Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolenSpec;
impl crate::RegisterSpec for FifolenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolen::R`](R) reader structure"]
impl crate::Readable for FifolenSpec {}
#[doc = "`write(|w| ..)` method takes [`fifolen::W`](W) writer structure"]
impl crate::Writable for FifolenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOLEN to value 0"]
impl crate::Resettable for FifolenSpec {
    const RESET_VALUE: u32 = 0;
}
