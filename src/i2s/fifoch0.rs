#[doc = "Register `FIFOCH0` reader"]
pub type R = crate::R<Fifoch0Spec>;
#[doc = "Register `FIFOCH0` writer"]
pub type W = crate::W<Fifoch0Spec>;
#[doc = "Field `DATA` reader - Load/unload location for TX and RX FIFO buffers."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Load/unload location for TX and RX FIFO buffers."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load/unload location for TX and RX FIFO buffers."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Fifoch0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "I2S Fifo.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifoch0Spec;
impl crate::RegisterSpec for Fifoch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoch0::R`](R) reader structure"]
impl crate::Readable for Fifoch0Spec {}
#[doc = "`write(|w| ..)` method takes [`fifoch0::W`](W) writer structure"]
impl crate::Writable for Fifoch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOCH0 to value 0"]
impl crate::Resettable for Fifoch0Spec {
    const RESET_VALUE: u32 = 0;
}
