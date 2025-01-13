#[doc = "Register `TRAIN` reader"]
pub type R = crate::R<TrainSpec>;
#[doc = "Register `TRAIN` writer"]
pub type W = crate::W<TrainSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length.\n\nYou can [`read`](crate::Reg::read) this register and get [`train::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`train::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrainSpec;
impl crate::RegisterSpec for TrainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`train::R`](R) reader structure"]
impl crate::Readable for TrainSpec {}
#[doc = "`write(|w| ..)` method takes [`train::W`](W) writer structure"]
impl crate::Writable for TrainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAIN to value 0"]
impl crate::Resettable for TrainSpec {
    const RESET_VALUE: u32 = 0;
}
