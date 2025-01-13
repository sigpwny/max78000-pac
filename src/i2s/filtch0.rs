#[doc = "Register `FILTCH0` reader"]
pub type R = crate::R<Filtch0Spec>;
#[doc = "Register `FILTCH0` writer"]
pub type W = crate::W<Filtch0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Filter.\n\nYou can [`read`](crate::Reg::read) this register and get [`filtch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filtch0Spec;
impl crate::RegisterSpec for Filtch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filtch0::R`](R) reader structure"]
impl crate::Readable for Filtch0Spec {}
#[doc = "`write(|w| ..)` method takes [`filtch0::W`](W) writer structure"]
impl crate::Writable for Filtch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTCH0 to value 0"]
impl crate::Resettable for Filtch0Spec {
    const RESET_VALUE: u32 = 0;
}
