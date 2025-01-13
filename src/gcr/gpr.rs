#[doc = "Register `GPR` reader"]
pub type R = crate::R<GprSpec>;
#[doc = "Register `GPR` writer"]
pub type W = crate::W<GprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General Purpose Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GprSpec;
impl crate::RegisterSpec for GprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpr::R`](R) reader structure"]
impl crate::Readable for GprSpec {}
#[doc = "`write(|w| ..)` method takes [`gpr::W`](W) writer structure"]
impl crate::Writable for GprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPR to value 0"]
impl crate::Resettable for GprSpec {
    const RESET_VALUE: u32 = 0;
}
