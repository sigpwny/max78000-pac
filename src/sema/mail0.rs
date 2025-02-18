#[doc = "Register `mail0` reader"]
pub type R = crate::R<Mail0Spec>;
#[doc = "Register `mail0` writer"]
pub type W = crate::W<Mail0Spec>;
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
    pub fn data(&mut self) -> DataW<Mail0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Semaphore Mailbox 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mail0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mail0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mail0Spec;
impl crate::RegisterSpec for Mail0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mail0::R`](R) reader structure"]
impl crate::Readable for Mail0Spec {}
#[doc = "`write(|w| ..)` method takes [`mail0::W`](W) writer structure"]
impl crate::Writable for Mail0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mail0 to value 0"]
impl crate::Resettable for Mail0Spec {
    const RESET_VALUE: u32 = 0;
}
