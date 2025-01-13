#[doc = "Register `SSEC` reader"]
pub type R = crate::R<SsecSpec>;
#[doc = "Register `SSEC` writer"]
pub type W = crate::W<SsecSpec>;
#[doc = "Field `SSEC` reader - Sub-Seconds Counter (12-bit)."]
pub type SsecR = crate::FieldReader<u16>;
#[doc = "Field `SSEC` writer - Sub-Seconds Counter (12-bit)."]
pub type SsecW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    pub fn ssec(&self) -> SsecR {
        SsecR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sub-Seconds Counter (12-bit)."]
    #[inline(always)]
    pub fn ssec(&mut self) -> SsecW<SsecSpec> {
        SsecW::new(self, 0)
    }
}
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsecSpec;
impl crate::RegisterSpec for SsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssec::R`](R) reader structure"]
impl crate::Readable for SsecSpec {}
#[doc = "`write(|w| ..)` method takes [`ssec::W`](W) writer structure"]
impl crate::Writable for SsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSEC to value 0"]
impl crate::Resettable for SsecSpec {
    const RESET_VALUE: u32 = 0;
}
