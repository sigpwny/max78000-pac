#[doc = "Register `GP1` reader"]
pub type R = crate::R<Gp1Spec>;
#[doc = "Register `GP1` writer"]
pub type W = crate::W<Gp1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "General Purpose Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gp1Spec;
impl crate::RegisterSpec for Gp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp1::R`](R) reader structure"]
impl crate::Readable for Gp1Spec {}
#[doc = "`write(|w| ..)` method takes [`gp1::W`](W) writer structure"]
impl crate::Writable for Gp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GP1 to value 0"]
impl crate::Resettable for Gp1Spec {
    const RESET_VALUE: u32 = 0;
}
