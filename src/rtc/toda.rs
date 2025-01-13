#[doc = "Register `TODA` reader"]
pub type R = crate::R<TodaSpec>;
#[doc = "Register `TODA` writer"]
pub type W = crate::W<TodaSpec>;
#[doc = "Field `TOD_ALARM` reader - Time-of-day Alarm."]
pub type TodAlarmR = crate::FieldReader<u32>;
#[doc = "Field `TOD_ALARM` writer - Time-of-day Alarm."]
pub type TodAlarmW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn tod_alarm(&self) -> TodAlarmR {
        TodAlarmR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn tod_alarm(&mut self) -> TodAlarmW<TodaSpec> {
        TodAlarmW::new(self, 0)
    }
}
#[doc = "Time-of-day Alarm.\n\nYou can [`read`](crate::Reg::read) this register and get [`toda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TodaSpec;
impl crate::RegisterSpec for TodaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`toda::R`](R) reader structure"]
impl crate::Readable for TodaSpec {}
#[doc = "`write(|w| ..)` method takes [`toda::W`](W) writer structure"]
impl crate::Writable for TodaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TODA to value 0"]
impl crate::Resettable for TodaSpec {
    const RESET_VALUE: u32 = 0;
}
