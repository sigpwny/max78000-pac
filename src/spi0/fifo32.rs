#[doc = "Register `FIFO32` reader"]
pub type R = crate::R<Fifo32Spec>;
#[doc = "Register `FIFO32` writer"]
pub type W = crate::W<Fifo32Spec>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Fifo32Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo32Spec;
impl crate::RegisterSpec for Fifo32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo32::R`](R) reader structure"]
impl crate::Readable for Fifo32Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo32::W`](W) writer structure"]
impl crate::Writable for Fifo32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO32 to value 0"]
impl crate::Resettable for Fifo32Spec {
    const RESET_VALUE: u32 = 0;
}
