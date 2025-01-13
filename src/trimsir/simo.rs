#[doc = "Register `SIMO` reader"]
pub type R = crate::R<SimoSpec>;
#[doc = "SIMO Clock Divide.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: `0`"]
    Div1 = 0,
    #[doc = "1: `1`"]
    Div16 = 1,
    #[doc = "3: `11`"]
    Div32 = 3,
    #[doc = "5: `101`"]
    Div64 = 5,
    #[doc = "7: `111`"]
    Div128 = 7,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - SIMO Clock Divide."]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::Div1),
            1 => Some(Clkdiv::Div16),
            3 => Some(Clkdiv::Div32),
            5 => Some(Clkdiv::Div64),
            7 => Some(Clkdiv::Div128),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Clkdiv::Div1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Clkdiv::Div16
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Clkdiv::Div32
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Clkdiv::Div64
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Clkdiv::Div128
    }
}
impl R {
    #[doc = "Bits 0:2 - SIMO Clock Divide."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 7) as u8)
    }
}
#[doc = "SIMO Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`simo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SimoSpec;
impl crate::RegisterSpec for SimoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simo::R`](R) reader structure"]
impl crate::Readable for SimoSpec {}
#[doc = "`reset()` method sets SIMO to value 0"]
impl crate::Resettable for SimoSpec {
    const RESET_VALUE: u32 = 0;
}
