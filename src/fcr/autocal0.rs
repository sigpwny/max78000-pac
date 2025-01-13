#[doc = "Register `AUTOCAL0` reader"]
pub type R = crate::R<Autocal0Spec>;
#[doc = "Register `AUTOCAL0` writer"]
pub type W = crate::W<Autocal0Spec>;
#[doc = "Auto-calibration Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acen {
    #[doc = "0: Disabled."]
    Dis = 0,
    #[doc = "1: Enabled."]
    En = 1,
}
impl From<Acen> for bool {
    #[inline(always)]
    fn from(variant: Acen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACEN` reader - Auto-calibration Enable."]
pub type AcenR = crate::BitReader<Acen>;
impl AcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acen {
        match self.bits {
            false => Acen::Dis,
            true => Acen::En,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Acen::Dis
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Acen::En
    }
}
#[doc = "Field `ACEN` writer - Auto-calibration Enable."]
pub type AcenW<'a, REG> = crate::BitWriter<'a, REG, Acen>;
impl<'a, REG> AcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Acen::Dis)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Acen::En)
    }
}
#[doc = "Autocalibration Run.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acrun {
    #[doc = "0: Not Running."]
    Not = 0,
    #[doc = "1: Running."]
    Run = 1,
}
impl From<Acrun> for bool {
    #[inline(always)]
    fn from(variant: Acrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACRUN` reader - Autocalibration Run."]
pub type AcrunR = crate::BitReader<Acrun>;
impl AcrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acrun {
        match self.bits {
            false => Acrun::Not,
            true => Acrun::Run,
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Acrun::Not
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Acrun::Run
    }
}
#[doc = "Field `ACRUN` writer - Autocalibration Run."]
pub type AcrunW<'a, REG> = crate::BitWriter<'a, REG, Acrun>;
impl<'a, REG> AcrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut crate::W<REG> {
        self.variant(Acrun::Not)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Acrun::Run)
    }
}
#[doc = "Field `LDTRM` reader - Load Trim."]
pub type LdtrmR = crate::BitReader;
#[doc = "Field `LDTRM` writer - Load Trim."]
pub type LdtrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Invert Gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gaininv {
    #[doc = "0: Not Running."]
    Not = 0,
    #[doc = "1: Running."]
    Run = 1,
}
impl From<Gaininv> for bool {
    #[inline(always)]
    fn from(variant: Gaininv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GAININV` reader - Invert Gain."]
pub type GaininvR = crate::BitReader<Gaininv>;
impl GaininvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gaininv {
        match self.bits {
            false => Gaininv::Not,
            true => Gaininv::Run,
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Gaininv::Not
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Gaininv::Run
    }
}
#[doc = "Field `GAININV` writer - Invert Gain."]
pub type GaininvW<'a, REG> = crate::BitWriter<'a, REG, Gaininv>;
impl<'a, REG> GaininvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut crate::W<REG> {
        self.variant(Gaininv::Not)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Gaininv::Run)
    }
}
#[doc = "Atomic mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atomic {
    #[doc = "0: Not Running."]
    Not = 0,
    #[doc = "1: Running."]
    Run = 1,
}
impl From<Atomic> for bool {
    #[inline(always)]
    fn from(variant: Atomic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATOMIC` reader - Atomic mode."]
pub type AtomicR = crate::BitReader<Atomic>;
impl AtomicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atomic {
        match self.bits {
            false => Atomic::Not,
            true => Atomic::Run,
        }
    }
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Atomic::Not
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Atomic::Run
    }
}
#[doc = "Field `ATOMIC` writer - Atomic mode."]
pub type AtomicW<'a, REG> = crate::BitWriter<'a, REG, Atomic>;
impl<'a, REG> AtomicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Running."]
    #[inline(always)]
    pub fn not(self) -> &'a mut crate::W<REG> {
        self.variant(Atomic::Not)
    }
    #[doc = "Running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Atomic::Run)
    }
}
#[doc = "Field `MU` reader - MU value."]
pub type MuR = crate::FieldReader<u16>;
#[doc = "Field `MU` writer - MU value."]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HIRC96MACTMROUT` reader - HIRC96M Trim Value."]
pub type Hirc96mactmroutR = crate::FieldReader<u16>;
#[doc = "Field `HIRC96MACTMROUT` writer - HIRC96M Trim Value."]
pub type Hirc96mactmroutW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    pub fn acen(&self) -> AcenR {
        AcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    pub fn acrun(&self) -> AcrunR {
        AcrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    pub fn ldtrm(&self) -> LdtrmR {
        LdtrmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    pub fn gaininv(&self) -> GaininvR {
        GaininvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    pub fn atomic(&self) -> AtomicR {
        AtomicR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 23:31 - HIRC96M Trim Value."]
    #[inline(always)]
    pub fn hirc96mactmrout(&self) -> Hirc96mactmroutR {
        Hirc96mactmroutR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-calibration Enable."]
    #[inline(always)]
    pub fn acen(&mut self) -> AcenW<Autocal0Spec> {
        AcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Autocalibration Run."]
    #[inline(always)]
    pub fn acrun(&mut self) -> AcrunW<Autocal0Spec> {
        AcrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Load Trim."]
    #[inline(always)]
    pub fn ldtrm(&mut self) -> LdtrmW<Autocal0Spec> {
        LdtrmW::new(self, 2)
    }
    #[doc = "Bit 3 - Invert Gain."]
    #[inline(always)]
    pub fn gaininv(&mut self) -> GaininvW<Autocal0Spec> {
        GaininvW::new(self, 3)
    }
    #[doc = "Bit 4 - Atomic mode."]
    #[inline(always)]
    pub fn atomic(&mut self) -> AtomicW<Autocal0Spec> {
        AtomicW::new(self, 4)
    }
    #[doc = "Bits 8:19 - MU value."]
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<Autocal0Spec> {
        MuW::new(self, 8)
    }
    #[doc = "Bits 23:31 - HIRC96M Trim Value."]
    #[inline(always)]
    pub fn hirc96mactmrout(&mut self) -> Hirc96mactmroutW<Autocal0Spec> {
        Hirc96mactmroutW::new(self, 23)
    }
}
#[doc = "Automatic Calibration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autocal0Spec;
impl crate::RegisterSpec for Autocal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocal0::R`](R) reader structure"]
impl crate::Readable for Autocal0Spec {}
#[doc = "`write(|w| ..)` method takes [`autocal0::W`](W) writer structure"]
impl crate::Writable for Autocal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCAL0 to value 0"]
impl crate::Resettable for Autocal0Spec {
    const RESET_VALUE: u32 = 0;
}
