#[doc = "Register `SLAVE2` reader"]
pub type R = crate::R<Slave2Spec>;
#[doc = "Register `SLAVE2` writer"]
pub type W = crate::W<Slave2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slave2Spec;
impl crate::RegisterSpec for Slave2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave2::R`](R) reader structure"]
impl crate::Readable for Slave2Spec {}
#[doc = "`write(|w| ..)` method takes [`slave2::W`](W) writer structure"]
impl crate::Writable for Slave2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE2 to value 0"]
impl crate::Resettable for Slave2Spec {
    const RESET_VALUE: u32 = 0;
}
