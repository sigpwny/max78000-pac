#[doc = "Register `NOLCMP` reader"]
pub type R = crate::R<NolcmpSpec>;
#[doc = "Register `NOLCMP` writer"]
pub type W = crate::W<NolcmpSpec>;
#[doc = "Field `LO_A` reader - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LoAR = crate::FieldReader;
#[doc = "Field `LO_A` writer - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LoAW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HI_A` reader - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HiAR = crate::FieldReader;
#[doc = "Field `HI_A` writer - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HiAW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LO_B` reader - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LoBR = crate::FieldReader;
#[doc = "Field `LO_B` writer - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
pub type LoBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HI_B` reader - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HiBR = crate::FieldReader;
#[doc = "Field `HI_B` writer - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
pub type HiBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_a(&self) -> LoAR {
        LoAR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_a(&self) -> HiAR {
        HiAR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_b(&self) -> LoBR {
        LoBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_b(&self) -> HiBR {
        HiBR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-Overlapping Low Compare value for Timer A controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_a(&mut self) -> LoAW<NolcmpSpec> {
        LoAW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non-Overlapping High Compare value for Timer A controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_a(&mut self) -> HiAW<NolcmpSpec> {
        HiAW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Non-Overlapping Low Compare value for Timer B controls the time between the falling edge of PWM Phase A and the next rising edge of PWM Phase A-Prime."]
    #[inline(always)]
    pub fn lo_b(&mut self) -> LoBW<NolcmpSpec> {
        LoBW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Non-Overlapping High Compare value for Timer B controls the time between the falling edge of PWM Phase A-Prime and the next rising edge of PWM Phase A."]
    #[inline(always)]
    pub fn hi_b(&mut self) -> HiBW<NolcmpSpec> {
        HiBW::new(self, 24)
    }
}
#[doc = "Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`nolcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nolcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NolcmpSpec;
impl crate::RegisterSpec for NolcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nolcmp::R`](R) reader structure"]
impl crate::Readable for NolcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`nolcmp::W`](W) writer structure"]
impl crate::Writable for NolcmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOLCMP to value 0"]
impl crate::Resettable for NolcmpSpec {
    const RESET_VALUE: u32 = 0;
}
