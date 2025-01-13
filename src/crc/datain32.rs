#[doc = "Register `DATAIN32` reader"]
pub type R = crate::R<Datain32Spec>;
#[doc = "Register `DATAIN32` writer"]
pub type W = crate::W<Datain32Spec>;
#[doc = "Field `DATA` reader - CRC Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Datain32Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Datain32Spec;
impl crate::RegisterSpec for Datain32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datain32::R`](R) reader structure"]
impl crate::Readable for Datain32Spec {}
#[doc = "`write(|w| ..)` method takes [`datain32::W`](W) writer structure"]
impl crate::Writable for Datain32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAIN32 to value 0"]
impl crate::Resettable for Datain32Spec {
    const RESET_VALUE: u32 = 0;
}
