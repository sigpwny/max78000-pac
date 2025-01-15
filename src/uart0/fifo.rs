#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `DATA` reader - Load/unload location for TX and RX FIFO buffers."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Load/unload location for TX and RX FIFO buffers."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_PAR` reader - Parity error flag for next byte to be read from FIFO."]
pub type RxParR = crate::BitReader;
#[doc = "Field `RX_PAR` writer - Parity error flag for next byte to be read from FIFO."]
pub type RxParW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Parity error flag for next byte to be read from FIFO."]
    #[inline(always)]
    pub fn rx_par(&self) -> RxParR {
        RxParR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<FifoSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - Parity error flag for next byte to be read from FIFO."]
    #[inline(always)]
    pub fn rx_par(&mut self) -> RxParW<FifoSpec> {
        RxParW::new(self, 8)
    }
}
#[doc = "FIFO Read/Write register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FifoSpec {
    const RESET_VALUE: u32 = 0;
}
