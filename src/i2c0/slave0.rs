#[doc = "Register `SLAVE0` reader"]
pub type R = crate::R<Slave0Spec>;
#[doc = "Register `SLAVE0` writer"]
pub type W = crate::W<Slave0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slave0Spec;
impl crate::RegisterSpec for Slave0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave0::R`](R) reader structure"]
impl crate::Readable for Slave0Spec {}
#[doc = "`write(|w| ..)` method takes [`slave0::W`](W) writer structure"]
impl crate::Writable for Slave0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE0 to value 0"]
impl crate::Resettable for Slave0Spec {
    const RESET_VALUE: u32 = 0;
}
