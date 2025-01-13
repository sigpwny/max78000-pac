#[doc = "Register `ECCADDR` reader"]
pub type R = crate::R<EccaddrSpec>;
#[doc = "Register `ECCADDR` writer"]
pub type W = crate::W<EccaddrSpec>;
#[doc = "Field `ECCERRAD` reader - ECC Error Address."]
pub type EccerradR = crate::FieldReader<u32>;
#[doc = "Field `ECCERRAD` writer - ECC Error Address."]
pub type EccerradW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    pub fn eccerrad(&self) -> EccerradR {
        EccerradR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC Error Address."]
    #[inline(always)]
    pub fn eccerrad(&mut self) -> EccerradW<EccaddrSpec> {
        EccerradW::new(self, 0)
    }
}
#[doc = "ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccaddrSpec;
impl crate::RegisterSpec for EccaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccaddr::R`](R) reader structure"]
impl crate::Readable for EccaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccaddr::W`](W) writer structure"]
impl crate::Writable for EccaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCADDR to value 0"]
impl crate::Resettable for EccaddrSpec {
    const RESET_VALUE: u32 = 0;
}
