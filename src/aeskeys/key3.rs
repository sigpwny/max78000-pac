#[doc = "Register `KEY3` reader"]
pub type R = crate::R<Key3Spec>;
#[doc = "Register `KEY3` writer"]
pub type W = crate::W<Key3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES Key 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3Spec;
impl crate::RegisterSpec for Key3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3::R`](R) reader structure"]
impl crate::Readable for Key3Spec {}
#[doc = "`write(|w| ..)` method takes [`key3::W`](W) writer structure"]
impl crate::Writable for Key3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY3 to value 0"]
impl crate::Resettable for Key3Spec {
    const RESET_VALUE: u32 = 0;
}
