#[doc = "Register `KEY4` reader"]
pub type R = crate::R<Key4Spec>;
#[doc = "Register `KEY4` writer"]
pub type W = crate::W<Key4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES Key 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`key4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key4Spec;
impl crate::RegisterSpec for Key4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4::R`](R) reader structure"]
impl crate::Readable for Key4Spec {}
#[doc = "`write(|w| ..)` method takes [`key4::W`](W) writer structure"]
impl crate::Writable for Key4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY4 to value 0"]
impl crate::Resettable for Key4Spec {
    const RESET_VALUE: u32 = 0;
}
