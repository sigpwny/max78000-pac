#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `INRO_EN` reader - INRO Enable."]
pub type InroEnR = crate::BitReader;
#[doc = "Field `INRO_EN` writer - INRO Enable."]
pub type InroEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTCO_EN` reader - ERTCO Enable."]
pub type ErtcoEnR = crate::BitReader;
#[doc = "Field `ERTCO_EN` writer - ERTCO Enable."]
pub type ErtcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMO_CLKSCL_EN` reader - SIMO Clock Scaling Enable."]
pub type SimoClksclEnR = crate::BitReader;
#[doc = "Field `SIMO_CLKSCL_EN` writer - SIMO Clock Scaling Enable."]
pub type SimoClksclEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMO_RSTD` reader - SIMO System Reset Disable."]
pub type SimoRstdR = crate::BitReader;
#[doc = "Field `SIMO_RSTD` writer - SIMO System Reset Disable."]
pub type SimoRstdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    pub fn inro_en(&self) -> InroEnR {
        InroEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    pub fn ertco_en(&self) -> ErtcoEnR {
        ErtcoEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    pub fn simo_clkscl_en(&self) -> SimoClksclEnR {
        SimoClksclEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    pub fn simo_rstd(&self) -> SimoRstdR {
        SimoRstdR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - INRO Enable."]
    #[inline(always)]
    pub fn inro_en(&mut self) -> InroEnW<CtrlSpec> {
        InroEnW::new(self, 2)
    }
    #[doc = "Bit 3 - ERTCO Enable."]
    #[inline(always)]
    pub fn ertco_en(&mut self) -> ErtcoEnW<CtrlSpec> {
        ErtcoEnW::new(self, 3)
    }
    #[doc = "Bit 8 - SIMO Clock Scaling Enable."]
    #[inline(always)]
    pub fn simo_clkscl_en(&mut self) -> SimoClksclEnW<CtrlSpec> {
        SimoClksclEnW::new(self, 8)
    }
    #[doc = "Bit 9 - SIMO System Reset Disable."]
    #[inline(always)]
    pub fn simo_rstd(&mut self) -> SimoRstdW<CtrlSpec> {
        SimoRstdW::new(self, 9)
    }
}
#[doc = "Miscellaneous Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
