#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `long_line_mode` reader - Long Line Mode."]
pub type LongLineModeR = crate::BitReader;
#[doc = "Field `long_line_mode` writer - Long Line Mode."]
pub type LongLineModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_pres_det` reader - Force Line During Presence Detect."]
pub type ForcePresDetR = crate::BitReader;
#[doc = "Field `force_pres_det` writer - Force Line During Presence Detect."]
pub type ForcePresDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_bang_en` reader - Bit Bang Enable."]
pub type BitBangEnR = crate::BitReader;
#[doc = "Field `bit_bang_en` writer - Bit Bang Enable."]
pub type BitBangEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_pullup_mode` reader - Provide an extra output control to control an external pullup."]
pub type ExtPullupModeR = crate::BitReader;
#[doc = "Field `ext_pullup_mode` writer - Provide an extra output control to control an external pullup."]
pub type ExtPullupModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_pullup_enable` reader - Enable External Pullup."]
pub type ExtPullupEnableR = crate::BitReader;
#[doc = "Field `ext_pullup_enable` writer - Enable External Pullup."]
pub type ExtPullupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `single_bit_mode` reader - Enable Single Bit TX/RX Mode."]
pub type SingleBitModeR = crate::BitReader;
#[doc = "Field `single_bit_mode` writer - Enable Single Bit TX/RX Mode."]
pub type SingleBitModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `overdrive` reader - Enables overdrive speed for 1-Wire operations."]
pub type OverdriveR = crate::BitReader;
#[doc = "Field `overdrive` writer - Enables overdrive speed for 1-Wire operations."]
pub type OverdriveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_pullup_enable` reader - Enable intenral pullup."]
pub type IntPullupEnableR = crate::BitReader;
#[doc = "Field `int_pullup_enable` writer - Enable intenral pullup."]
pub type IntPullupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    pub fn long_line_mode(&self) -> LongLineModeR {
        LongLineModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    pub fn force_pres_det(&self) -> ForcePresDetR {
        ForcePresDetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    pub fn bit_bang_en(&self) -> BitBangEnR {
        BitBangEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&self) -> ExtPullupModeR {
        ExtPullupModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    pub fn ext_pullup_enable(&self) -> ExtPullupEnableR {
        ExtPullupEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    pub fn single_bit_mode(&self) -> SingleBitModeR {
        SingleBitModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&self) -> OverdriveR {
        OverdriveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&self) -> IntPullupEnableR {
        IntPullupEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    pub fn long_line_mode(&mut self) -> LongLineModeW<CfgSpec> {
        LongLineModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    pub fn force_pres_det(&mut self) -> ForcePresDetW<CfgSpec> {
        ForcePresDetW::new(self, 1)
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    pub fn bit_bang_en(&mut self) -> BitBangEnW<CfgSpec> {
        BitBangEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&mut self) -> ExtPullupModeW<CfgSpec> {
        ExtPullupModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    pub fn ext_pullup_enable(&mut self) -> ExtPullupEnableW<CfgSpec> {
        ExtPullupEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    pub fn single_bit_mode(&mut self) -> SingleBitModeW<CfgSpec> {
        SingleBitModeW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&mut self) -> OverdriveW<CfgSpec> {
        OverdriveW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&mut self) -> IntPullupEnableW<CfgSpec> {
        IntPullupEnableW::new(self, 7)
    }
}
#[doc = "1-Wire Master Configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
