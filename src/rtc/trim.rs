#[doc = "Register `TRIM` reader"]
pub type R = crate::R<TrimSpec>;
#[doc = "Register `TRIM` writer"]
pub type W = crate::W<TrimSpec>;
#[doc = "Field `TRIM` reader - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VRTC_TMR` reader - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
pub type VrtcTmrR = crate::FieldReader<u32>;
#[doc = "Field `VRTC_TMR` writer - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
pub type VrtcTmrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    pub fn vrtc_tmr(&self) -> VrtcTmrR {
        VrtcTmrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Trim. This register contains the 2's complement value that specifies the trim resolution. Each increment or decrement of the bit adds or subtracts 1ppm at each 4KHz clock value, with a maximum correction of +/- 127ppm."]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<TrimSpec> {
        TrimW::new(self, 0)
    }
    #[doc = "Bits 8:31 - VBAT Timer Value. When RTC is running off of VBAT, this field is incremented every 32 seconds."]
    #[inline(always)]
    pub fn vrtc_tmr(&mut self) -> VrtcTmrW<TrimSpec> {
        VrtcTmrW::new(self, 8)
    }
}
#[doc = "RTC Trim Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimSpec;
impl crate::RegisterSpec for TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim::R`](R) reader structure"]
impl crate::Readable for TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`trim::W`](W) writer structure"]
impl crate::Writable for TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TrimSpec {
    const RESET_VALUE: u32 = 0;
}
