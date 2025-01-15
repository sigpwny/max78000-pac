#[doc = "Register `PWM` reader"]
pub type R = crate::R<PwmSpec>;
#[doc = "Register `PWM` writer"]
pub type W = crate::W<PwmSpec>;
#[doc = "Field `PWM` reader - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PwmR = crate::FieldReader<u32>;
#[doc = "Field `PWM` writer - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
pub type PwmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    pub fn pwm(&self) -> PwmR {
        PwmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer PWM Match: In PWM Mode, this field sets the count value for the first transition period of the PWM cycle. At the end of the cycle where CNT equals PWM, the PWM output transitions to the second period of the PWM cycle. The second PWM period count is stored in the CMP register. The value set for PWM must me less than the value set in CMP for PWM mode operation. Timer Capture Value: In Capture, Compare, and Capture/Compare modes, this field is used to store the CNT value when a Capture, Compare, or Capture/Compare event occurs."]
    #[inline(always)]
    pub fn pwm(&mut self) -> PwmW<PwmSpec> {
        PwmW::new(self, 0)
    }
}
#[doc = "Timer PWM Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmSpec;
impl crate::RegisterSpec for PwmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm::R`](R) reader structure"]
impl crate::Readable for PwmSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm::W`](W) writer structure"]
impl crate::Writable for PwmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM to value 0"]
impl crate::Resettable for PwmSpec {
    const RESET_VALUE: u32 = 0;
}
