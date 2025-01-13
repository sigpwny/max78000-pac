#[doc = "Register `INEN` reader"]
pub type R = crate::R<InenSpec>;
#[doc = "Register `INEN` writer"]
pub type W = crate::W<InenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GPIO Input Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InenSpec;
impl crate::RegisterSpec for InenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inen::R`](R) reader structure"]
impl crate::Readable for InenSpec {}
#[doc = "`write(|w| ..)` method takes [`inen::W`](W) writer structure"]
impl crate::Writable for InenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INEN to value 0"]
impl crate::Resettable for InenSpec {
    const RESET_VALUE: u32 = 0;
}
