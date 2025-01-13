#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<ClkselSpec>;
#[doc = "Field `SOURCE` reader - WWDT Clock Selection Register."]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - WWDT Clock Selection Register."]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WWDT Clock Selection Register."]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<ClkselSpec> {
        SourceW::new(self, 0)
    }
}
#[doc = "Windowed Watchdog Timer Clock Select Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
