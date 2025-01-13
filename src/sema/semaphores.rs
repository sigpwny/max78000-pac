#[doc = "Register `SEMAPHORES[%s]` reader"]
pub type R = crate::R<SemaphoresSpec>;
#[doc = "Register `SEMAPHORES[%s]` writer"]
pub type W = crate::W<SemaphoresSpec>;
#[doc = "Field `sema` reader - "]
pub type SemaR = crate::BitReader;
#[doc = "Field `sema` writer - "]
pub type SemaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&self) -> SemaR {
        SemaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&mut self) -> SemaW<SemaphoresSpec> {
        SemaW::new(self, 0)
    }
}
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nYou can [`read`](crate::Reg::read) this register and get [`semaphores::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semaphores::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SemaphoresSpec;
impl crate::RegisterSpec for SemaphoresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`semaphores::R`](R) reader structure"]
impl crate::Readable for SemaphoresSpec {}
#[doc = "`write(|w| ..)` method takes [`semaphores::W`](W) writer structure"]
impl crate::Writable for SemaphoresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEMAPHORES[%s]
to value 0"]
impl crate::Resettable for SemaphoresSpec {
    const RESET_VALUE: u32 = 0;
}
