#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `SEC` reader - Seconds Counter."]
pub type SecR = crate::FieldReader<u32>;
#[doc = "Field `SEC` writer - Seconds Counter."]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Seconds Counter."]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Seconds Counter."]
    #[inline(always)]
    pub fn sec(&mut self) -> SecW<SecSpec> {
        SecW::new(self, 0)
    }
}
#[doc = "RTC Second Counter. This register contains the 32-bit second counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {
    const RESET_VALUE: u32 = 0;
}
