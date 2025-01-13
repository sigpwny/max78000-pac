#[doc = "Register `FIFO16[%s]` reader"]
pub type R = crate::R<Fifo16Spec>;
#[doc = "Register `FIFO16[%s]` writer"]
pub type W = crate::W<Fifo16Spec>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Fifo16Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo16Spec;
impl crate::RegisterSpec for Fifo16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fifo16::R`](R) reader structure"]
impl crate::Readable for Fifo16Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo16::W`](W) writer structure"]
impl crate::Writable for Fifo16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FIFO16[%s]
to value 0"]
impl crate::Resettable for Fifo16Spec {
    const RESET_VALUE: u16 = 0;
}
