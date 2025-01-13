#[doc = "Register `KEY7` reader"]
pub type R = crate::R<Key7Spec>;
#[doc = "Register `KEY7` writer"]
pub type W = crate::W<Key7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES Key 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`key7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key7Spec;
impl crate::RegisterSpec for Key7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key7::R`](R) reader structure"]
impl crate::Readable for Key7Spec {}
#[doc = "`write(|w| ..)` method takes [`key7::W`](W) writer structure"]
impl crate::Writable for Key7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY7 to value 0"]
impl crate::Resettable for Key7Spec {
    const RESET_VALUE: u32 = 0;
}
