#[doc = "Register `FIFO8[%s]` reader"]
pub type R = crate::R<Fifo8Spec>;
#[doc = "Register `FIFO8[%s]` writer"]
pub type W = crate::W<Fifo8Spec>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Fifo8Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo8Spec;
impl crate::RegisterSpec for Fifo8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fifo8::R`](R) reader structure"]
impl crate::Readable for Fifo8Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo8::W`](W) writer structure"]
impl crate::Writable for Fifo8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FIFO8[%s]
to value 0"]
impl crate::Resettable for Fifo8Spec {
    const RESET_VALUE: u8 = 0;
}
