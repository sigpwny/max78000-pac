#[doc = "Register `KEY6` reader"]
pub type R = crate::R<Key6Spec>;
#[doc = "Register `KEY6` writer"]
pub type W = crate::W<Key6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES Key 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`key6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key6Spec;
impl crate::RegisterSpec for Key6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6::R`](R) reader structure"]
impl crate::Readable for Key6Spec {}
#[doc = "`write(|w| ..)` method takes [`key6::W`](W) writer structure"]
impl crate::Writable for Key6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY6 to value 0"]
impl crate::Resettable for Key6Spec {
    const RESET_VALUE: u32 = 0;
}
