#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WkenSpec>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WkenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wakeup Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkenSpec;
impl crate::RegisterSpec for WkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WkenSpec {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WkenSpec {
    const RESET_VALUE: u32 = 0;
}
