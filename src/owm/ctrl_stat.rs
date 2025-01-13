#[doc = "Register `CTRL_STAT` reader"]
pub type R = crate::R<CtrlStatSpec>;
#[doc = "Register `CTRL_STAT` writer"]
pub type W = crate::W<CtrlStatSpec>;
#[doc = "Field `start_ow_reset` reader - Start OW Reset."]
pub type StartOwResetR = crate::BitReader;
#[doc = "Field `start_ow_reset` writer - Start OW Reset."]
pub type StartOwResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sra_mode` reader - SRA Mode."]
pub type SraModeR = crate::BitReader;
#[doc = "Field `sra_mode` writer - SRA Mode."]
pub type SraModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_bang_oe` reader - Bit Bang Output Enable."]
pub type BitBangOeR = crate::BitReader;
#[doc = "Field `bit_bang_oe` writer - Bit Bang Output Enable."]
pub type BitBangOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ow_input` reader - OW Input State."]
pub type OwInputR = crate::BitReader;
#[doc = "Field `od_spec_mode` reader - Overdrive Spec Mode."]
pub type OdSpecModeR = crate::BitReader;
#[doc = "Field `presence_detect` reader - Presence Pulse Detected."]
pub type PresenceDetectR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    pub fn start_ow_reset(&self) -> StartOwResetR {
        StartOwResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    pub fn sra_mode(&self) -> SraModeR {
        SraModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    pub fn bit_bang_oe(&self) -> BitBangOeR {
        BitBangOeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Input State."]
    #[inline(always)]
    pub fn ow_input(&self) -> OwInputR {
        OwInputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overdrive Spec Mode."]
    #[inline(always)]
    pub fn od_spec_mode(&self) -> OdSpecModeR {
        OdSpecModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Presence Pulse Detected."]
    #[inline(always)]
    pub fn presence_detect(&self) -> PresenceDetectR {
        PresenceDetectR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    pub fn start_ow_reset(&mut self) -> StartOwResetW<CtrlStatSpec> {
        StartOwResetW::new(self, 0)
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    pub fn sra_mode(&mut self) -> SraModeW<CtrlStatSpec> {
        SraModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    pub fn bit_bang_oe(&mut self) -> BitBangOeW<CtrlStatSpec> {
        BitBangOeW::new(self, 2)
    }
}
#[doc = "1-Wire Master Control/Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlStatSpec;
impl crate::RegisterSpec for CtrlStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_stat::R`](R) reader structure"]
impl crate::Readable for CtrlStatSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_stat::W`](W) writer structure"]
impl crate::Writable for CtrlStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_STAT to value 0"]
impl crate::Resettable for CtrlStatSpec {
    const RESET_VALUE: u32 = 0;
}
