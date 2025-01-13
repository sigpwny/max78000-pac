#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WkflSpec>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WkflSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wakeup Flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkflSpec;
impl crate::RegisterSpec for WkflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WkflSpec {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WkflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WkflSpec {
    const RESET_VALUE: u32 = 0;
}
