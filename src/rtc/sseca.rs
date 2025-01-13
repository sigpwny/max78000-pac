#[doc = "Register `SSECA` reader"]
pub type R = crate::R<SsecaSpec>;
#[doc = "Register `SSECA` writer"]
pub type W = crate::W<SsecaSpec>;
#[doc = "Field `SSEC_ALARM` reader - This register contains the reload value for the sub-second alarm."]
pub type SsecAlarmR = crate::FieldReader<u32>;
#[doc = "Field `SSEC_ALARM` writer - This register contains the reload value for the sub-second alarm."]
pub type SsecAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn ssec_alarm(&self) -> SsecAlarmR {
        SsecAlarmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub fn ssec_alarm(&mut self) -> SsecAlarmW<SsecaSpec> {
        SsecAlarmW::new(self, 0)
    }
}
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nYou can [`read`](crate::Reg::read) this register and get [`sseca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sseca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsecaSpec;
impl crate::RegisterSpec for SsecaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sseca::R`](R) reader structure"]
impl crate::Readable for SsecaSpec {}
#[doc = "`write(|w| ..)` method takes [`sseca::W`](W) writer structure"]
impl crate::Writable for SsecaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSECA to value 0"]
impl crate::Resettable for SsecaSpec {
    const RESET_VALUE: u32 = 0;
}
