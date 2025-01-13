#[doc = "Register `WELR0` reader"]
pub type R = crate::R<Welr0Spec>;
#[doc = "Register `WELR0` writer"]
pub type W = crate::W<Welr0Spec>;
#[doc = "Field `WELR0` reader - Access control."]
pub type Welr0R = crate::FieldReader<u32>;
#[doc = "Field `WELR0` writer - Access control."]
pub type Welr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr0(&self) -> Welr0R {
        Welr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr0(&mut self) -> Welr0W<Welr0Spec> {
        Welr0W::new(self, 0)
    }
}
#[doc = "WELR0\n\nYou can [`read`](crate::Reg::read) this register and get [`welr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Welr0Spec;
impl crate::RegisterSpec for Welr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr0::R`](R) reader structure"]
impl crate::Readable for Welr0Spec {}
#[doc = "`write(|w| ..)` method takes [`welr0::W`](W) writer structure"]
impl crate::Writable for Welr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WELR0 to value 0"]
impl crate::Resettable for Welr0Spec {
    const RESET_VALUE: u32 = 0;
}
