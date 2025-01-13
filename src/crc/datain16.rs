#[doc = "Register `DATAIN16` reader"]
pub type R = crate::R<Datain16Spec>;
#[doc = "Register `DATAIN16` writer"]
pub type W = crate::W<Datain16Spec>;
#[doc = "Field `DATA` reader - CRC Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Datain16Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Datain16Spec;
impl crate::RegisterSpec for Datain16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`datain16::R`](R) reader structure"]
impl crate::Readable for Datain16Spec {}
#[doc = "`write(|w| ..)` method takes [`datain16::W`](W) writer structure"]
impl crate::Writable for Datain16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATAIN16 to value 0"]
impl crate::Resettable for Datain16Spec {
    const RESET_VALUE: u16 = 0;
}
