#[doc = "Register `FIFO_DATA` reader"]
pub type R = crate::R<FifoDataSpec>;
#[doc = "Register `FIFO_DATA` writer"]
pub type W = crate::W<FifoDataSpec>;
#[doc = "Field `DATA` reader - Data from FIFO to be read by DMA."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data from FIFO to be read by DMA."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data from FIFO to be read by DMA."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data from FIFO to be read by DMA."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<FifoDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "FIFO DATA Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoDataSpec;
impl crate::RegisterSpec for FifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FifoDataSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_data::W`](W) writer structure"]
impl crate::Writable for FifoDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FifoDataSpec {
    const RESET_VALUE: u32 = 0;
}
