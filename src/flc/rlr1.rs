#[doc = "Register `RLR1` reader"]
pub type R = crate::R<Rlr1Spec>;
#[doc = "Register `RLR1` writer"]
pub type W = crate::W<Rlr1Spec>;
#[doc = "Field `RLR1` reader - Access control."]
pub type Rlr1R = crate::FieldReader<u32>;
#[doc = "Field `RLR1` writer - Access control."]
pub type Rlr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr1(&self) -> Rlr1R {
        Rlr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr1(&mut self) -> Rlr1W<Rlr1Spec> {
        Rlr1W::new(self, 0)
    }
}
#[doc = "RLR1\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rlr1Spec;
impl crate::RegisterSpec for Rlr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr1::R`](R) reader structure"]
impl crate::Readable for Rlr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rlr1::W`](W) writer structure"]
impl crate::Writable for Rlr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLR1 to value 0"]
impl crate::Resettable for Rlr1Spec {
    const RESET_VALUE: u32 = 0;
}
