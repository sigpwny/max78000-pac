#[doc = "Register `DATAIN8` reader"]
pub type R = crate::R<Datain8Spec>;
#[doc = "Register `DATAIN8` writer"]
pub type W = crate::W<Datain8Spec>;
#[doc = "Field `DATA` reader - CRC Data"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Datain8Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Datain8Spec;
impl crate::RegisterSpec for Datain8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`datain8::R`](R) reader structure"]
impl crate::Readable for Datain8Spec {}
#[doc = "`write(|w| ..)` method takes [`datain8::W`](W) writer structure"]
impl crate::Writable for Datain8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DATAIN8 to value 0"]
impl crate::Resettable for Datain8Spec {
    const RESET_VALUE: u8 = 0;
}
