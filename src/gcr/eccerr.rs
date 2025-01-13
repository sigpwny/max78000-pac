#[doc = "Register `ECCERR` reader"]
pub type R = crate::R<EccerrSpec>;
#[doc = "Register `ECCERR` writer"]
pub type W = crate::W<EccerrSpec>;
#[doc = "Field `RAM` reader - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<EccerrSpec> {
        RamW::new(self, 0)
    }
}
#[doc = "ECC Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccerrSpec;
impl crate::RegisterSpec for EccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccerr::R`](R) reader structure"]
impl crate::Readable for EccerrSpec {}
#[doc = "`write(|w| ..)` method takes [`eccerr::W`](W) writer structure"]
impl crate::Writable for EccerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCERR to value 0"]
impl crate::Resettable for EccerrSpec {
    const RESET_VALUE: u32 = 0;
}
