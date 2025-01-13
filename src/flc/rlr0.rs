#[doc = "Register `RLR0` reader"]
pub type R = crate::R<Rlr0Spec>;
#[doc = "Register `RLR0` writer"]
pub type W = crate::W<Rlr0Spec>;
#[doc = "Field `RLR0` reader - Access control."]
pub type Rlr0R = crate::FieldReader<u32>;
#[doc = "Field `RLR0` writer - Access control."]
pub type Rlr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr0(&self) -> Rlr0R {
        Rlr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr0(&mut self) -> Rlr0W<Rlr0Spec> {
        Rlr0W::new(self, 0)
    }
}
#[doc = "RLR0\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rlr0Spec;
impl crate::RegisterSpec for Rlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr0::R`](R) reader structure"]
impl crate::Readable for Rlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`rlr0::W`](W) writer structure"]
impl crate::Writable for Rlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLR0 to value 0"]
impl crate::Resettable for Rlr0Spec {
    const RESET_VALUE: u32 = 0;
}
