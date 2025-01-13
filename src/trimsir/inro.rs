#[doc = "Register `INRO` reader"]
pub type R = crate::R<InroSpec>;
#[doc = "Register `INRO` writer"]
pub type W = crate::W<InroSpec>;
#[doc = "Field `TRIM16K` reader - INRO 16KHz Trim."]
pub type Trim16kR = crate::FieldReader;
#[doc = "Field `TRIM16K` writer - INRO 16KHz Trim."]
pub type Trim16kW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIM30K` reader - INRO 30KHz Trim."]
pub type Trim30kR = crate::FieldReader;
#[doc = "Field `TRIM30K` writer - INRO 30KHz Trim."]
pub type Trim30kW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "INRO Low Power Mode Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpclksel {
    #[doc = "0: `0`"]
    _8khz = 0,
    #[doc = "1: `1`"]
    _16khz = 1,
    #[doc = "2: `10`"]
    _30khz = 2,
}
impl From<Lpclksel> for u8 {
    #[inline(always)]
    fn from(variant: Lpclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpclksel {
    type Ux = u8;
}
impl crate::IsEnum for Lpclksel {}
#[doc = "Field `LPCLKSEL` reader - INRO Low Power Mode Clock Select."]
pub type LpclkselR = crate::FieldReader<Lpclksel>;
impl LpclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpclksel> {
        match self.bits {
            0 => Some(Lpclksel::_8khz),
            1 => Some(Lpclksel::_16khz),
            2 => Some(Lpclksel::_30khz),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == Lpclksel::_8khz
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == Lpclksel::_16khz
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_30khz(&self) -> bool {
        *self == Lpclksel::_30khz
    }
}
#[doc = "Field `LPCLKSEL` writer - INRO Low Power Mode Clock Select."]
pub type LpclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpclksel>;
impl<'a, REG> LpclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut crate::W<REG> {
        self.variant(Lpclksel::_8khz)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut crate::W<REG> {
        self.variant(Lpclksel::_16khz)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _30khz(self) -> &'a mut crate::W<REG> {
        self.variant(Lpclksel::_30khz)
    }
}
impl R {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    pub fn trim16k(&self) -> Trim16kR {
        Trim16kR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    pub fn trim30k(&self) -> Trim30kR {
        Trim30kR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpclksel(&self) -> LpclkselR {
        LpclkselR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - INRO 16KHz Trim."]
    #[inline(always)]
    pub fn trim16k(&mut self) -> Trim16kW<InroSpec> {
        Trim16kW::new(self, 0)
    }
    #[doc = "Bits 3:5 - INRO 30KHz Trim."]
    #[inline(always)]
    pub fn trim30k(&mut self) -> Trim30kW<InroSpec> {
        Trim30kW::new(self, 3)
    }
    #[doc = "Bits 6:7 - INRO Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpclksel(&mut self) -> LpclkselW<InroSpec> {
        LpclkselW::new(self, 6)
    }
}
#[doc = "RTC Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InroSpec;
impl crate::RegisterSpec for InroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inro::R`](R) reader structure"]
impl crate::Readable for InroSpec {}
#[doc = "`write(|w| ..)` method takes [`inro::W`](W) writer structure"]
impl crate::Writable for InroSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INRO to value 0"]
impl crate::Resettable for InroSpec {
    const RESET_VALUE: u32 = 0;
}
