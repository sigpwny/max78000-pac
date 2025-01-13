#[doc = "Register `WELR1` reader"]
pub type R = crate::R<Welr1Spec>;
#[doc = "Register `WELR1` writer"]
pub type W = crate::W<Welr1Spec>;
#[doc = "Field `WELR1` reader - Access control."]
pub type Welr1R = crate::FieldReader<u32>;
#[doc = "Field `WELR1` writer - Access control."]
pub type Welr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr1(&self) -> Welr1R {
        Welr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr1(&mut self) -> Welr1W<Welr1Spec> {
        Welr1W::new(self, 0)
    }
}
#[doc = "WELR1\n\nYou can [`read`](crate::Reg::read) this register and get [`welr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Welr1Spec;
impl crate::RegisterSpec for Welr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr1::R`](R) reader structure"]
impl crate::Readable for Welr1Spec {}
#[doc = "`write(|w| ..)` method takes [`welr1::W`](W) writer structure"]
impl crate::Writable for Welr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WELR1 to value 0"]
impl crate::Resettable for Welr1Spec {
    const RESET_VALUE: u32 = 0;
}
