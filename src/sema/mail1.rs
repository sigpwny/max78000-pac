#[doc = "Register `mail1` reader"]
pub type R = crate::R<Mail1Spec>;
#[doc = "Register `mail1` writer"]
pub type W = crate::W<Mail1Spec>;
#[doc = "Field `data` reader - "]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - "]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Mail1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Semaphore Mailbox 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mail1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mail1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mail1Spec;
impl crate::RegisterSpec for Mail1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mail1::R`](R) reader structure"]
impl crate::Readable for Mail1Spec {}
#[doc = "`write(|w| ..)` method takes [`mail1::W`](W) writer structure"]
impl crate::Writable for Mail1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mail1 to value 0"]
impl crate::Resettable for Mail1Spec {
    const RESET_VALUE: u32 = 0;
}
