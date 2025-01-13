#[doc = "Register `ECCIE` reader"]
pub type R = crate::R<EccieSpec>;
#[doc = "Register `ECCIE` writer"]
pub type W = crate::W<EccieSpec>;
#[doc = "Field `RAM` reader - ECC System RAM0 Error Interrup Enable"]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Interrup Enable"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Interrup Enable"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Interrup Enable"]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<EccieSpec> {
        RamW::new(self, 0)
    }
}
#[doc = "ECC IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccieSpec;
impl crate::RegisterSpec for EccieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccie::R`](R) reader structure"]
impl crate::Readable for EccieSpec {}
#[doc = "`write(|w| ..)` method takes [`eccie::W`](W) writer structure"]
impl crate::Writable for EccieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCIE to value 0"]
impl crate::Resettable for EccieSpec {
    const RESET_VALUE: u32 = 0;
}
