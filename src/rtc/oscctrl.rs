#[doc = "Register `OSCCTRL` reader"]
pub type R = crate::R<OscctrlSpec>;
#[doc = "Register `OSCCTRL` writer"]
pub type W = crate::W<OscctrlSpec>;
#[doc = "Field `FILTER_EN` reader - Enables analog deglitch filter."]
pub type FilterEnR = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Enables analog deglitch filter."]
pub type FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_SEL` reader - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IbiasSelR = crate::BitReader;
#[doc = "Field `IBIAS_SEL` writer - If IBIAS_EN is 1, selects 4x,2x mode."]
pub type IbiasSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST_EN` reader - Enables high current hysteresis buffer."]
pub type HystEnR = crate::BitReader;
#[doc = "Field `HYST_EN` writer - Enables high current hysteresis buffer."]
pub type HystEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIAS_EN` reader - Enables higher 4x,2x current modes."]
pub type IbiasEnR = crate::BitReader;
#[doc = "Field `IBIAS_EN` writer - Enables higher 4x,2x current modes."]
pub type IbiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - RTC Crystal Bypass"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - RTC Crystal Bypass"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQW_32K` reader - RTC 32kHz Square Wave Output"]
pub type Sqw32kR = crate::BitReader;
#[doc = "Field `SQW_32K` writer - RTC 32kHz Square Wave Output"]
pub type Sqw32kW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IbiasSelR {
        IbiasSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    pub fn hyst_en(&self) -> HystEnR {
        HystEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    pub fn ibias_en(&self) -> IbiasEnR {
        IbiasEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn sqw_32k(&self) -> Sqw32kR {
        Sqw32kR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables analog deglitch filter."]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<OscctrlSpec> {
        FilterEnW::new(self, 0)
    }
    #[doc = "Bit 1 - If IBIAS_EN is 1, selects 4x,2x mode."]
    #[inline(always)]
    pub fn ibias_sel(&mut self) -> IbiasSelW<OscctrlSpec> {
        IbiasSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables high current hysteresis buffer."]
    #[inline(always)]
    pub fn hyst_en(&mut self) -> HystEnW<OscctrlSpec> {
        HystEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables higher 4x,2x current modes."]
    #[inline(always)]
    pub fn ibias_en(&mut self) -> IbiasEnW<OscctrlSpec> {
        IbiasEnW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<OscctrlSpec> {
        BypassW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn sqw_32k(&mut self) -> Sqw32kW<OscctrlSpec> {
        Sqw32kW::new(self, 5)
    }
}
#[doc = "RTC Oscillator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`oscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscctrlSpec;
impl crate::RegisterSpec for OscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oscctrl::R`](R) reader structure"]
impl crate::Readable for OscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`oscctrl::W`](W) writer structure"]
impl crate::Writable for OscctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCCTRL to value 0"]
impl crate::Resettable for OscctrlSpec {
    const RESET_VALUE: u32 = 0;
}
