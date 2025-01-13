#[doc = "Register `ECCDATA` reader"]
pub type R = crate::R<EccdataSpec>;
#[doc = "Register `ECCDATA` writer"]
pub type W = crate::W<EccdataSpec>;
#[doc = "Field `EVEN` reader - Error Correction Code Odd Data."]
pub type EvenR = crate::FieldReader<u16>;
#[doc = "Field `EVEN` writer - Error Correction Code Odd Data."]
pub type EvenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ODD` reader - Error Correction Code Even Data."]
pub type OddR = crate::FieldReader<u16>;
#[doc = "Field `ODD` writer - Error Correction Code Even Data."]
pub type OddW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    pub fn even(&self) -> EvenR {
        EvenR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Error Correction Code Odd Data."]
    #[inline(always)]
    pub fn even(&mut self) -> EvenW<EccdataSpec> {
        EvenW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Error Correction Code Even Data."]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<EccdataSpec> {
        OddW::new(self, 16)
    }
}
#[doc = "ECC Data Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccdataSpec;
impl crate::RegisterSpec for EccdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdata::R`](R) reader structure"]
impl crate::Readable for EccdataSpec {}
#[doc = "`write(|w| ..)` method takes [`eccdata::W`](W) writer structure"]
impl crate::Writable for EccdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCDATA to value 0"]
impl crate::Resettable for EccdataSpec {
    const RESET_VALUE: u32 = 0;
}
